use bevy::ecs::{ entity::Entity, system::{ Commands, Query } };
use bevy::prelude::*;

use crate::min_max::MinMaxCurrent;

#[derive(Component, Reflect)]
pub struct Health {
    pub value: MinMaxCurrent<f32>,
}

pub fn despawn_entity_when_health_zero(heaths: Query<(Entity, &Health)>, mut command: Commands) {
    for (entity, health) in &heaths {
        if health.value.is_current_equal_to_min() {
            debug!("Despawning entity: {}", entity);
            command.entity(entity).despawn();
        }
    }
}
