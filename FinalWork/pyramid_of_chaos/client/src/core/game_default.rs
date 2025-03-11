use avian3d::{
    math::Scalar,
    prelude::{ Collider, CollidingEntities, SpatialQuery, SpatialQueryFilter },
};
use bevy::{
    app::{ Plugin, Startup, Update },
    color::palettes::css::{ self, ANTIQUE_WHITE, BLACK, YELLOW },
    prelude::*,
};
use big_brain::{ prelude::{ FirstToScore, Steps }, thinker::Thinker, BigBrainPlugin, BigBrainSet };
use leafwing_input_manager::{
    plugin::InputManagerPlugin,
    prelude::ActionState,
    InputManagerBundle,
};
use rand::{ rng, seq::IteratorRandom, Rng };
use std::{ collections::HashMap, f32::consts::*, time::Duration };

use crate::{
    core::input,
    destroy_after_time::{ destroy_after_time_system, DestroyTimer },
    fire::{ Burning, Fireball, Ignitable },
    freeze::{ Freezing, Frozen, Storm },
    health::{ despawn_entity_when_health_zero, Health },
    min_max::{ self, MinMaxCurrent },
    movement_input::MovementInputComponent,
    worker_ai::House,
    worker_entity::worker_bundle::NPC,
};

use crate::{
    movement_input::{
        apply_constant_movement_system,
        apply_movement_input_system,
        MovementInputSet,
    },
    worker_ai::{
        get_bricks_from_quarry::{
            build_action_system,
            get_bricks_action_system,
            get_bricks_scorer_system,
            move_bricks_to_construction_site_scorer_system,
            Build,
            GetBricks,
        },
        movement::move_to_nearest_system,
    },
};
use crate::{
    worker_ai::{
        get_bricks_from_quarry::{
            GetBricksNeedScorer,
            MoveBricksToConstructionSitesNeedScorer,
            WorkerInventory,
        },
        movement::MoveToNearest,
    },
    worker_entity::worker_bundle::WorkerBundle,
};

use super::{ effects::{ self, stun::Stunned, EffectsSet }, input::CharacterAction };

#[derive(Debug, Clone, Component)]
struct Quarry;

#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct ConstuctionSite {
    pub bricks: min_max::MinMaxCurrent<u32>,
}

fn visualize_constuction_sites(
    sites: Query<(&ConstuctionSite, &Children)>,
    mut transforms: Query<&mut Transform>
) {
    for (ConstuctionSite { bricks }, children) in &sites {
        for child in children {
            let Ok(mut transform) = transforms.get_mut(*child) else {
                continue;
            };
            let percent = (bricks.get_current_copy() as f32) / (bricks.get_max_copy() as f32);

            let calculated_y_coord = percent.remap(0.0, 1.0, -13.0, 0.0);
            transform.translation = Vec3::new(0.0, calculated_y_coord, 0.0);
        }
    }
}

trait MyTrait {
    fn HelloWorld(&self);
}

impl MyTrait for u32 {
    fn HelloWorld(&self) {
        todo!()
    }
}

#[derive(PartialEq, Eq, Default, Hash, Debug, Clone, Copy)]
enum MagicType {
    #[default]
    Poking,
    Fireball,
    Freeze,
}

#[derive(Component, Default)]
struct MagicTypesTimeOut {
    pub magic_type_timers: HashMap<MagicType, Timer>,
}

fn tick_magic_types_time_out(mut query: Query<&mut MagicTypesTimeOut>, time: Res<Time>) {
    for mut magic_types_time_out in query.iter_mut() {
        for (_, timer) in magic_types_time_out.magic_type_timers.iter_mut() {
            timer.tick(time.delta());
        }
    }
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash, Reflect)]
enum GameState {
    #[default]
    Running,
    GameOver,
}

fn spawn_worker_in_random_house(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    houses: Query<&GlobalTransform, With<House>>
) {
    let mut rng = rand::rng();
    let Some(house) = houses.iter().choose(&mut rng) else {
        return;
    };

    let worker_unit = WorkerBundle::new(
        &mut meshes,
        &mut materials,
        Capsule3d {
            half_length: 2.0,
            radius: 0.5,
        },
        house.clone().compute_transform(),
        Color::Srgba(css::BLACK)
    );

    let move_and_get_brick = Steps::build()
        .label("MoveAndGetBrickFromQuarry")
        .step(MoveToNearest::<Quarry>::new(5.0, 0.1))
        .step(GetBricks::new(1, Duration::from_secs(1)));

    let move_and_build = Steps::build()
        .label("MoveAndGrabBrickToConstuctionSite")
        .step(MoveToNearest::<ConstuctionSite>::new(5.0, 0.1))
        .step(Build);

    commands.spawn((
        Name::new("Worker"),
        worker_unit,
        WorkerInventory::new(0, 1),
        Thinker::build()
            .label("My Thinker")
            // Selects the action with the highest score that is above the threshold.
            .picker(FirstToScore::new(0.6))
            .when(GetBricksNeedScorer, move_and_get_brick)
            .when(MoveBricksToConstructionSitesNeedScorer, move_and_build),
    ));
}

#[derive(Default, Resource)]
struct GameplayScore {
    pub score_value: f32,
}

fn tick_gameplay_score(
    mut score: ResMut<GameplayScore>,
    gameplay_time: Res<GameplayTimer>,
    time: Res<Time>,
    npcs: Query<(), With<NPC>>
) {
    let delta_score = f32::max(
        5.0,
        gameplay_time.timer.elapsed_secs() * (npcs.iter().count() as f32) * 0.5
    );
    score.score_value += time.delta_secs() * delta_score;
}

#[derive(Resource, Debug, Clone, Default)]
struct GameplayTimer {
    pub timer: Timer,
}

fn tick_gameplay_timer(mut timer: ResMut<GameplayTimer>, time: Res<Time>) {
    timer.timer.tick(time.delta());
}

#[derive(Component, Default)]
#[require(MagicTypesTimeOut)]
struct Player {
    choosed_magic_type: MagicType,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut gameplay_timer: ResMut<GameplayTimer>,
    mut score: ResMut<GameplayScore>
) {
    debug!("setup");

    gameplay_timer.timer.reset();
    score.score_value = 0.0;

    // light
    commands.spawn((
        Name::new("Sun"),
        DirectionalLight {
            color: Color::srgb(255.0 / 255.0, 204.0 / 255.0, 51.0 / 255.0),
            illuminance: 1.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(100.0, 100.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands
        .spawn((
            Name::new("Player"),
            Player::default(),
            Transform::default(),
            InputManagerBundle::with_map(input::CharacterAction::default_input_map()),
            InheritedVisibility::default(),
        ))
        .with_child((
            Camera3d::default(),
            Transform::from_xyz(0.0, 25.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
        ));

    commands.spawn((
        Name::new("Quarry"),
        Transform::from_xyz(35.0, 0.0, 30.0),
        Quarry,
        Mesh3d(
            meshes.add(
                Mesh::from(Cuboid {
                    half_size: Vec3::new(1.0, 0.3, 1.0),
                })
            )
        ),
        MeshMaterial3d(materials.add(Color::Srgba(css::GRAY))),
        Collider::cuboid(1.0 * 2.0, 0.3 * 2.0, 1.0 * 2.0),
    ));

    commands.spawn((
        Name::new("Quarry"),
        Transform::from_xyz(-35.0, 0.0, -30.0),
        Quarry,
        Mesh3d(
            meshes.add(
                Mesh::from(Cuboid {
                    half_size: Vec3::new(1.0, 0.3, 1.0),
                })
            )
        ),
        MeshMaterial3d(materials.add(Color::Srgba(css::GRAY))),
        Collider::cuboid(1.0 * 2.0, 0.3 * 2.0, 1.0 * 2.0),
    ));

    commands.spawn((
        Name::new("House"),
        Transform::from_xyz(50.0, 1.5, 50.0),
        House,
        Mesh3d(
            meshes.add(
                Mesh::from(Cuboid {
                    half_size: Vec3::new(2.0, 1.5, 1.0),
                })
            )
        ),
        MeshMaterial3d(materials.add(Color::Srgba(css::GREEN))),
    ));

    commands.spawn((
        Name::new("House"),
        Transform::from_xyz(-50.0, 1.5, 50.0),
        House,
        Mesh3d(
            meshes.add(
                Mesh::from(Cuboid {
                    half_size: Vec3::new(2.0, 1.5, 1.0),
                })
            )
        ),
        MeshMaterial3d(materials.add(Color::Srgba(css::GREEN))),
    ));

    commands.spawn((
        Name::new("House"),
        Transform::from_xyz(-50.0, 1.5, -50.0),
        House,
        Mesh3d(
            meshes.add(
                Mesh::from(Cuboid {
                    half_size: Vec3::new(2.0, 1.5, 1.0),
                })
            )
        ),
        MeshMaterial3d(materials.add(Color::Srgba(css::GREEN))),
    ));

    commands.spawn((
        Name::new("House"),
        Transform::from_xyz(50.0, 1.5, -50.0),
        House,
        Mesh3d(
            meshes.add(
                Mesh::from(Cuboid {
                    half_size: Vec3::new(2.0, 1.5, 1.0),
                })
            )
        ),
        MeshMaterial3d(materials.add(Color::Srgba(css::GREEN))),
    ));

    let piramide_mesh = Mesh::from(
        Tetrahedron::new(
            Vec3::new(0.0, 13.0, 0.0),
            Vec3::new(0.0, 0.0, 11.376),
            Vec3::new(-9.09615, 0.0, -8.9251),
            Vec3::new(9.0961, 0.0, -8.9251)
        )
    );

    commands
        .spawn((
            Name::new("ConstuctionSite"),
            Transform::from_xyz(0.0, 0.0, 0.0),
            ConstuctionSite {
                bricks: MinMaxCurrent::new(0, 100, 0),
            },
            Collider::triangle(
                Vec3::new(0.0, 0.0, 11.376),
                Vec3::new(-9.09615, 0.0, -8.9251),
                Vec3::new(9.0961, 0.0, -8.9251)
            ),
        ))
        .with_child((
            Mesh3d(meshes.add(piramide_mesh)),
            MeshMaterial3d(materials.add(Color::Srgba(css::BROWN))),
            Transform::from_xyz(0.0, 0.0, 0.0),
        ));

    commands.spawn((
        Name::new("Ground"),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Mesh3d(
            meshes.add(
                Mesh::from(Plane3d {
                    normal: Dir3::Y,
                    half_size: Vec2::new(100.0, 100.0),
                })
            )
        ),
        MeshMaterial3d(materials.add(Color::Srgba(css::YELLOW))),
    ));
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct UISet;

#[derive(Component)]
struct ScoreUIMarker;

#[derive(Component)]
struct MagicUIMarker {
    magic_type: MagicType,
}

#[derive(Component)]
struct ConstructionSiteProgressUIMarker;

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Контейнер для иконок способностей
    let icons = vec![
        ("Poking", asset_server.load("textures/poking.png"), MagicType::Poking),
        ("Fireball", asset_server.load("textures/fireball.png"), MagicType::Fireball),
        ("Freeze", asset_server.load("textures/freeze.png"), MagicType::Freeze)
    ];

    for (i, (name, image, magic_type)) in icons.iter().enumerate() {
        commands.spawn((
            Name::new(*name),
            ImageNode::new(image.clone()),
            Node {
                width: Val::Px(64.0),
                height: Val::Px(64.0),
                left: Val::Px(30.0),
                top: Val::Percent(((i + 1) as f32) * 20.0),
                position_type: PositionType::Relative,
                ..default()
            },
            BackgroundColor(ANTIQUE_WHITE.into()),
            Outline::new(Val::Px(8.0), Val::ZERO, BLACK.into()),
            Text::new("0"),
            TextFont {
                font_size: 30.0,
                ..default()
            },
            TextLayout::new_with_justify(JustifyText::Center),
            MagicUIMarker { magic_type: *magic_type },
        ));
    }

    commands.spawn((
        Node {
            right: Val::Px(30.0),
            top: Val::Px(30.0),
            position_type: PositionType::Absolute,
            ..default()
        },
        Name::new("Score"),
        Text::new("Score : 0"),
        TextFont {
            font_size: 30.0,
            ..default()
        },
        TextLayout::new_with_justify(JustifyText::Center),
        ScoreUIMarker,
    ));

    commands.spawn((
        Node {
            right: Val::Percent(50.0),
            bottom: Val::Percent(10.0),
            position_type: PositionType::Absolute,
            ..default()
        },
        Name::new("ConstructionSite Progress"),
        Text::new("Pyramid Progress : 0%"),
        TextFont {
            font_size: 30.0,
            ..default()
        },
        TextLayout::new_with_justify(JustifyText::Center),
        ConstructionSiteProgressUIMarker,
    ));
}

fn mark_choosed_magic(mut query: Query<(&mut Outline, &MagicUIMarker)>, player: Query<&Player>) {
    if let Ok(player) = player.get_single() {
        for (mut outline, marker) in &mut query {
            outline.color = if marker.magic_type == player.choosed_magic_type {
                YELLOW.into()
            } else {
                BLACK.into()
            };
        }
    }
}

fn update_timeout_text(
    mut query: Query<(&mut Text, &mut TextFont, &MagicUIMarker)>,
    magic_timeout: Query<&MagicTypesTimeOut>
) {
    if let Ok(timeouts) = magic_timeout.get_single() {
        for (mut text, mut text_font, marker) in &mut query {
            let result_timeout = timeouts.magic_type_timers
                .get(&marker.magic_type)
                .map_or(0.0, |timer| timer.remaining().as_secs_f32());
            *text = Text::new(format!("{:.1}", result_timeout));
            text_font.font_size = if result_timeout >= 0.001 { 30.0 } else { 0.0 };
        }
    }
}

fn update_score_text(mut query: Query<&mut Text, With<ScoreUIMarker>>, score: Res<GameplayScore>) {
    for mut text in &mut query {
        *text = Text::new(format!("Score: {}", score.score_value as u32));
    }
}

fn update_construction_site_text(
    mut query: Query<&mut Text, With<ConstructionSiteProgressUIMarker>>,
    consturction_site: Single<&ConstuctionSite>
) {
    let construction_progress =
        ((consturction_site.bricks.get_current_copy() as f32) /
            (consturction_site.bricks.get_max_copy() as f32)) *
        100.0;

    for mut text in &mut query {
        *text = Text::new(format!("Pyramid Progress : {:.1}%", construction_progress));
    }
}

fn animate_light_direction(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>
) {
    for mut transform in &mut query {
        transform.rotation = Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            (time.elapsed_secs() * PI) / 5.0,
            -FRAC_PI_4
        );
    }
}

fn on_random_timer(
    base_duration: Duration,
    min_duration: Duration
) -> impl (FnMut(Res<Time>, Res<GameplayTimer>) -> bool) + Clone {
    let mut timer = Timer::new(base_duration, TimerMode::Repeating);

    move |time: Res<Time>, game_time: Res<GameplayTimer>| {
        let mut rng = rng();
        timer.tick(time.delta());

        if timer.just_finished() {
            let factor = (game_time.timer.elapsed_secs() / 100.0).min(1.0); // Ограничиваем уменьшение
            let new_duration = (
                base_duration.as_secs_f32() -
                factor * (base_duration.as_secs_f32() - min_duration.as_secs_f32())
            ).max(0.5);
            let current_duration = Duration::from_secs_f32(
                rng.random_range(min_duration.as_secs_f32()..=new_duration)
            );
            timer.set_duration(current_duration);

            true
        } else {
            false
        }
    }
}

fn on_game_end(
    mut commands: Commands,
    score_ui: Query<Entity, With<ScoreUIMarker>>,
    magic_ui: Query<Entity, With<MagicUIMarker>>,
    consturction_site: Query<Entity, With<ConstructionSiteProgressUIMarker>>,
    score: Res<GameplayScore>
) {
    for entity in &score_ui {
        commands.entity(entity).despawn();
    }
    for entity in &magic_ui {
        commands.entity(entity).despawn();
    }
    for entity in &consturction_site {
        commands.entity(entity).despawn();
    }

    let game_over_text = format!("Game Over\nScore: {}", score.score_value as u32);
    commands.spawn((
        Node {
            right: Val::Percent(50.0),
            top: Val::Percent(50.0),
            position_type: PositionType::Absolute,
            ..default()
        },
        Name::new("Game Over screen"),
        Text::new(game_over_text),
        TextFont {
            font_size: 30.0,
            ..default()
        },
        TextLayout::new_with_justify(JustifyText::Center),
    ));
}

fn check_game_state(
    consturction_site: Single<&ConstuctionSite>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>
) {
    if consturction_site.bricks.is_current_equal_to_max() && *state.get() == GameState::Running {
        next_state.set(GameState::GameOver);
    }
}

pub struct GameCore;

impl Plugin for GameCore {
    fn build(&self, app: &mut bevy::prelude::App) {
        let random_timer = on_random_timer(
            Duration::from_secs_f32(2.0),
            Duration::from_secs_f32(0.1)
        );
        app.add_plugins(InputManagerPlugin::<CharacterAction>::default())
            .init_resource::<ActionState<CharacterAction>>()
            .insert_resource(CharacterAction::default_input_map())
            .init_state::<GameState>()
            .add_systems(Startup, (setup, setup_ui))
            .add_systems(Update, animate_light_direction)
            .add_plugins(BigBrainPlugin::new(PreUpdate))
            //.add_plugins(PhysicsDebugPlugin::default())
            .add_systems(PreUpdate, (
                (
                    get_bricks_action_system,
                    build_action_system,
                    move_to_nearest_system::<Quarry>,
                    move_to_nearest_system::<ConstuctionSite>,
                ).in_set(BigBrainSet::Actions),
                (get_bricks_scorer_system, move_bricks_to_construction_site_scorer_system).in_set(
                    BigBrainSet::Scorers
                ),
            ))
            .add_systems(Update, (
                (
                    apply_constant_movement_system,
                    apply_movement_input_system.after(apply_constant_movement_system),
                    destroy_after_time_system,
                    check_game_state,
                )
                    .in_set(MovementInputSet)
                    .run_if(in_state(GameState::Running)),
            ))
            .add_systems(
                Update,
                (
                    effects::stun::apply_stun.before(effects::stun::remove_finished_stun),
                    effects::stun::remove_finished_stun,
                    crate::fire::burning_colided,
                    crate::fire::decrease_health_while_burning,
                    crate::fire::process_burning,
                    crate::freeze::decrease_movement_input.before(apply_movement_input_system),
                    crate::freeze::process_freeze,
                    crate::freeze::freezing_colided,
                ).in_set(EffectsSet)
            )
            .add_systems(OnEnter(GameState::GameOver), on_game_end)
            .add_systems(
                Update,
                (
                    mark_choosed_magic,
                    update_score_text,
                    update_timeout_text,
                    update_construction_site_text,
                ).in_set(UISet)
            )
            .add_systems(Update, spawn_worker_in_random_house.run_if(random_timer))
            .add_systems(PostUpdate, despawn_entity_when_health_zero)
            .add_systems(FixedUpdate, process_input)
            .add_systems(PostUpdate, tick_magic_types_time_out)
            .add_systems(PostUpdate, visualize_constuction_sites)
            .add_systems(PostUpdate, tick_gameplay_score.run_if(in_state(GameState::Running)))
            .add_systems(PostUpdate, tick_gameplay_timer.before(tick_gameplay_score))
            .configure_sets(Update, EffectsSet.before(MovementInputSet))
            .add_observer(on_spawn_magic_triggered)
            .add_observer(on_choose_magic_triggered)
            .add_observer(on_spawn_pocking_triggered)
            .add_observer(on_spawn_fireball_triggered)
            .add_observer(on_spawn_freeze_triggered)
            .add_observer(on_move_camera_triggered)
            .add_observer(on_zoom_camera_triggered)
            .register_type::<Stunned>()
            .register_type::<Freezing>()
            .register_type::<MovementInputComponent>()
            .register_type::<DestroyTimer>()
            .register_type::<Ignitable>()
            .register_type::<Burning>()
            .register_type::<Frozen>()
            .register_type::<Freezing>()
            .register_type::<Health>()
            .register_type::<ConstuctionSite>()
            .insert_resource(GameplayTimer::default())
            .insert_resource(GameplayScore::default());
    }
}

#[derive(Event)]
struct SpawnMagicEvent;

#[derive(Event)]
struct ChooseMagicEvent {
    event_instigator: Entity,
    magic_number: u32,
}

#[derive(Event)]
struct SpawnPockingMagicEvent;

#[derive(Event)]
struct SpawnFireballMagicEvent;

#[derive(Event)]
struct SpawnFreezeMagicEvent;

fn get_cursor_position(windows: &Query<&Window>) -> Option<Vec2> {
    for window in windows {
        if let Some(cursor_position) = window.cursor_position() {
            return Some(cursor_position);
        }
    }
    None
}

fn on_spawn_pocking_triggered(
    event: Trigger<SpawnPockingMagicEvent>,
    query_for_raycast: SpatialQuery,
    mut player: Query<&mut MagicTypesTimeOut, With<Player>>,
    query: Query<(&Camera, &GlobalTransform)>,
    npc: Query<(), With<NPC>>,
    windows: Query<&Window>,
    mut commands: Commands
) {
    let _ = event;
    let Ok(mut magic_types_time_out) = player.get_single_mut() else {
        return;
    };
    if let Some(cursor_position) = get_cursor_position(&windows) {
        if let Ok((camera, camera_transform)) = query.get_single() {
            if let Ok(world_ray) = camera.viewport_to_world(camera_transform, cursor_position) {
                let filter = SpatialQueryFilter::default();
                if
                    let Some(ray_hit_data) = query_for_raycast.cast_ray_predicate(
                        camera_transform.translation(),
                        world_ray.direction,
                        Scalar::MAX,
                        true,
                        &filter,
                        &(|entity| npc.contains(entity))
                    )
                {
                    commands.entity(ray_hit_data.entity).insert((Stunned::new(1.0),));
                    magic_types_time_out.magic_type_timers.insert(
                        MagicType::Poking,
                        Timer::from_seconds(0.5, TimerMode::Once)
                    );
                }
            }
        }
    }
}

fn on_spawn_fireball_triggered(
    event: Trigger<SpawnFireballMagicEvent>,
    mut player: Query<&mut MagicTypesTimeOut, With<Player>>,
    query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    let _ = event;
    let Ok(mut magic_types_time_out) = player.get_single_mut() else {
        return;
    };

    if let Some(cursor_position) = get_cursor_position(&windows) {
        if let Ok((camera, camera_transform)) = query.get_single() {
            if let Ok(world_ray) = camera.viewport_to_world(camera_transform, cursor_position) {
                let fireball_color_material = StandardMaterial::from_color(css::RED);
                commands.spawn(
                    Fireball::new(
                        camera_transform.translation(),
                        world_ray.direction * 2.0,
                        meshes.add(Mesh::from(Sphere { radius: 2.0 })),
                        2.0,
                        MeshMaterial3d(materials.add(fireball_color_material))
                    )
                );

                magic_types_time_out.magic_type_timers.insert(
                    MagicType::Fireball,
                    Timer::from_seconds(2.0, TimerMode::Once)
                );
            }
        }
    }
}

fn on_spawn_freeze_triggered(
    event: Trigger<SpawnFreezeMagicEvent>,
    mut player: Query<&mut MagicTypesTimeOut, With<Player>>,
    query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    let _ = event;
    let Ok(mut magic_types_time_out) = player.get_single_mut() else {
        return;
    };

    if let Some(cursor_position) = get_cursor_position(&windows) {
        if let Ok((camera, camera_transform)) = query.get_single() {
            if let Ok(world_ray) = camera.viewport_to_world(camera_transform, cursor_position) {
                let storm_color_material = StandardMaterial::from_color(css::BLUE.with_alpha(0.5));
                let storm_material_handle = MeshMaterial3d(materials.add(storm_color_material));

                let intersection_len = world_ray.intersect_plane(
                    Vec3::ZERO,
                    InfinitePlane3d::new(Vec3::Y)
                );
                if let Some(intersection_len) = intersection_len {
                    let result_point = world_ray.origin + world_ray.direction * intersection_len;
                    commands.spawn(Storm {
                        name: Name::new("Storm"),
                        transform: Transform::from_translation(result_point),
                        collider: avian3d::collision::Collider::sphere(5.0),
                        mesh: Mesh3d(meshes.add(Mesh::from(Sphere { radius: 5.0 }))),
                        material: storm_material_handle,
                        destroy_timer: DestroyTimer {
                            timer: Timer::from_seconds(5.0, TimerMode::Once),
                        },
                        freezing: Freezing { freez_coeff: 0.5 },
                        colliding_entities: CollidingEntities::default(),
                    });
                    magic_types_time_out.magic_type_timers.insert(
                        MagicType::Freeze,
                        Timer::from_seconds(5.0, TimerMode::Once)
                    );
                }
            }
        }
    }
}

fn can_use_magic(magic_type_to_check: &MagicType, magics_time_out: &MagicTypesTimeOut) -> bool {
    magics_time_out.magic_type_timers
        .iter()
        .find(|(magic_type, _)| magic_type_to_check == *magic_type)
        .map_or(true, |(_, timer)| timer.finished())
}

fn on_spawn_magic_triggered(
    event: Trigger<SpawnMagicEvent>,
    query: Query<(&Player, &MagicTypesTimeOut)>,
    mut commands: Commands
) {
    let _ = event;
    debug!("on_spawn_magic_triggered");

    if query.is_empty() {
        return;
    }

    let Ok((player, magics_time_out)) = query.get_single() else {
        return;
    };

    match player.choosed_magic_type {
        MagicType::Poking if can_use_magic(&MagicType::Poking, &magics_time_out) =>
            commands.trigger(SpawnPockingMagicEvent),
        MagicType::Fireball if can_use_magic(&MagicType::Fireball, &magics_time_out) =>
            commands.trigger(SpawnFireballMagicEvent),
        MagicType::Freeze if can_use_magic(&MagicType::Freeze, &magics_time_out) =>
            commands.trigger(SpawnFreezeMagicEvent),
        _ => (),
    }
}

fn on_choose_magic_triggered(event: Trigger<ChooseMagicEvent>, mut query: Query<&mut Player>) {
    if query.is_empty() {
        return;
    }

    debug!("magic number {}", event.magic_number);

    let player = query.get_mut(event.event_instigator);

    if let Ok(mut player) = player {
        match event.magic_number {
            1 => {
                player.choosed_magic_type = MagicType::Poking;
            }
            2 => {
                player.choosed_magic_type = MagicType::Fireball;
            }
            3 => {
                player.choosed_magic_type = MagicType::Freeze;
            }
            n => error!("Invalid magic number {n}"),
        }
    }
}

#[derive(Event)]
struct CameraMoveEvent {
    event_instigator: Entity,
    move_value: f32,
}

fn on_move_camera_triggered(
    event: Trigger<CameraMoveEvent>,
    mut query: Query<(&Player, &mut Transform)>,
    time: Res<Time>
) {
    if query.is_empty() {
        return;
    }

    let player_and_transform = query.get_mut(event.event_instigator);

    if let Ok((_, mut transform)) = player_and_transform {
        let speed = 20.0;
        transform.rotate(
            Quat::from_rotation_y((time.delta_secs() * event.move_value * speed).to_radians())
        );

        debug!("rotate camera {:?}", (time.delta_secs() * event.move_value * speed).to_radians());
    }
}

#[derive(Event)]
struct CameraZoomEvent {
    zoom_value: f32,
}

fn on_zoom_camera_triggered(
    event: Trigger<CameraZoomEvent>,
    mut query: Query<(&Camera3d, &mut Transform)>,
    time: Res<Time>
) {
    if query.is_empty() {
        return;
    }

    let camera_and_trasform = query.get_single_mut();

    if let Ok((_, mut trasform)) = camera_and_trasform {
        let speed = 20.0;
        let forward = trasform.forward();
        trasform.translation += forward * event.zoom_value * speed * time.delta_secs();
        trasform.translation.y = f32::max(trasform.translation.y, 0.0);

        debug!("camera arm {}", trasform.translation.length());
    }
}

fn process_input(
    mut action_query: Query<(&ActionState<CharacterAction>, Entity)>,
    mut commands: Commands
) {
    for (action_state, entity) in action_query.iter_mut() {
        if action_state.just_pressed(&CharacterAction::SpawnMagic) {
            commands.trigger(SpawnMagicEvent);
        }

        if action_state.just_pressed(&CharacterAction::ChooseMagic1) {
            commands.trigger(ChooseMagicEvent {
                event_instigator: entity,
                magic_number: 1,
            });
        }

        if action_state.just_pressed(&CharacterAction::ChooseMagic2) {
            commands.trigger(ChooseMagicEvent {
                event_instigator: entity,
                magic_number: 2,
            });
        }

        if action_state.just_pressed(&CharacterAction::ChooseMagic3) {
            commands.trigger(ChooseMagicEvent {
                event_instigator: entity,
                magic_number: 3,
            });
        }

        let a_d_movement = action_state.clamped_value(&CharacterAction::Move);
        if a_d_movement.abs() > 0.001 {
            commands.trigger(CameraMoveEvent {
                event_instigator: entity,
                move_value: a_d_movement,
            });
        }

        let zooming = action_state.clamped_value(&CharacterAction::Zooming);
        if zooming.abs() > 0.001 {
            commands.trigger(CameraZoomEvent {
                zoom_value: zooming,
            });
        }
    }
}
