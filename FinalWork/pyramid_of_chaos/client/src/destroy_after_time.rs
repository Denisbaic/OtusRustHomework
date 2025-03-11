use bevy::{
    ecs::{
        component::Component,
        entity::Entity,
        system::{Commands, Query, Res},
    },
    reflect::Reflect,
    time::{Time, Timer},
};

#[derive(Component, Reflect)]
pub struct DestroyTimer {
    pub timer: Timer,
}

pub fn destroy_after_time_system(
    mut destroy_after_time: Query<(Entity, &mut DestroyTimer)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut destroy_timer) in &mut destroy_after_time {
        destroy_timer.timer.tick(time.delta());
        if destroy_timer.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}
