use avian3d::prelude::{Collider, CollidingEntities};
use bevy::prelude::*;
use bevy::{
    asset::Handle,
    core::Name,
    ecs::{
        entity::Entity,
        query::{With, Without},
        system::{Commands, Query, Res},
    },
    math::Vec3,
    pbr::{MeshMaterial3d, StandardMaterial},
    render::mesh::{Mesh, Mesh3d},
    time::{Time, Timer, TimerMode},
    transform::components::Transform,
};

use crate::{
    destroy_after_time::DestroyTimer,
    health::Health,
    movement_input::{ConstantApplyMovementComponent, MovementInputComponent},
};

#[derive(Component, Reflect)]
pub struct Ignitable {
    pub light_coeff: f32,
    pub upper_bound: f32,
}

#[derive(Component, Reflect)]
pub struct Burning {
    pub fire_coeff: f32,
}

#[derive(Bundle)]
pub struct Fireball {
    pub name: Name,
    pub transform: Transform,
    pub mesh: Mesh3d,
    pub material: MeshMaterial3d<StandardMaterial>,
    pub burning: Burning,
    pub movement_input: ConstantApplyMovementComponent,
    pub movement_input_component: MovementInputComponent,
    pub destroy_timer: DestroyTimer,
    pub colliding_entities: CollidingEntities,
    pub collider: Collider,
}

impl Fireball {
    pub fn new(
        position: Vec3,
        velocity: Vec3,
        mesh_handle: Handle<Mesh>,
        sphere_size: f32,
        fireball_material_handle: MeshMaterial3d<StandardMaterial>,
    ) -> Self {
        Fireball {
            name: Name::new("Fireball"),
            transform: Transform::from_translation(position).looking_to(velocity, Vec3::Y),
            burning: Burning {
                fire_coeff: f32::INFINITY,
            },
            movement_input: ConstantApplyMovementComponent { offset: velocity },
            mesh: Mesh3d::from(mesh_handle),
            material: fireball_material_handle,
            movement_input_component: MovementInputComponent::default(),
            destroy_timer: DestroyTimer {
                timer: Timer::from_seconds(10.0, TimerMode::Once),
            },
            colliding_entities: CollidingEntities::default(),
            collider: Collider::sphere(sphere_size),
        }
    }
}

pub fn burning_colided(
    ignitiable: Query<Entity, (With<Ignitable>, Without<Burning>)>,
    burnings: Query<(&CollidingEntities, &Burning)>,
    mut commands: Commands,
) {
    for ignited in &ignitiable {
        for (collided, burning) in &burnings {
            if collided.contains(&ignited) {
                commands.entity(ignited).insert(Burning {
                    fire_coeff: burning.fire_coeff / 2.0,
                });
            }
        }
    }
}

pub fn process_burning(
    mut commands: Commands,
    mut burnings: Query<(Entity, &mut Burning)>,
    ignitiables: Query<&Ignitable>,
    time: Res<Time>,
) {
    for (entity, mut burning) in burnings.iter_mut() {
        burning.fire_coeff -= time.delta_secs();
        if let Ok(ignitable) = ignitiables.get(entity) {
            burning.fire_coeff += ignitable.light_coeff * time.delta_secs();
            burning.fire_coeff = f32::min(burning.fire_coeff, ignitable.upper_bound);
        }
        burning.fire_coeff = f32::max(burning.fire_coeff, 0.0);
        if burning.fire_coeff <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn decrease_health_while_burning(
    mut burnings: Query<(&mut Health, &Burning)>,
    time: Res<Time>,
) {
    let health_decrease_in_second = 5.0;
    for (mut health, burning) in burnings.iter_mut() {
        let current_health = health.value.get_current_copy();
        health.value.set_current(
            current_health - time.delta_secs() * health_decrease_in_second * burning.fire_coeff,
        );
    }
}
