use bevy::ecs::component::Component;

pub mod get_bricks_from_quarry;
pub mod movement;

#[derive(Component)]
pub struct House;
