use std::ops::Sub;
use std::time::Duration;

use crate::game_over::{GameOver, GameOverLine};
use crate::loading::TextureAssets;
use crate::{GameState, typing};
use crate::menu::{ButtonColors, ChangeState};
use crate::score::{Score, ComboMeter, ComboModifier};
use crate::trash::{TrashType, PowerUp, BufferText, TrashActionDuplicate, TrashActionActive, TrashBundle, get_trash_sprite, TrashMarked, Trash};
use crate::trash_text::{TrashText, TrashTextBundle, highlight_characters, remove_highlight};
use crate::typing::TypingBuffer;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::transform::TransformSystem;
use bevy::utils::HashMap;
use bevy_progressbar::ProgressBar;
use bevy_rapier2d::prelude::*;
use rand::Rng;


pub struct PlayPlugin;


#[derive(Component, Default)]
pub struct Wall;

#[derive(Component, Default)]
pub struct Floor;

#[derive(Resource)]
pub struct TrashSpawnTimer(pub Timer);

#[derive(Resource)]
struct DifficultyTimer(pub Timer);

#[derive(Resource)]
struct BufferTextDeleteTimer(Timer);

#[derive(Resource)]
pub struct AvailableWords(HashMap<String, Vec<String>>);


pub const WINDOW_WIDTH: f32 = 700.0;
pub const WINDOW_HEIGHT: f32 = 800.0;

const BORDER_TILE_SIZE: f32 = 48.0;
const BORDER_TILE_SCALE: Vec2 = Vec2::new(BORDER_TILE_SIZE, BORDER_TILE_SIZE);

const TRASH_STARTING_VELOCITY: Vec2 = Vec2::new(0.0, -100.0);
const TRASH_MAXIMUM_VERTICAL_VELOCITY_LENGTH: f32 = 40.0;
const TRASH_MAXIMUM_HORIZONTAL_VELOCITY_LENGTH: f32 = 600.0;
const TRASH_SPAWN_DISTANCE_BETWEEN_SPAWNS: f32 = 30.0;

const INITIAL_TRASH_SPAWN_RATE: f32 = 2.0;
const INITIAL_DIFICULTY_INCREASE_RATE: f32 = 10.0;


impl Plugin for PlayPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DifficultyTimer(Timer::from_seconds(INITIAL_DIFICULTY_INCREASE_RATE, TimerMode::Repeating)))
        .insert_resource(TrashSpawnTimer(Timer::from_seconds(INITIAL_TRASH_SPAWN_RATE, TimerMode::Repeating)))
        .insert_resource(AvailableWords(get_available_words_from_file()))
        .add_systems(OnEnter(GameState::Playing), setup)
        .add_systems(Update, (
                spawn_trash.run_if(in_state(GameState::Playing)),
                // update_trash.after(update_on_wrong_letter).run_if(in_state(GameState::Playing)),
                update_difficuly.after(setup).run_if(in_state(GameState::Playing)),
                update_on_wrong_letter.after(typing::typing),
                update_buffer_text.after(typing::typing),
                clean_typing_buffer.after(update_on_wrong_letter),
            ))
        .add_systems(OnExit(GameState::Playing), delete_all_play_entities);

        // app.insert_resource(TypingBuffer("".to_string()))
        // .add_systems(Update, (
        //         typing.run_if(in_state(GameState::Playing)),
        //     ));
        // app.insert_resource(TrashSpawnTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        // app.add_systems(OnEnter(GameState::Playing), spawn_player)
        // .add_systems(Update, spawn_trash.run_if(in_state(GameState::Playing)));
    }
}


fn setup(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    window: Query<&Window>,
    mut trash_spawn_timer: ResMut<TrashSpawnTimer>,
    mut difficulty_timer: ResMut<DifficultyTimer>,
    // typing_buffer: Res<TypingBuffer>,
) {
    trash_spawn_timer.0.reset();
    difficulty_timer.0.reset();
    trash_spawn_timer.0.set_duration(Duration::from_secs(INITIAL_TRASH_SPAWN_RATE as u64));
    difficulty_timer.0.set_duration(Duration::from_secs(INITIAL_DIFICULTY_INCREASE_RATE as u64));

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
    // let max_x: f32 = window.width() / 2.0;
    // let max_y = window.height() / 2.0;
    let max_x: f32 = WINDOW_WIDTH / 2.0;
    let max_y = WINDOW_HEIGHT / 2.0;

    create_borders(&mut commands, &textures, max_x, max_y);

    commands.spawn(
        SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::new(WINDOW_WIDTH - 85.0, 10.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, WINDOW_HEIGHT - 390.0, 0.0)),
            ..default()
        }
    )
    .insert(Collider::cuboid(window.width() - 85.0, 10.0))
    .insert(Sensor)
    .insert(GameOverLine);

    commands.spawn(
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT)),
                ..default()
            },
            texture: textures.background.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, max_y, -4.0)),
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

    for y in (16..=iterations_y as u32).step_by(BORDER_TILE_SIZE as usize) {
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


pub fn spawn_trash(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    // keyboard_input: Res<Input<KeyCode>>,
    // window: Query<&Window>,
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
        // let window = window.single();

        let mut random = rand::thread_rng();
        // let max_x: f32 = window.width() / 2.0;
        // let y_pos = (window.height() / 2.0) * 2.0;
        let max_x: f32 = WINDOW_WIDTH / 2.0;
        let y_pos = WINDOW_HEIGHT;

        let random_x = get_random_coordinate(max_x, *previous_spawn_position);
        *previous_spawn_position = random_x;
        let trash_type: TrashType = SPAWN_CHANCES[random.gen_range(0..SPAWN_CHANCES.len())].clone();
        let power_up: PowerUp = POWER_UP_CHANCES[random.gen_range(0..POWER_UP_CHANCES.len())].clone();

        // TODO: make sure the same word doesn't appear twice in a row
        // A solution might be to have search to search for a word as long as it's not in a list of
        // already used words which we can get from a query
        // Also we need to be able to to limit the amount of letters in a word
        let mut trash = Trash::get_by_type(trash_type);
        if power_up != PowerUp::None {
            trash.power_up = power_up;
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
            });

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
        }

        let duration = trash_spawn_timer.0.duration().sub(Duration::from_secs_f32(0.2));
        trash_spawn_timer.0.set_duration(duration) ;

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


pub fn clean_typing_buffer(
    mut typing_buffer: ResMut<TypingBuffer>,
    marked_trash_query: Query<Entity, With<TrashMarked>>,
    trash_query: Query<(&Parent, &TrashText)>,
    mut previous_typing_buffer: Local<String>,

) {

    let mut is_word_exists = false;
    if marked_trash_query.is_empty() {
        for (_, trash_text) in trash_query.iter() {
            if trash_text.word.starts_with(&*previous_typing_buffer) {
                is_word_exists = true;
                break;
            }
        }
        if !is_word_exists {
            typing_buffer.0 = "".to_string();
        }
    }

    if *previous_typing_buffer != typing_buffer.0 && !is_word_exists {
        *previous_typing_buffer = typing_buffer.0.clone();
    }
}

pub fn update_on_wrong_letter(
    mut commands: Commands,
    mut typing_buffer: ResMut<TypingBuffer>,
    mut combo_meter_query: Query<&mut ProgressBar, With<ComboMeter>>,
    mut combo_modifier: ResMut<ComboModifier>,
    // keyboard_input: Res<Input<KeyCode>>,
    trash_query: Query<(&Parent, &TrashText)>,
    marked_trash_query: Query<Entity, With<TrashMarked>>,
    mut previous_typing_buffer: Local<String>,

) {

    if !typing_buffer.is_changed() {
        return;
    }

    let mut to_be_removed = vec![];
    let mut is_existing_matching_word = false;

    for (parent, trash_text) in trash_query.iter() {
        if trash_text.word.starts_with(&typing_buffer.0) {
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

    if !is_existing_matching_word {
        for mut progress_bar in &mut combo_meter_query.iter_mut() {
            progress_bar.reset();
            combo_modifier.0 = 1;
        }

        typing_buffer.0 = previous_typing_buffer.clone();

    } else {
        *previous_typing_buffer = typing_buffer.0.clone();
    }


    for entity in to_be_removed {
        commands.entity(entity).remove::<TrashMarked>();
    }
}


fn delete_all_play_entities(
    mut commands: Commands,
    query: Query<Entity, (Without<GameOver>, Without<Camera>, Without<Window>)>,
) {
    for entity in &mut query.iter() {
        commands.entity(entity).despawn_recursive();
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
