use bevy::{prelude::Component, time::Timer};

pub mod cargo_bundle;
pub mod cargo_view_processor_impl;
pub mod worker_bundle;

#[derive(Component)]
pub struct Faint {
    pub faint_timer: Timer,
}
