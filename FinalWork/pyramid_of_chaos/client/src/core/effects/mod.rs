use bevy::prelude::SystemSet;

pub mod stun;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct EffectsSet;
