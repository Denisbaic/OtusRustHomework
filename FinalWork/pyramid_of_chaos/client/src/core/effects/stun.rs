use bevy::{
    math::Vec3,
    prelude::{Commands, Component, Entity, Query, Res, With},
    reflect::Reflect,
    time::{Time, Timer, TimerMode},
};

use crate::movement_input::MovementInputComponent;

#[derive(Component, Reflect)]
pub struct Stunned(Timer);

impl Stunned {
    pub fn new(duration: f32) -> Self {
        Self(Timer::from_seconds(duration, TimerMode::Once))
    }
}

pub fn apply_stun(mut query: Query<&mut MovementInputComponent, With<Stunned>>) {
    for mut movement_input in query.iter_mut() {
        movement_input.offset = Vec3::ZERO;
    }
}

pub fn remove_finished_stun(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Stunned)>,
) {
    for (entity, mut stun) in query.iter_mut() {
        if stun.0.tick(time.delta()).finished() {
            commands.entity(entity).remove::<Stunned>();
        }
    }
}
