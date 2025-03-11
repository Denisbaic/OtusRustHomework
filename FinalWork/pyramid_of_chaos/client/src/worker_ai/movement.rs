use crate::movement_input::MovementInputComponent;
use avian3d::prelude::{ Collider, CollidingEntities };
use bevy::{
    ecs::entity::Entity,
    log::debug,
    math::Vec3,
    prelude::{ Component, Query, Res, Transform, With, Without },
    time::Time,
    transform::components::GlobalTransform,
};
use big_brain::{ prelude::ActionState, thinker::{ ActionSpan, Actor, HasThinker } };
use big_brain_derive::ActionBuilder;

// This is a component that will be attached to the actor entity when it is
// moving to a location. It's not an AI component, but a standard Bevy component
#[derive(Debug, Clone, Component, ActionBuilder)]
#[action_label = "MoveToNearestLabel"]
pub struct MoveToNearest<T: Component + std::fmt::Debug + Clone> {
    // We use a PhantomData to store the type of the component we're moving to.
    _marker: std::marker::PhantomData<T>,
    speed: f32,
    min_distance: f32,
}

impl<T: Component + std::fmt::Debug + Clone> MoveToNearest<T> {
    pub fn new(speed: f32, min_distance: f32) -> Self {
        Self {
            _marker: std::marker::PhantomData,
            speed,
            min_distance,
        }
    }
}

fn get_nearest<'a>(
    current: &Vec3,
    query: &'a Query<(Entity, &GlobalTransform, &Collider)>
) -> Option<Entity> {
    let compare_method = |
        a: &(_, &GlobalTransform, &Collider),
        b: &(_, &GlobalTransform, &Collider)
    | {
        let distance_to_a = a.2.distance_to_point(
            a.1.translation(),
            a.1.rotation(),
            *current,
            true
        );
        let distance_to_b = b.2.distance_to_point(
            b.1.translation(),
            b.1.rotation(),
            *current,
            true
        );
        distance_to_a.partial_cmp(&distance_to_b).unwrap_or(std::cmp::Ordering::Equal)
    };
    query
        .iter()
        .min_by(compare_method)
        .map(|(entity, _, _)| entity)
}

// This system manages the movement of entities. It moves the entity towards the
// nearest entity with the specified component and updates the entity's state
// based on the MoveToNearest component's parameters.
pub fn move_to_nearest_system<T: Component + std::fmt::Debug + Clone>(
    time: Res<Time>,
    mut query: Query<(Entity, &GlobalTransform, &Collider), With<T>>,
    // We filter on HasThinker since otherwise we'd be querying for every
    // entity in the world with a transform!
    mut thinkers: Query<
        (&Transform, &mut MovementInputComponent, &CollidingEntities),
        (With<HasThinker>, Without<T>)
    >,
    mut action_query: Query<(&Actor, &mut ActionState, &MoveToNearest<T>, &ActionSpan)>
) {
    for (actor, mut action_state, move_to, span) in &mut action_query {
        let _guard = span.span().enter();

        match *action_state {
            ActionState::Requested => {
                debug!("Let's go find a {:?}", std::any::type_name::<T>());

                *action_state = ActionState::Executing;
            }
            ActionState::Executing => {
                let (actor_transform, mut movement_input, collided_entities) = thinkers
                    .get_mut(actor.0)
                    .unwrap();
                // The goal is the nearest entity with the specified component.
                let mut nearest_lens = query.transmute_lens::<
                    (Entity, &GlobalTransform, &Collider)
                >();
                let nearest_query = nearest_lens.query();

                let Some(goal_entity) = get_nearest(
                    &actor_transform.translation,
                    &nearest_query
                ) else {
                    continue;
                };

                let Ok((goal_entity, goal_transform, goal_collider)) = query.get(goal_entity) else {
                    continue;
                };

                let dir = (goal_transform.translation() - actor_transform.translation).normalize();

                let Some((distance, _)) = goal_collider.cast_ray(
                    goal_transform.translation(),
                    goal_transform.rotation(),
                    actor_transform.translation,
                    dir,
                    500.0,
                    true
                ) else {
                    continue;
                };

                debug!("Distance: {}", distance);

                if collided_entities.contains(&goal_entity) || distance < move_to.min_distance {
                    debug!("We got there !");
                    *action_state = ActionState::Success;
                } else {
                    debug!("Stepping closer.");

                    let step_size = time.delta_secs() * move_to.speed;
                    let step = dir * step_size.min(distance);

                    movement_input.offset.x += step.x;
                    movement_input.offset.z += step.z;
                }
            }
            ActionState::Cancelled => {
                *action_state = ActionState::Failure;
            }
            _ => {}
        }
    }
}
