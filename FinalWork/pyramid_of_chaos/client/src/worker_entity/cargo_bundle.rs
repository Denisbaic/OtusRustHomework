use bevy::{
    pbr::{MeshMaterial3d, StandardMaterial},
    prelude::{Bundle, Mesh3d, Transform},
};

#[derive(Bundle)]
pub struct CargoBundle {
    pub center: Transform,
    pub body: Mesh3d,
    pub material: MeshMaterial3d<StandardMaterial>,
}

pub trait CargoViewProcessor<CargoOwner, Cargo> {
    fn process(cargo_owner: &CargoOwner, cargo: &Cargo) -> CargoBundle;
}
