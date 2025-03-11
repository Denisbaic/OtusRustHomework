use avian3d::prelude::{ Collider, CollidingEntities };
use bevy::{
    asset::Assets,
    color::Color,
    pbr::{ MeshMaterial3d, StandardMaterial },
    prelude::{ Bundle, Capsule3d, Component, Mesh, Mesh3d, ResMut, Transform },
};

use crate::{
    fire::Ignitable,
    health::Health,
    min_max::MinMaxCurrent,
    movement_input::MovementInputComponent,
};

#[derive(Debug, Clone, Component, Default)]
#[require(Ignitable(|| Ignitable { light_coeff: 1.0, upper_bound: 20.0 }))]
pub struct NPC;

#[derive(Bundle)]
pub struct WorkerBundle {
    pub npc: NPC,
    pub body: Mesh3d,
    pub material: MeshMaterial3d<StandardMaterial>,
    pub collider: Collider,
    pub transform: Transform,
    pub movement_input: MovementInputComponent,
    pub health: Health,
    pub coliding_entities: CollidingEntities,
}

impl WorkerBundle {
    pub fn new(
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        capsule_info: Capsule3d,
        transform: Transform,
        color: Color
    ) -> WorkerBundle {
        WorkerBundle {
            npc: NPC,
            body: Mesh3d(meshes.add(Mesh::from(capsule_info))),
            collider: Collider::capsule(capsule_info.radius, capsule_info.half_length * 2.0),
            transform,
            material: MeshMaterial3d(materials.add(color)),
            movement_input: MovementInputComponent::default(),
            health: Health {
                value: MinMaxCurrent::new(0.0, 100.0, 100.0),
            },
            coliding_entities: CollidingEntities::default(),
        }
    }
}
