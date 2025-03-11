use bevy::{
    asset::Assets,
    color::Color,
    math::Vec3,
    pbr::{MeshMaterial3d, StandardMaterial},
    prelude::{Cuboid, Mesh, Mesh3d, ResMut, Transform},
};

use super::{
    cargo_bundle::{CargoBundle, CargoViewProcessor},
    worker_bundle::WorkerBundle,
};

pub struct CargoViewProcessorImpl;

pub struct BrickInfo {
    brick_mesh: Mesh3d,
    material: MeshMaterial3d<StandardMaterial>,
    y_size: f32,
}

impl BrickInfo {
    pub fn new(
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        color: Color,
        cuboid: Cuboid,
    ) -> BrickInfo {
        BrickInfo {
            brick_mesh: Mesh3d(meshes.add(Mesh::from(cuboid))),
            y_size: cuboid.half_size.y * 2.0,
            material: MeshMaterial3d(materials.add(color)),
        }
    }
}

impl CargoViewProcessor<WorkerBundle, BrickInfo> for CargoViewProcessorImpl {
    fn process(cargo_owner: &WorkerBundle, cargo: &BrickInfo) -> CargoBundle {
        let top = cargo_owner
            .collider
            .shape_scaled()
            .as_capsule()
            .unwrap()
            .height();

        let brick_result_y = top + cargo.y_size / 2.0;

        println!("Brick result point: {}", brick_result_y);
        CargoBundle {
            center: Transform::from_translation(Vec3::new(0.0, brick_result_y, 0.0)),
            body: cargo.brick_mesh.clone(),
            material: cargo.material.clone(),
        }
    }
}
