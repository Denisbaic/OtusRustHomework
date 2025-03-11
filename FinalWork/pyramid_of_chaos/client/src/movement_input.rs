use bevy::{
    math::Vec3,
    prelude::{Component, Query, SystemSet, Transform},
    reflect::Reflect,
};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MovementInputSet;

#[derive(Component, Default, Reflect)]
pub struct MovementInputComponent {
    pub offset: Vec3,
}

#[derive(Component, Reflect)]
pub struct ConstantApplyMovementComponent {
    pub offset: Vec3,
}

pub fn apply_constant_movement_system(
    mut query: Query<(&ConstantApplyMovementComponent, &mut MovementInputComponent)>,
) {
    for (constant_movement_info, mut movement_input) in &mut query {
        movement_input.offset = constant_movement_info.offset;
    }
}

pub fn apply_movement_input_system(
    mut query: Query<(&mut Transform, &mut MovementInputComponent)>,
) {
    for (mut current_transfort, mut movement_input) in &mut query {
        current_transfort.translation += movement_input.offset;
        movement_input.offset = Vec3::ZERO;
    }
}
