use std::ops::Sub;
use std::time::Duration;

use crate::loading::TextureAssets;
use crate::GameState;
use crate::menu::{ButtonColors, ChangeState};
use crate::score::{Score, ComboMeter, ComboModifier};
use crate::trash_text::{TrashText, TrashTextBundle, highlight_characters, remove_highlight};
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::transform::TransformSystem;
use bevy::utils::HashMap;
use bevy_progressbar::ProgressBar;
use bevy_rapier2d::prelude::*;
use rand::Rng;

const BORDER_TILE_SIZE: f32 = 48.0;
const BORDER_TILE_SCALE: Vec2 = Vec2::new(BORDER_TILE_SIZE, BORDER_TILE_SIZE);

const TRASH_STARTING_VELOCITY: Vec2 = Vec2::new(0.0, -100.0);
const TRASH_MAXIMUM_VERTICAL_VELOCITY_LENGTH: f32 = 40.0;
const TRASH_MAXIMUM_HORIZONTAL_VELOCITY_LENGTH: f32 = 600.0;
const TRASH_SPAWN_DISTANCE_BETWEEN_SPAWNS: f32 = 30.0;

pub struct TrashPlugin;


#[derive(Clone, Debug)]
pub enum RunState {
    Running,
    Ended,
}


#[derive(Clone, Debug)]
pub enum TrashType {
    Bottle,
    Pizza,
    BigBox,
    GlassBottle,
    News,
    Shampoo,
    SmallCan,
    Soda,
    Spray,
}

impl Default for TrashType {
    fn default() -> Self {
        Self::Bottle
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PowerUp {
    None,
    Explosion,
    Link
}

impl Default for PowerUp {
    fn default() -> Self {
        Self::None
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum PowerUpEvent {
    None,
    EntityDestroyed,
    DestroyLinked,
}


#[derive(Component, Debug, Clone)]
pub struct Trash {
    pub trash_type: TrashType,
    pub power_up: PowerUp,
    pub size: Vec2,
    pub activated: bool,
}

impl Default for Trash {
    fn default() -> Self {
        Self::bottle()
    }
}


#[derive(Component, Debug, Clone, Default)]
pub struct BufferText;


#[derive(Component, Default)]
pub struct TrashActionActive;

#[derive(Component, Default)]
pub struct TrashActionDuplicate;

#[derive(Component, Default)]
pub struct TrashMarked;

#[derive(Component, Default)]
pub struct TrashExplosion;

#[derive(Component, Default)]
pub struct Wall;

#[derive(Component, Default)]
pub struct Floor;

#[derive(Component, Default)]
pub struct GameOverLine;

#[derive(Component, Default)]
pub struct GameOver;

// #[derive(Component, Debug, Clone)]
// pub struct TrashLabel;

#[derive(Resource)]
struct TrashSpawnTimer(pub Timer);

#[derive(Resource)]
struct DifficultyTimer(pub Timer);

#[derive(Resource)]
struct BufferTextDeleteTimer(Timer);

#[derive(Resource)]
pub struct TypingBuffer(String);

#[derive(Resource)]
pub struct AvailableWords(HashMap<String, Vec<String>>);


#[derive(Bundle)]
pub struct TrashBundle {
    sprite: SpriteBundle,
    // transform: Transform,
    rigidbody: RigidBody,
    velocity: Velocity,
    collider: Collider,
    collider_mass_properties: ColliderMassProperties,
    restitution: Restitution,
    active_events: ActiveEvents,
    trash: Trash,
}

impl TrashBundle {
    pub fn new(sprite: Handle<Image>, trash: Trash) -> Self {
        Self {
            sprite: SpriteBundle {
                texture: sprite,
                ..default()
            },
            // transform: Transform::from_translation(Vec3::new(pos.x, pos.y, 0.0)),
            rigidbody: RigidBody::Dynamic,
            velocity: Velocity::linear(TRASH_STARTING_VELOCITY),
            collider: Collider::cuboid(trash.size.x, trash.size.y),
            collider_mass_properties: ColliderMassProperties::Mass(1.0),
            restitution: Restitution::coefficient(0.2),
            active_events: ActiveEvents::COLLISION_EVENTS,
            trash: trash.clone(),
        }
    }

    pub fn create_text(text: String, anchor: Anchor, color: Color, style: TextStyle) -> TrashTextBundle {
        // self.sprite = self.with with_children(|parent| {
        //     parent.spawn(
                TrashTextBundle::new(
                    text,
                    anchor,
                    color,
                    style,
                )
            // )
        // });
        // self
    }
}

        // .spawn(SpriteBundle {
        //     texture: sprite,
        //     // transform: transform.clone(), // Transform::from_scale(Vec3::new(1.5, 1.5, 1.5)), //transform.clone(),
        //     ..default()
        // })
        // // .insert(Velocity::angular(3.0))
        // .insert(Transform::from_translation(Vec3::new(pos.x, pos.y, 0.0)))
        // .insert(RigidBody::Dynamic)
        // .insert(Collider::cuboid(trash.size.x, trash.size.y))
        // .insert(ColliderMassProperties::Mass(10.0))
        // .insert(Restitution::coefficient(0.7))
        // .insert(ActiveEvents::COLLISION_EVENTS)
        // .insert(TrashActive)
        // .insert(trash.clone())
        // .with_children(|parent| {
        //     parent.spawn(
        //         TrashTextBundle::new(
        //             word,
        //             Anchor::Custom(Vec2::new(0.0, -2.0)),
        //             Color::GREEN,
        //             TextStyle {
        //                 color: Color::WHITE,
        //                 font_size: 20.0,
        //                 ..default()
        //             }
        //         )
        //     );
        // });

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for TrashPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TrashSpawnTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .insert_resource(DifficultyTimer(Timer::from_seconds(15.0, TimerMode::Repeating)))
        // .insert_resource(BufferTextDeleteTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .insert_resource(TypingBuffer("".to_string()))
        .insert_resource(AvailableWords(get_available_words_from_file()))
        .add_systems(OnEnter(GameState::Playing), setup)
        .add_systems(Update, (
                spawn_trash.run_if(in_state(GameState::Playing)),
                trash_power_ups_effects.after(spawn_trash),
                update_difficuly.after(setup).run_if(in_state(GameState::Playing)),
                // destroy_trash_text.after(handle_trash_collision),
                typing.after(setup).run_if(in_state(GameState::Playing)),
                handle_trash_collision.after(typing).run_if(in_state(GameState::Playing)),
                // is_game_over.before(handle_trash_collision),
                clamp_duplicated_trash.after(handle_trash_collision),
                activate_matching_trash.after(typing).before(handle_trash_collision),
                remove_explosions.after(handle_trash_collision),
                update_buffer_text.after(typing),
                highlight_character.after(typing),
                click_restart_button.run_if(in_state(GameState::GameOver)),
            )
        )
        .add_systems(PostStartup, fix_trash_label_rotation.before(TransformSystem::TransformPropagate).run_if(in_state(GameState::Playing)))
        .add_systems(PostUpdate, fix_trash_label_rotation.before(TransformSystem::TransformPropagate).run_if(in_state(GameState::Playing)))
        .add_systems(OnEnter(GameState::GameOver), spawn_game_over_menu)
        .add_systems(OnExit(GameState::Playing), delete_all_play_entities)
        .add_systems(OnExit(GameState::GameOver), delete_all_gameover_entities);
    }
}

impl Trash {
    pub fn get_by_type(trash_type: TrashType) -> Self {
        match trash_type {
            TrashType::Bottle => Self::bottle(),
            TrashType::Pizza => Self::pizza(),
            TrashType::News => Self::news(),
            TrashType::Shampoo => Self::shampoo(),
            TrashType::SmallCan => Self::small_can(),
            TrashType::Soda => Self::soda(),
            TrashType::Spray => Self::spray(),
            TrashType::BigBox => Self::big_box(),
            TrashType::GlassBottle => Self::glass_bottle(),
        }
    }

    pub fn bottle() -> Self {
        Self {
            trash_type: TrashType::Bottle,
            size: Vec2::new(15.0, 16.0),
            power_up: PowerUp::None,
            activated: false,
        }
    }

    pub fn pizza() -> Self {
        Self {
            trash_type: TrashType::Pizza,
            size: Vec2::new(32.0, 16.0), // 60x30
            power_up: PowerUp::None,
            activated: false,
        }
    }

    pub fn big_box() -> Self {
        Self {
            trash_type: TrashType::BigBox,
            size: Vec2::new(25.0, 24.0), // 51x48
            power_up: PowerUp::None,
            activated: false,
        }
    }

    pub fn glass_bottle() -> Self {
        Self {
            trash_type: TrashType::GlassBottle,
            size: Vec2::new(8.0, 25.0), // 17x50
            power_up: PowerUp::None,
            activated: false,
        }
    }

    pub fn news() -> Self {
        Self {
            trash_type: TrashType::News,
            size: Vec2::new(26.0, 16.0), // 53x28
            power_up: PowerUp::None,
            activated: false,
        }
    }

    pub fn shampoo() -> Self {
        Self {
            trash_type: TrashType::Shampoo,
            size: Vec2::new(17.0, 22.0), // 36x44
            power_up: PowerUp::None,
            activated: false,
        }
    }

    pub fn small_can() -> Self {
        Self {
            trash_type: TrashType::SmallCan,
            size: Vec2::new(11.0, 15.0), // 23x30
            power_up: PowerUp::None,
            activated: false,
        }
    }

    pub fn soda() -> Self {
        Self {
            trash_type: TrashType::Soda,
            size: Vec2::new(9.0, 17.0), // 18x34
            power_up: PowerUp::None,
            activated: false,
        }
    }

    pub fn spray() -> Self {
        Self {
            trash_type: TrashType::Spray,
            size: Vec2::new(8.0, 20.0), // 17x40
            power_up: PowerUp::None,
            activated: false,
        }
    }
}


fn get_random_coordinate(border: f32, previous: f32) -> f32 {
    let mut random = rand::thread_rng();
    // let max_x: f32 = window.width() / 2.0;
    // let y_pos = (window.height() / 2.0) * 2.0;

    let mut random_x: f32 = random.gen_range(-border + BORDER_TILE_SIZE * 2.0 .. border - BORDER_TILE_SIZE * 2.0);

    while (random_x - previous).abs() < TRASH_SPAWN_DISTANCE_BETWEEN_SPAWNS {
        random_x = random.gen_range(-border + BORDER_TILE_SIZE * 2.0 .. border - BORDER_TILE_SIZE * 2.0);
    }

    return random_x;
}

fn spawn_trash(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    // keyboard_input: Res<Input<KeyCode>>,
    window: Query<&Window>,
    time: Res<Time>,
    available_words: Res<AvailableWords>,
    mut spawn_timer: ResMut<TrashSpawnTimer>,
    mut previous_spawn_position: Local<f32>,
) {

    static SPAWN_CHANCES: [TrashType; 9] = [
        TrashType::Bottle,
        TrashType::Pizza,
        TrashType::BigBox,
        TrashType::GlassBottle,
        TrashType::News,
        TrashType::Shampoo,
        TrashType::SmallCan,
        TrashType::Soda,
        TrashType::Spray,
    ];
    static POWER_UP_CHANCES: [PowerUp; 10] = [
        PowerUp::None,
        PowerUp::None,
        PowerUp::None,
        PowerUp::None,
        PowerUp::None,
        PowerUp::None,
        PowerUp::None,
        PowerUp::None,
        PowerUp::None,
        PowerUp::Explosion,
    ];

    if spawn_timer.0.tick(time.delta()).just_finished() {
        let window = window.single();

        let mut random = rand::thread_rng();
        let max_x: f32 = window.width() / 2.0;
        let y_pos = (window.height() / 2.0) * 2.0;

        // let random_x: f32 = random.gen_range(-max_x + BORDER_TILE_SIZE * 2.0 .. max_x - BORDER_TILE_SIZE * 2.0);
        let random_x = get_random_coordinate(max_x, *previous_spawn_position);
        *previous_spawn_position = random_x;
        // println!("Random x: {}", random_x);
        let trash_type: TrashType = SPAWN_CHANCES[random.gen_range(0..SPAWN_CHANCES.len())].clone();
        let power_up: PowerUp = POWER_UP_CHANCES[random.gen_range(0..POWER_UP_CHANCES.len())].clone();

        // TODO: make sure the same word doesn't appear twice in a row
        // A solution might be to have search to search for a word as long as it's not in a list of
        // already used words which we can get from a query
        // Also we need to be able to to limit the amount of letters in a word
        let mut trash = Trash::get_by_type(trash_type);
        if power_up != PowerUp::None {
            trash.power_up = power_up;
            // trash.size = Vec2::new(32.0, 32.0);
        }

        let trash_bundle = TrashBundle::new(get_trash_sprite(&trash.trash_type, &textures), trash);
        let trash_text = TrashBundle::create_text(
            get_random_word(&available_words),
            Anchor::Custom(Vec2::new(0.0, -2.0)),
            Color::GREEN,
            TextStyle {
                color: Color::WHITE,
                font_size: 30.0,
                ..default()
            }
        );
        commands.spawn(trash_bundle)
            .insert(Transform::from_translation(Vec3::new(random_x as f32, y_pos, 0.0)))
            .insert(TrashActionActive)
            .insert(TrashActionDuplicate)
            .with_children(|parent| {
                parent.spawn(trash_text);
                    // .insert(
                    //     Sprite {
                    //         color: Color::GRAY,
                    //         // custom_size: Some(Vec2::new(10.0, 10.0)),
                    //         ..default()
                    //     }
                    // );
            });

    }
}


fn get_trash_sprite(trash_type: &TrashType, textures: &Res<TextureAssets>) -> Handle<Image> {
    match trash_type {
        TrashType::Bottle => textures.bottle.clone(),
        TrashType::Pizza => textures.pizza.clone(),
        TrashType::BigBox => textures.big_box.clone(),
        TrashType::GlassBottle => textures.glass_bottle.clone(),
        TrashType::News => textures.news.clone(),
        TrashType::Shampoo => textures.shampoo.clone(),
        TrashType::SmallCan => textures.small_can.clone(),
        TrashType::Soda => textures.soda.clone(),
        TrashType::Spray => textures.spray.clone(),
    }
}


fn setup(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    window: Query<&Window>,
    typing_buffer: Res<TypingBuffer>,
) {
    // commands
    //     .spawn(NodeBundle {
    //         style: Style {
    //             width: Val::Percent(100.),
    //             height: Val::Percent(100.),
    //             flex_direction: FlexDirection::Column,
    //             ..default()
    //         },
    //         ..default()
    //     })
    //     .with_children(|parent| {

    // commands.spawn((
    //     TextBundle::from_section(
    //         typing_buffer.0.clone(),
    //         TextStyle {
    //             font_size: 50.0,
    //             ..default()
    //         }
    //     )
    //     // .with_text_alignment(TextAlignment::Center)
    //     .with_style(Style {
    //             // align_self: AlignSelf::FlexEnd,
    //             flex_direction: FlexDirection::Row,
    //             align_items: AlignItems::Center,
    //             position_type: PositionType::Absolute,
    //             top: Val::Percent(50.0),
    //             left: Val::Percent(50.0),
    //             max_width: Val::Px(200.0),
    //             max_height: Val::Percent(100.0),
    //             flex_wrap: FlexWrap::WrapReverse,
    //             // flex_wrap: FlexWrap::Wrap,
    //             // bottom: Val::Px(5.0),
    //             // right: Val::Px(5.0),
    //             ..default()
    //         }),
    //     BufferText,
    //     )
    // );

    // });


    let window = window.single();
    let max_x: f32 = window.width() / 2.0;
    let max_y = window.height() / 2.0;

    create_borders(&mut commands, &textures, max_x, max_y);

    // commands.spawn(Sprite {
    //     color: Color::RED,
    //     custom_size: Some(Vec2::new(max_x, 10.0)),
    //     ..default()
    // })
    commands.spawn(
        SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::new(window.width() - 85.0, 10.0)),
                ..default()
            }, // Set the size (20x20)
            transform: Transform::from_translation(Vec3::new(0.0, window.height() - 294.0, 0.0)),
            // transform: Transform::from_translation(Vec3::new(0.0, window.height() - 800.0, 0.0)),
            ..default()
        }
    )
    .insert(Collider::cuboid(window.width() - 85.0, 10.0))
    .insert(Sensor)
    .insert(GameOverLine);

    commands.spawn(
        SpriteBundle {
            sprite: Sprite {
                // color: Color::rgb(0.2, 0.2, 0.2),
                custom_size: Some(Vec2::new(window.width(), window.height())),
                ..default()
            },
            texture: textures.background.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, max_y, -4.0)),
            // transform: Transform::from_translation(Vec3::new(0.0, window.height() - 800.0, 0.0)),
            ..default()
        }
    );
}


fn create_borders(commands: &mut Commands, textures: &Res<TextureAssets>, max_x: f32, max_y: f32) {
    let y_pos = 16.0;
    let x_pos = (max_x * -1.0) + 16.0;
    let iterations_x = (max_x % BORDER_TILE_SIZE) + max_x;
    let iterations_y = ((max_y % BORDER_TILE_SIZE) + max_y) * 10.0;

    for x in (0..=iterations_x as u32).step_by(BORDER_TILE_SIZE as usize) {
        commands.spawn(
            get_border_tile(Vec3::new(x as f32, y_pos, 1.0), textures.ground.clone(), BORDER_TILE_SCALE.clone())
        ).insert(Collider::cuboid(BORDER_TILE_SIZE / 2.0, BORDER_TILE_SIZE / 2.0))
        .insert(Floor);

        commands.spawn(
            get_border_tile(Vec3::new(x as f32 * -1.0, y_pos, 1.0), textures.ground.clone(), BORDER_TILE_SCALE.clone())
        ).insert(Collider::cuboid(BORDER_TILE_SIZE / 2.0, BORDER_TILE_SIZE / 2.0))
        .insert(Floor);
    }

    for y in (0..=iterations_y as u32).step_by(BORDER_TILE_SIZE as usize) {
        commands.spawn(
            get_border_tile(Vec3::new(x_pos, y as f32, 0.0), textures.wall.clone(), BORDER_TILE_SCALE.clone())
        )
        .insert(Collider::cuboid(BORDER_TILE_SIZE / 2.0, BORDER_TILE_SIZE / 2.0))
        .insert(Wall);
        // .with_children(|parent| {
        //     parent.spawn(
        //         Text2dBundle {
        //             text: Text::from_section(
        //                     format!("Position: {:?}", (x_pos, y)),
        //                     TextStyle {
        //                         font_size: 20.0,
        //                         color: Color::RED,
        //                         ..default()
        //                     }
        //                 ),
        //             // transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
        //             ..default()
        //         }
        //     );
        // });

        commands.spawn(
            get_border_tile(Vec3::new(-x_pos, y as f32, 0.0), textures.wall.clone(), BORDER_TILE_SCALE.clone())
        )
        .insert(Collider::cuboid(BORDER_TILE_SIZE / 2.0, BORDER_TILE_SIZE / 2.0))
        .insert(Wall);
    }
}

fn get_border_tile(position: Vec3, texture: Handle<Image>, texture_scale: Vec2) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite {
            custom_size: Some(texture_scale),
            ..default()
        },
        texture,
        transform: Transform::from_translation(position),
        ..Default::default()
    }
}


fn update_buffer_text(
    typing_buffer: Res<TypingBuffer>,
    mut query: Query<&mut Text, With<BufferText>>,
) {
    for mut text in &mut query {
        text.sections[0].value = typing_buffer.0.clone();
    }
}


// https://github.com/bevyengine/bevy/issues/1780#issuecomment-1760929069
fn fix_trash_label_rotation(
    mut text_query: Query<(&Parent, &mut Transform), With<TrashText>>,
    query_parents: Query<&Transform, (With<Trash>, Without<TrashText>, With<TrashActionActive>)>,
) {
    for (parent, mut transform) in text_query.iter_mut() {
        if let Ok(parent_transform) = query_parents.get(parent.get()) {
            transform.rotation = parent_transform.rotation.inverse();
        }
    }
}


fn remove_all_marked_trash(
    mut commands: Commands,
    marked_trash_query: Query<Entity, With<TrashMarked>>,
) {
    for entity in &mut marked_trash_query.iter() {
        commands.entity(entity).remove::<TrashMarked>();
    }
}

fn trash_power_ups_effects(
    mut active_trash_query: Query<(&Trash, &mut Sprite), With<TrashActionActive>>,
    mut inactive_trash_query: Query<&mut Sprite, Without<TrashActionActive>>,
    time: Res<Time>,
) {
    let seconds = time.elapsed_seconds();
    let color_change_interval = (3.5 * seconds).sin() / 2.0 + 0.5;
    // let explosion_color = (3.5 * seconds).sin() / 2.0 + 0.5;

    for (trash, mut sprite) in &mut active_trash_query.iter_mut() {
        match trash.power_up {
            PowerUp::Explosion => {
                sprite.color = Color::rgb(
                    1.0,
                    color_change_interval,
                    color_change_interval
                );
            },
            PowerUp::Link => {
                sprite.color = Color::rgb(
                    color_change_interval,
                    color_change_interval,
                    1.0,
                );
            },
            _ => {}
        }
    }

    for mut sprite in &mut inactive_trash_query.iter_mut() {
        if sprite.color != Color::WHITE {
            sprite.color = Color::WHITE;
        }
    }
}


fn typing(
    mut commands: Commands,
    mut typing_buffer: ResMut<TypingBuffer>,
    mut combo_meter_query: Query<&mut ProgressBar, With<ComboMeter>>,
    mut combo_modifier: ResMut<ComboModifier>,
    keyboard_input: Res<Input<KeyCode>>,
    trash_query: Query<(&Parent, &TrashText)>,
    marked_trash_query: Query<Entity, With<TrashMarked>>,
) {

    if keyboard_input.pressed(KeyCode::ControlLeft) {
        if keyboard_input.just_pressed(KeyCode::Back) {
            typing_buffer.0 = "".to_string();
            remove_all_marked_trash(commands, marked_trash_query);
            return
        }
    }

    let mut buffer_word = typing_buffer.0.clone();
    let mut did_delete_letter = false;

    for key in keyboard_input.get_just_pressed() {
        match key {
            KeyCode::A => buffer_word.push('a'),
            KeyCode::B => buffer_word.push('b'),
            KeyCode::C => buffer_word.push('c'),
            KeyCode::D => buffer_word.push('d'),
            KeyCode::E => buffer_word.push('e'),
            KeyCode::F => buffer_word.push('f'),
            KeyCode::G => buffer_word.push('g'),
            KeyCode::H => buffer_word.push('h'),
            KeyCode::I => buffer_word.push('i'),
            KeyCode::J => buffer_word.push('j'),
            KeyCode::K => buffer_word.push('k'),
            KeyCode::L => buffer_word.push('l'),
            KeyCode::M => buffer_word.push('m'),
            KeyCode::N => buffer_word.push('n'),
            KeyCode::O => buffer_word.push('o'),
            KeyCode::P => buffer_word.push('p'),
            KeyCode::Q => buffer_word.push('q'),
            KeyCode::R => buffer_word.push('r'),
            KeyCode::S => buffer_word.push('s'),
            KeyCode::T => buffer_word.push('t'),
            KeyCode::U => buffer_word.push('u'),
            KeyCode::V => buffer_word.push('v'),
            KeyCode::W => buffer_word.push('w'),
            KeyCode::X => buffer_word.push('x'),
            KeyCode::Y => buffer_word.push('y'),
            KeyCode::Z => buffer_word.push('z'),
            KeyCode::Back => { 
                let _ = buffer_word.pop(); 
                did_delete_letter = true;
            },
            _ => {}
        }
    }

    if buffer_word == typing_buffer.0 {
        return;
    }

    let mut to_be_removed = vec![];
    let mut is_existing_matching_word = false;

    for (parent, trash_text) in trash_query.iter() {
        if trash_text.word.starts_with(&buffer_word) {
            if marked_trash_query.get(parent.get()).is_err() {
                commands.entity(parent.get()).insert(TrashMarked);
            }
            is_existing_matching_word = true;
        } else {
            if marked_trash_query.get(parent.get()).is_ok() {
                to_be_removed.push(parent.get());
            }
        }
    }

    // TODO: Make the combo meter reset only for words that the player tried to type and then
    // failed, not for every trash that lands. It's too hard
    if is_existing_matching_word {
        typing_buffer.0 = buffer_word.clone();
        if !did_delete_letter {
            for mut progress_bar in &mut combo_meter_query.iter_mut() {
                progress_bar.increase_progress(0.1);
            }
        }

        for entity in to_be_removed {
            commands.entity(entity).remove::<TrashMarked>();
        }

    } else {
        for mut progress_bar in &mut combo_meter_query.iter_mut() {
            progress_bar.reset();
            combo_modifier.0 = 1;
       }

    }

}


fn highlight_character(
    mut trash_query: Query<(&TrashText, &mut Text)>,
    typing_buffer: Res<TypingBuffer>,
) {
    for (trash_text, mut ui_text) in &mut trash_query.iter_mut() {
        if trash_text.word.starts_with(&typing_buffer.0) {
            ui_text.sections = highlight_characters(&ui_text.sections, typing_buffer.0.len(), trash_text.highlight_color)
        } else {
            ui_text.sections = remove_highlight(&ui_text.sections, trash_text.color)
        }
    }
}

fn remove_explosions(
    mut commands: Commands,
    explosion_query: Query<Entity, With<TrashExplosion>>,
) {
    for entity in &mut explosion_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}


fn remove_trash_text(commands: &mut Commands, trash_entity: &Entity) {
    commands.entity(*trash_entity).remove::<TrashActionActive>();
    commands.entity(*trash_entity).remove::<TrashMarked>();
    commands.entity(*trash_entity).despawn_descendants();

    // commands.entity(*trash_entity).insert(TrashActionDuplicate);
}

fn create_duplicated_trash_from_entity(commands: &mut Commands, sprite: Handle<Image>, trash: Trash, transform: Transform) {
    // let trash = duplicate_trash_query.get_component::<Trash>(*entity1).unwrap();
    let mut trash = trash.clone();
    trash.power_up = PowerUp::None;
    trash.activated = false;

    let mut bundle = TrashBundle::new(sprite, trash);
    bundle.velocity = Velocity::zero();

    commands.spawn(bundle)
        .insert(transform)
        .insert(GravityScale(40.0));
        // .insert(TrashActionDuplicate);
}

fn should_delete_text(
    entity: &Entity,
    other: &Entity,
    active_trash_query: &Query<(Entity, &Velocity, &Trash, &Transform), With<TrashActionActive>>,
    inactive_trash_query: &Query<(Entity, &Velocity), (Without<TrashActionActive>, With<Trash>)>,
    // mut duplicate_trash_query: Query<(Entity, &Trash, &Transform, &Handle<Image>), (Without<TrashActionActive>, With<TrashActionDuplicate>)>,
    walls_query: &Query<Entity, With<Wall>>,
    floor_query: &Query<Entity, With<Floor>>,

) -> bool {

    if walls_query.get(*entity).is_ok() || walls_query.get(*other).is_ok() {
        return false;
    }

    if let Ok(active_trash) = active_trash_query.get(*entity) {
        // println!("Active trash: {:?}", active_trash);
        if floor_query.get(*other).is_ok() {
            // println!("We hit the floor");
            return true;
            // remove_trash_text(&mut commands, &active_trash.0);
            // typing_buffer.0 = "".to_string();
        }

        if active_trash_query.get(*other).is_err() {
            if let Ok(inactive_trash) = inactive_trash_query.get(*other) {
                if inactive_trash.1.linvel.length() < TRASH_STARTING_VELOCITY.length() / 2.0 {
                    return true;
                    // remove_trash_text(&mut commands, &entity);
                }
                // if inactive_trash.1.linvel.length() < active_trash.1.linvel.length() {
                //     // should_remove = true;
                //     return true;
                // }
            }
        }

        // if should_remove {
        //     remove_trash_text(&mut commands, &active_trash.0);
        //     typing_buffer.0 = "".to_string();
        // }
    }

    return false;
}

fn should_duplicate_trash(
    entity: &Entity,
    other: &Entity,
    active_trash_query: &Query<(Entity, &Velocity, &Trash, &Transform), With<TrashActionActive>>,
    duplicate_trash_query: &Query<(Entity, &Trash, &Transform, &Handle<Image>), With<TrashActionDuplicate>>,
    walls_query: &Query<Entity, With<Wall>>,
    floor_query: &Query<Entity, With<Floor>>,
    game_over_query: &Query<Entity, With<GameOverLine>>,

) -> bool {
    if duplicate_trash_query.get(*entity).is_ok() {
        if floor_query.get(*other).is_err() &&
            walls_query.get(*other).is_err() && 
            game_over_query.get(*other).is_err() {
            return true;
        }
    }

    false
}


fn should_explode(
    entity: &Entity,
    other: &Entity,
    explosion_query: &Query<&Transform, With<TrashExplosion>>,
    trash_query: &Query<Entity, With<Trash>>
) -> bool {

    if let Ok(_) = explosion_query.get(*other) {
        if trash_query.get(*entity).is_ok() {
            return true;
        }
    }

    false
}


fn handle_power_up_event(
    entity: &Entity,
    // texture: Handle<Image>,
    commands: &mut Commands,
    trash_query: &Query<(Entity, &Velocity, &Trash, &Transform), With<TrashActionActive>>,
) -> PowerUpEvent {
    // for (trash, transform) in &mut trash_query.iter() {
    if let Ok((_, _, trash, transform)) = trash_query.get(*entity) {
        if trash.activated {
            match trash.power_up {
                PowerUp::Explosion => {
                    commands.spawn(
                        SpriteBundle {
                            ..default()
                        }
                    )
                    .insert(transform.clone())
                    .insert(Collider::cuboid(50.0, 50.0))
                    .insert(Sensor)
                    .insert(TrashExplosion);

                    // commands.entity(*entity).despawn_recursive();
                    return PowerUpEvent::EntityDestroyed;
                    // commands.entity(transform.parent().unwrap().id()).insert(TrashExplosion);
                    // commands.entity(transform.parent().unwrap().id()).remove::<TrashActionActive>();
                    // commands.entity(transform.parent().unwrap().id()).remove::<TrashMarked>();
                    // commands.entity(transform.parent().unwrap().id()).despawn_descendants();
                },
                PowerUp::Link => {
                    return PowerUpEvent::DestroyLinked;
                },
                _ => {}
            }
        }
    }

    PowerUpEvent::None
    // }
}

fn handle_trash_entity_collision(
    entity: &Entity,
    other: &Entity,
    commands: &mut Commands,
    typing_buffer: &mut ResMut<TypingBuffer>,
    active_trash_query: &Query<(Entity, &Velocity, &Trash, &Transform), With<TrashActionActive>>,
    inactive_trash_query: &Query<(Entity, &Velocity), (Without<TrashActionActive>, With<Trash>)>,
    duplicate_trash_query: &Query<(Entity, &Trash, &Transform, &Handle<Image>), With<TrashActionDuplicate>>,
    marked_trash_query: &Query<Entity, With<TrashMarked>>,
    // trash_query: Query<(&mut Trash, &Transform), With<TrashActionActive>>,
    explosion_query: &Query<&Transform, With<TrashExplosion>>,
    all_trash_query: &Query<Entity, With<Trash>>,
    // duplicate_trash_query: Query<&Trash, (Without<TrashActionActive>, With<TrashActionDuplicate>)>,
    walls_query: &Query<Entity, With<Wall>>,
    floor_query: &Query<Entity, With<Floor>>,
    game_over_query: &Query<Entity, With<GameOverLine>>,

) {
    let powerup_event = handle_power_up_event(entity, commands, active_trash_query);

    if should_explode(entity, other, explosion_query, all_trash_query) {
        if marked_trash_query.get(*entity).is_ok() {
            typing_buffer.0 = "".to_string();
        }
        commands.entity(*entity).despawn_recursive();
        return;
    }

    let mut should_remove_text = false;
    if should_delete_text(entity, other, active_trash_query, inactive_trash_query, walls_query, floor_query) {
        if marked_trash_query.get(*entity).is_ok() {
            typing_buffer.0 = "".to_string();
        }
        should_remove_text = true;

    }

    if should_duplicate_trash(entity, other, active_trash_query, duplicate_trash_query, walls_query, floor_query, game_over_query) {
        let trash_data = duplicate_trash_query.get(*entity).unwrap();
        commands.entity(trash_data.0).remove::<TrashActionDuplicate>();
        let mut transform = trash_data.2.clone();
        transform.translation.y += 10.0;
        create_duplicated_trash_from_entity(
            commands,
            trash_data.3.clone(),
            trash_data.1.clone(),
            trash_data.2.clone());
    }


    if powerup_event != PowerUpEvent::EntityDestroyed && should_remove_text {
        remove_trash_text(commands, entity);
    }
}


fn is_game_over(
    entity: &Entity,
    other: &Entity,
    commands: &mut Commands,
    inactive_trash_query: &Query<(Entity, &Velocity), (Without<TrashActionActive>, With<Trash>)>,
    game_over_query: &Query<Entity, With<GameOverLine>>,
) -> bool {
        if game_over_query.get(*entity).is_ok() || game_over_query.get(*other).is_ok() {
            if inactive_trash_query.get(*entity).is_ok() || inactive_trash_query.get(*other).is_ok() {
                return true;
            //         SpriteBundle {
            // ..default()
            //         }
            //     )
            //     .insert(Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)))
            //     .insert(Collider::cuboid(100.0, 100.0))
            //     .insert(Sensor)
            //     .insert(TrashExplosion);

            }
        }

    return false;
}

fn delete_all_play_entities(
    mut commands: Commands,
    query: Query<Entity, (Without<GameOver>, Without<Camera>, Without<Window>)>,
) {
    for entity in &mut query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn delete_all_gameover_entities(
    mut commands: Commands,
    game_over_entities: Query<Entity, With<GameOver>>,
) {
    for entity in &mut game_over_entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn spawn_game_over_menu(
    mut commands: Commands,
    score: Res<Score>,
    // mut next_state: ResMut<NextState<GameState>>,
) {
    let style = Style {
        position_type: PositionType::Absolute,
        // width: Val::Percent(100.0),
        // height: Val::Px(10.0),
        // bottom: Val::Px(0.0),
        top: Val::Percent(30.0),
        left: Val::Px(120.0),
        ..default()
    };
    commands.spawn(
        TextBundle {
            text: Text::from_section(
                "Game Over".to_string(),
                TextStyle {
                    font_size: 100.0,
                    color: Color::RED,
                    ..default()
                }
            ),
            style,
            ..default()
        },
    )
    .insert(Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)))
    .insert(GameOver);

    commands.spawn(
        TextBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Score: ".to_string(),
                        style: TextStyle {
                            font_size: 50.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    },
                    TextSection {
                        value: score.0.to_string(),
                        style: TextStyle {
                            font_size: 50.0,
                            color: Color::GREEN,
                            ..default()
                        },
                    },
                ],
                ..default()
            },


            // from_section(
            //     format!("Score: {}", score.0),
            //     TextStyle {
            //         font_size: 100.0,
            //         color: Color::,
            //         ..default()
            //     }
            // ),
            style: Style {
                position_type: PositionType::Absolute,
                // width: Val::Percent(100.0),
                // height: Val::Px(10.0),
                // bottom: Val::Px(0.0),
                top: Val::Px(100.0),
                left: Val::Px(120.0),
                ..default()
            },
            ..default()
        },
    )
    .insert(Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)))
    .insert(GameOver);


    let button_colors = ButtonColors::default();
    commands.spawn((
        ButtonBundle {
            // text: Text::from_section(
            //     "Game Over".to_string(),
            //     TextStyle {
            //         font_size: 100.0,
            //         color: Color::RED,
            //         ..default()
            //     }
            // ),
            style: Style {
                position_type: PositionType::Absolute,
                // width: Val::Percent(100.0),
                // height: Val::Px(10.0),
                // bottom: Val::Px(0.0),
                top: Val::Percent(50.0),
                left: Val::Px(260.0),
                ..default()
            },
            background_color: button_colors.normal.into(),
            ..default()
        },
        ButtonColors::default(),
        ChangeState(GameState::Playing),
    ))
    .insert(Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)))
    .insert(GameOver)
    .with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "Restart",
            TextStyle {
                font_size: 40.0,
                color: Color::rgb(0.9, 0.9, 0.9),
                ..default()
            },
        ));
    });
}

fn click_restart_button(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (
        &Interaction,
        &mut BackgroundColor,
        &mut ButtonColors,
        Option<&ChangeState>,
    ),
    (Changed<Interaction>, With<Button>)
    >,
) {
    for (interaction, mut color, button_colors, change_state) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if let Some(state) = change_state {
                    next_state.set(state.0.clone());
                } 
            }
            Interaction::Hovered => {
                *color = button_colors.hovered.into();
            }
            Interaction::None => {
                *color = button_colors.normal.into();
            }
        }
    }

}

fn handle_trash_collision(
    mut commands: Commands,
    mut typing_buffer: ResMut<TypingBuffer>,
    mut collision_events: EventReader<CollisionEvent>,
    mut next_state: ResMut<NextState<GameState>>,
    // mut trash_query: Query<(&mut Trash, &Transform), With<TrashActionActive>>,
    active_trash_query: Query<(Entity, &Velocity, &Trash, &Transform), With<TrashActionActive>>,
    inactive_trash_query: Query<(Entity, &Velocity), (Without<TrashActionActive>, With<Trash>)>,
    duplicate_trash_query: Query<(Entity, &Trash, &Transform, &Handle<Image>), With<TrashActionDuplicate>>,
    marked_trash_query: Query<Entity, With<TrashMarked>>,
    // trash_query: Query<(&mut Trash, &Transform), With<TrashActionActive>>,
    explosion_query: Query<&Transform, With<TrashExplosion>>,
    all_trash_query: Query<Entity, With<Trash>>,
    // duplicate_trash_query: Query<&Trash, (Without<TrashActionActive>, With<TrashActionDuplicate>)>,
    walls_query: Query<Entity, With<Wall>>,
    floor_query: Query<Entity, With<Floor>>,
    game_over_query: Query<Entity, With<GameOverLine>>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity1, entity2, _) => {

                if is_game_over(entity1, entity2, &mut commands, &inactive_trash_query, &game_over_query) {
                    commands.spawn(GameOver);
                    next_state.set(GameState::GameOver);
                    return;
                }

                handle_trash_entity_collision(
                    entity1,
                    entity2,
                    &mut commands,
                    &mut typing_buffer,
                    &active_trash_query,
                    &inactive_trash_query,
                    &duplicate_trash_query,
                    &marked_trash_query,
                    &explosion_query,
                    &all_trash_query,
                    &walls_query,
                    &floor_query,
                    &game_over_query
                );

                handle_trash_entity_collision(
                    entity2,
                    entity1,
                    &mut commands,
                    &mut typing_buffer,
                    &active_trash_query,
                    &inactive_trash_query,
                    &duplicate_trash_query,
                    &marked_trash_query,
                    &explosion_query,
                    &all_trash_query,
                    &walls_query,
                    &floor_query,
                    &game_over_query
                );
                // let mut should_remove_text = false;
                //
                // if should_delete_text(entity1, entity2, &active_trash_query, &inactive_trash_query, &walls_query, &floor_query) {
                //     if marked_trash_query.get(*entity1).is_ok() {
                //         typing_buffer.0 = "".to_string();
                //     }
                //     remove_trash_text(&mut commands, &entity1);
                //
                // } else if should_delete_text(entity2, entity1, &active_trash_query, &inactive_trash_query, &walls_query, &floor_query) {
                //     if marked_trash_query.get(*entity2).is_ok() {
                //         typing_buffer.0 = "".to_string();
                //     }
                //     remove_trash_text(&mut commands, &entity2);
                // }
                //
                // let mut duplicate_entity: Option<(Entity, &Trash, &Transform, &Handle<Image>)> = None;
                //
                // if should_duplicate_trash(entity1, entity2, &active_trash_query, &duplicate_trash_query, &walls_query, &floor_query) {
                //     let trash_data = duplicate_trash_query.get(*entity1).unwrap();
                //     commands.entity(trash_data.0).remove::<TrashActionDuplicate>();
                //     duplicate_entity = Some(trash_data.clone());
                //
                // }
                //
                // if should_duplicate_trash(entity2, entity1, &active_trash_query, &duplicate_trash_query, &walls_query, &floor_query) {
                //     let trash_data = duplicate_trash_query.get(*entity2).unwrap();
                //     commands.entity(trash_data.0).remove::<TrashActionDuplicate>();
                //     duplicate_entity = Some(trash_data.clone());
                //
                // }

                // handle_power_up_event(entity1, &mut commands, &active_trash_query);
                // handle_power_up_event(entity2, &mut commands, &active_trash_query);
                //
                // if should_explode(entity1, entity2, &explosion_query, &all_trash_query) {
                //     commands.entity(*entity1).despawn_recursive();
                // }
                //
                // if should_explode(entity2, entity1, &explosion_query, &all_trash_query) {
                //     commands.entity(*entity1).despawn_recursive();
                // }

                // if let Some(trash_data) = duplicate_entity {
                //     let mut transform = trash_data.2.clone();
                //     transform.translation.y += 10.0;
                //     create_duplicated_trash_from_entity(
                //         &mut commands,
                //         trash_data.3.clone(),
                //         trash_data.1.clone(),
                //         trash_data.2.clone());
                // }


            }
            CollisionEvent::Stopped(_entity1, _entity2, _) => {},
        }
    }
}

fn update_difficuly(
    mut trash_spawn_timer: ResMut<TrashSpawnTimer>,
    mut difficulty_timer: ResMut<DifficultyTimer>,
    time: Res<Time>,
) {


    if difficulty_timer.0.tick(time.delta()).just_finished() {
        if trash_spawn_timer.0.duration() <= Duration::from_secs_f32(1.0) {
            return;
            // let duration = trash_spawn_timer.0.duration().sub(Duration::from_secs_f32(0.1));
            // trash_spawn_timer.0.set_duration(duration) ;
        }

        let duration = trash_spawn_timer.0.duration().sub(Duration::from_secs_f32(0.2));
        trash_spawn_timer.0.set_duration(duration) ;

    }

    // if time.elapsed_seconds_f64() % 5.0 == 0.0 {
    // }
    // if trash_spawn_timer.0.duration() > 0.5 {
    //     let duration = trash_spawn_timer.0.duration().sub(Duration::from_secs_f32(0.1));
    //     trash_spawn_timer.0.set_duration(duration) ;
    // }
}

// fn destroy_trash_text(
//     mut commands: Commands,
//     trash_query: Query<(Entity, &Velocity), (With<Trash>, With<TrashActionActive>)>,
// ) {
//     for (entity, velocity) in &mut trash_query.iter() {
//         println!("Velocity: {:?}", velocity.linvel.length());
//         // if velocity.linvel.length() < TRASH_STARTING_VELOCITY.length() / 2.0 {
//         //     remove_trash_text(&mut commands, &entity);
//         // }
//     }
// }

fn clamp_duplicated_trash(
    mut trash_query: Query<&mut Velocity, (With<Trash>, Without<TrashActionActive>)>,
) {
    for mut velocity in trash_query.iter_mut() {
        if velocity.linvel.length() > TRASH_STARTING_VELOCITY.length() / 2.0 {
            velocity.linvel = velocity.linvel.clamp(Vec2::new(0.0, 0.0), Vec2::new(TRASH_MAXIMUM_HORIZONTAL_VELOCITY_LENGTH, TRASH_MAXIMUM_VERTICAL_VELOCITY_LENGTH ));
        }
        // println!("Velocity: {:?}", velocity.linvel.length());
        // if velocity.linvel.length() < TRASH_STARTING_VELOCITY.length() / 2.0 {
        //     remove_trash_text(&mut commands, &entity);
        // }
    }
}


fn activate_matching_trash(
    mut commands: Commands,
    trash_text_query: Query<(&Parent, &Transform, &TrashText)>,
    mut trash_query: Query<(&mut Trash, &Transform), With<TrashActionActive>>,
    mut typing_buffer: ResMut<TypingBuffer>,
    mut score: ResMut<Score>,
    combo_modifier: Res<ComboModifier>,
) {

    if !typing_buffer.is_changed() {
        return;
    }

    // let mut trash_to_destroy: Vec<&Parent> = Vec::new();
    let mut should_clear_buffer = false;

    for (entity, _transform, trash_text) in &mut trash_text_query.iter() {
        if typing_buffer.0 == trash_text.word {
            if let Ok(mut trash) = trash_query.get_mut(entity.get()) {
                if trash.0.activated {
                    continue;
                }

                match trash.0.power_up {
                    PowerUp::Explosion => {
                        commands.entity(entity.get()).remove::<TrashMarked>();
                        commands.entity(entity.get()).despawn_descendants();
                        // remove_trash_text(&mut commands, entity);
                        // commands.spawn(TrashExplosion)
                        //     .insert(trash.1.clone());
                    },
                    PowerUp::Link => {
                        commands.entity(entity.get()).remove::<TrashMarked>();
                        commands.entity(entity.get()).despawn_descendants();

                    },
                    PowerUp::None => {
                        commands.entity(entity.get()).despawn_recursive();
                    }
                }

                trash.0.activated = true;
                score.0 += 1 * combo_modifier.0;
                should_clear_buffer = true;
            }
            // trash_to_destroy.push(entity);
        }
    }
    // for entity in &trash_to_destroy {
    //     commands.entity(entity.get()).despawn_recursive();
    // }

    if should_clear_buffer {
        typing_buffer.0 = "".to_string();
    }
}


fn get_random_word(available_words: &Res<AvailableWords>) -> String {
    let mut random = rand::thread_rng();
    let mut random_words: Vec<String> = Vec::new();

    while random_words.len() == 0 {
        let random_letter = random.gen_range('a' .. 'z').to_string();
        random_words = available_words.0.get(random_letter.as_str()).unwrap().clone();
    }

    random_words[random.gen_range(0..random_words.len())].clone()
}


fn get_available_words_from_file() -> HashMap<String, Vec<String>> {
    let words: Vec<&str> = include_str!("../assets/no-swear.txt").split("\n").collect();

    let mut serialized_words: HashMap<String, Vec<String>>  = HashMap::from([
        ("a".to_string(), Vec::new()),
        ("b".to_string(), Vec::new()),
        ("c".to_string(), Vec::new()),
        ("d".to_string(), Vec::new()),
        ("e".to_string(), Vec::new()),
        ("f".to_string(), Vec::new()),
        ("g".to_string(), Vec::new()),
        ("h".to_string(), Vec::new()),
        ("i".to_string(), Vec::new()),
        ("j".to_string(), Vec::new()),
        ("k".to_string(), Vec::new()),
        ("l".to_string(), Vec::new()),
        ("m".to_string(), Vec::new()),
        ("n".to_string(), Vec::new()),
        ("o".to_string(), Vec::new()),
        ("p".to_string(), Vec::new()),
        ("q".to_string(), Vec::new()),
        ("r".to_string(), Vec::new()),
        ("s".to_string(), Vec::new()),
        ("t".to_string(), Vec::new()),
        ("u".to_string(), Vec::new()),
        ("v".to_string(), Vec::new()),
        ("w".to_string(), Vec::new()),
        ("x".to_string(), Vec::new()),
        ("y".to_string(), Vec::new()),
        ("z".to_string(), Vec::new()),
    ]);

    for word in words {
        let first_letter = match word.chars().next() {
            Some(letter) => letter.to_string(),
            None => continue,
        };

        serialized_words.get_mut(first_letter.as_str()).unwrap().push(word.to_string());
    }

    return serialized_words;
}
