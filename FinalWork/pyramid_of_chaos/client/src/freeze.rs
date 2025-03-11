use crate::{ destroy_after_time::DestroyTimer, movement_input::MovementInputComponent };
use avian3d::prelude::{ Collider, CollidingEntities };
use bevy::prelude::*;
use bevy::{
    core::Name,
    ecs::{ entity::Entity, system::{ Commands, Query, Res } },
    pbr::{ MeshMaterial3d, StandardMaterial },
    render::mesh::Mesh3d,
    time::Time,
    transform::components::Transform,
};

#[derive(Component, Reflect)]
pub struct Frozen {
    pub frozen_coeff: f32,
}

#[derive(Component, Reflect)]
pub struct Freezing {
    pub freez_coeff: f32,
}

#[derive(Bundle)]
pub struct Storm {
    pub name: Name,
    pub transform: Transform,
    pub collider: Collider,
    pub freezing: Freezing,
    pub mesh: Mesh3d,
    pub material: MeshMaterial3d<StandardMaterial>,
    pub destroy_timer: DestroyTimer,
    pub colliding_entities: CollidingEntities,
}

pub fn freezing_colided(
    freezings: Query<(&CollidingEntities, &Freezing)>,
    mut already_frozens: Query<&mut Frozen>,
    mut commands: Commands
) {
    for (collided, freezing) in &freezings {
        for colided_entity in collided.iter() {
            let frozen_opt = already_frozens.get_mut(*colided_entity).ok();
            if let Some(mut frozen) = frozen_opt {
                frozen.frozen_coeff += freezing.freez_coeff;
            } else {
                let Some(mut entity) = commands.get_entity(*colided_entity) else {
                    continue;
                };
                entity.insert(Frozen {
                    frozen_coeff: freezing.freez_coeff,
                });
            }
        }
    }
}

pub fn decrease_movement_input(mut frozens: Query<(&Frozen, &mut MovementInputComponent)>) {
    for (frozen, mut movement_input) in frozens.iter_mut() {
        movement_input.offset /= frozen.frozen_coeff * 10.0;
    }
}

pub fn process_freeze(
    mut commands: Commands,
    mut frozens: Query<(Entity, &mut Frozen)>,
    time: Res<Time>
) {
    for (entity, mut frozen) in frozens.iter_mut() {
        frozen.frozen_coeff -= time.delta_secs() * 10.0;

        frozen.frozen_coeff = f32::max(frozen.frozen_coeff, 1.0);

        if frozen.frozen_coeff <= 1.0 {
            commands.entity(entity).remove::<Frozen>();
        }
    }
}
