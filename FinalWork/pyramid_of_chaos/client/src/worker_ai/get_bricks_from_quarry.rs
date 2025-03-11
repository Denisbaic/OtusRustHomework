use std::time::Duration;

use avian3d::prelude::Collider;
use bevy::{
    asset::Assets,
    pbr::{ MeshMaterial3d, StandardMaterial },
    prelude::{ Component, Query, Res, ResMut },
    reflect::Reflect,
    time::{ Time, Timer },
};
use bevy::{ color::palettes::css, prelude::* };
use big_brain::{ prelude::ActionState, scorers::Score, thinker::{ ActionSpan, Actor } };
use big_brain_derive::{ ActionBuilder, ScorerBuilder };

use crate::core::game_default::ConstuctionSite;
use crate::min_max::MinMaxCurrent;

#[derive(Component, Debug, Clone)]
pub struct Quarry {
    pub bricks_in_storage: MinMaxCurrent<u8>,
    pub spawn_bricks_timer: Timer,
}

#[derive(Component, Reflect)]
pub struct WorkerInventory {
    pub bricks_in_hands: MinMaxCurrent<u8>,
}

impl WorkerInventory {
    pub fn new(current: u8, max: u8) -> Self {
        Self {
            bricks_in_hands: MinMaxCurrent::new(0, max, current),
        }
    }
}

#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct GetBricks {
    bricks_to_get_on_timer_end: u8,
    get_brick_timer: Timer,
}

impl GetBricks {
    pub fn new(bricks_to_get_on_timer_end: u8, repeat_duration: Duration) -> Self {
        Self {
            bricks_to_get_on_timer_end,
            get_brick_timer: Timer::new(repeat_duration, TimerMode::Repeating),
        }
    }
}

pub fn get_bricks_action_system(
    time: Res<Time>,
    mut inventories: Query<(&mut WorkerInventory, &MeshMaterial3d<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(&Actor, &mut ActionState, &mut GetBricks, &ActionSpan)>
) {
    for (Actor(actor), mut state, mut get_bricks, span) in &mut query {
        let _guard = span.span().enter();

        if let Ok((mut inventory, material)) = inventories.get_mut(*actor) {
            match *state {
                ActionState::Requested => {
                    debug!("Time to grab some bricks!");
                    get_bricks.get_brick_timer.reset();

                    *state = ActionState::Executing;
                }
                ActionState::Executing => {
                    trace!("Grabbing bricks...");
                    materials.get_mut(material).unwrap().base_color = Color::Srgba(css::BROWN);

                    if get_bricks.get_brick_timer.tick(time.delta()).finished() {
                        let next_current_bricks_count =
                            inventory.bricks_in_hands.get_current_copy() +
                            get_bricks.bricks_to_get_on_timer_end;
                        inventory.bricks_in_hands.set_current(next_current_bricks_count);
                    }

                    if inventory.bricks_in_hands.is_current_equal_to_max() {
                        debug!("Inventory full");
                        materials.get_mut(material).unwrap().base_color = Color::Srgba(css::CORAL);

                        *state = ActionState::Success;
                    }
                }
                // All Actions should make sure to handle cancellations!
                ActionState::Cancelled => {
                    debug!("Getting bricks was interrupted");

                    materials.get_mut(material).unwrap().base_color = Color::Srgba(css::CORAL);

                    get_bricks.get_brick_timer.reset();

                    *state = ActionState::Failure;
                }
                _ => {}
            }
        }
    }
}

#[derive(Clone, Component, Debug, ScorerBuilder)]
pub struct GetBricksNeedScorer;

pub fn get_bricks_scorer_system(
    actors: Query<&WorkerInventory>,
    mut query: Query<(&Actor, &mut Score), With<GetBricksNeedScorer>>
) {
    for (Actor(actor), mut score) in &mut query {
        if let Ok(inventory) = actors.get(*actor) {
            if inventory.bricks_in_hands.is_current_equal_to_max() {
                score.set(0.0);
            } else {
                score.set(0.6);
            }
        }
    }
}

fn get_nearest<'a>(
    current: &Vec3,
    query: &'a Query<(Entity, &mut ConstuctionSite, &GlobalTransform, &Collider)>
) -> Option<Entity> {
    let compare_method = |
        a: &(_, _, &GlobalTransform, &Collider),
        b: &(_, _, &GlobalTransform, &Collider)
    | {
        let distance_to_a = a.3.distance_to_point(
            a.2.translation(),
            a.2.rotation(),
            *current,
            true
        );
        let distance_to_b = b.3.distance_to_point(
            b.2.translation(),
            b.2.rotation(),
            *current,
            true
        );
        distance_to_a.partial_cmp(&distance_to_b).unwrap_or(std::cmp::Ordering::Equal)
    };
    query
        .iter()
        .min_by(compare_method)
        .map(|(entity, _, _, _)| entity)
}

#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct Build;

pub fn build_action_system(
    mut construction_sites: Query<(Entity, &mut ConstuctionSite, &GlobalTransform, &Collider)>,
    mut workers: Query<(&mut WorkerInventory, &Transform, &MeshMaterial3d<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(&Actor, &mut ActionState, &mut Build, &ActionSpan)>
) {
    for (Actor(actor), mut state, _get_bricks, span) in &mut query {
        let _guard = span.span().enter();

        if let Ok(mut worker) = workers.get_mut(*actor) {
            match *state {
                ActionState::Requested => {
                    debug!("time to move bricks to construction site!");
                    *state = ActionState::Executing;
                }
                ActionState::Executing => {
                    trace!("Moving bricks to construction site...");
                    let Some(nearest_construction_site) = get_nearest(
                        &worker.1.translation,
                        &construction_sites
                    ) else {
                        error!("No construction site found");
                        continue;
                    };
                    let (_, mut construction_site, _, _) = construction_sites
                        .get_mut(nearest_construction_site)
                        .unwrap();

                    let ConstuctionSite { bricks: ref mut bricks_in_site } =
                        construction_site.as_mut();

                    if bricks_in_site.is_current_equal_to_max() {
                        debug!("Construction site full");
                        *state = ActionState::Cancelled;
                        continue;
                    }

                    if worker.0.bricks_in_hands.is_current_equal_to_max() {
                        bricks_in_site.set_current(
                            bricks_in_site.get_current_copy() +
                                (worker.0.bricks_in_hands.get_current_copy() as u32)
                        );
                        debug!("Bricks moved");

                        //let bricks_moved = worker_inventory.bricks_in_hands.get_current_copy();
                        worker.0.bricks_in_hands.set_current(0);
                        materials.get_mut(worker.2).unwrap().base_color = Color::Srgba(css::BLACK);

                        *state = ActionState::Success;
                    } else {
                        *state = ActionState::Cancelled;
                    }
                }
                // All Actions should make sure to handle cancellations!
                ActionState::Cancelled => {
                    debug!("Moving bricks was interrupted");

                    *state = ActionState::Failure;
                }
                _ => {}
            }
        }
    }
}

#[derive(Clone, Component, Debug, ScorerBuilder)]
pub struct MoveBricksToConstructionSitesNeedScorer;

pub fn move_bricks_to_construction_site_scorer_system(
    actors: Query<&WorkerInventory>,
    mut query: Query<(&Actor, &mut Score), With<MoveBricksToConstructionSitesNeedScorer>>
) {
    for (Actor(actor), mut score) in &mut query {
        if let Ok(inventory) = actors.get(*actor) {
            if inventory.bricks_in_hands.is_current_equal_to_min() {
                score.set(0.0);
            }
            if inventory.bricks_in_hands.is_current_equal_to_max() {
                score.set(0.6);
            }
        }
    }
}
