use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::utils::HashMap;
use bevy_rapier2d::prelude::*;
use rand::Rng;

pub struct TrashPlugin;

#[derive(Clone, Debug)]
pub enum TrashType {
    Bottle,
    Pizza
}


#[derive(Component, Debug, Clone)]
pub struct Trash {
    pub trash_type: TrashType,
    pub word: String,
    pub size: Vec2,
}

#[derive(Component, Debug, Clone)]
pub struct BufferText;

#[derive(Resource)]
struct TrashSpawnTimer(Timer);

#[derive(Resource)]
pub struct TypingBuffer(String);

#[derive(Resource)]
pub struct AvailableWords(HashMap<String, Vec<String>>);

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for TrashPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TrashSpawnTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .insert_resource(TypingBuffer("".to_string()))
        .insert_resource(AvailableWords(get_available_words_from_file()))
        .add_systems(OnEnter(GameState::Playing), setup)
        .add_systems(Update, (
                spawn_trash.run_if(in_state(GameState::Playing)),
                typing.run_if(in_state(GameState::Playing)),
                destroy_matching_trash.after(typing),
                update_buffer_text.after(typing),
            )
        );
    }
}

impl Trash {
    pub fn get_by_type(trash_type: TrashType, word: String) -> Self {
        match trash_type {
            TrashType::Bottle => Self::bottle(word),
            TrashType::Pizza => Self::pizza(word),
        }
    }

    pub fn bottle(word: String) -> Self {
        Self {
            trash_type: TrashType::Bottle,
            word,
            size: Vec2::new(15.0, 16.0),
        }
    }

    pub fn pizza(word: String) -> Self {
        Self {
            trash_type: TrashType::Pizza,
            word,
            size: Vec2::new(32.0, 16.0),
        }
    }
}


fn spawn_trash(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    // keyboard_input: Res<Input<KeyCode>>,
    window: Query<&Window>,
    time: Res<Time>,
    available_words: Res<AvailableWords>,
    mut spawn_timer: ResMut<TrashSpawnTimer>,
) {

    static SPAWN_CHANCES: [TrashType; 2] = [TrashType::Bottle, TrashType::Pizza];

    if spawn_timer.0.tick(time.delta()).just_finished() {
        let window = window.single();

        let mut random = rand::thread_rng();
        let max_x: f32 = window.width() / 2.0;
        let y_pos = window.height() / 2.0;

        let random_x: f32 = random.gen_range(-max_x .. max_x);
        let trash_type: TrashType = SPAWN_CHANCES[random.gen_range(0..SPAWN_CHANCES.len())].clone();

        // TODO: make sure the same word doesn't appear twice in a row
        // A solution might be to have search to search for a word as long as it's not in a list of
        // already used words which we can get from a query
        // Also we need to be able to to limit the amount of letters in a word
        let trash = Trash::get_by_type(trash_type, get_random_word(&available_words));
        //     trash_type,
        //     word: 
        // };

        println!("Spawning trash: {:?}", trash);
        create_trash(&mut commands, &textures, trash, Vec2::new(random_x as f32, y_pos));
    }


    // match keyboard_input.pressed(KeyCode::B) {
    //     true => {
    //         let trash = Trash {
    //             trash_type: TrashType::Bottle
    //         };
    //         create_trash(&mut commands, &textures, trash, Vec2::new(random_x as f32, y_pos));
    //     }
    //     false => {}
    // }
    //
    // match keyboard_input.pressed(KeyCode::P) {
    //     true => {
    //         let trash = Trash {
    //             trash_type: TrashType::Pizza
    //         };
    //         create_trash(&mut commands, &textures, trash, Vec2::new(random_x as f32, y_pos));
    //     }
    //     false => {}
    // }

}


pub fn create_trash(commands: &mut Commands, textures: &Res<TextureAssets>, trash: Trash, pos: Vec2) {
    let sprite = get_trash_sprite(&trash.trash_type, textures);
    let mut transform = Transform::from_translation(Vec3::new(pos.x, pos.y, 0.0));
    transform.scale = Vec3::new(1.5, 1.5, 1.5);
    transform.rotation = Quat::from_rotation_y(30.0);

    let text = commands.spawn(Text2dBundle {
            text: Text::from_section(trash.word.clone(), TextStyle {
                color: Color::WHITE,
                font_size: 20.0,
                ..default()

            }),//.with_alignment(TextAlignment::Center),
            // transform: Transform::from_translation(Vec3::new(pos.x, pos.y, 1.0)),
            // transform: transform.clone(),
            // Custom anchor point. Top left is `(-0.5, 0.5)`, center is `(0.0, 0.0)`. The value will
            text_anchor: Anchor::Custom(Vec2::new(0.0, -2.0)),
            ..default()
        })
        // .insert(RigidBody::Fixed)
        .id();

    let sprite = commands
        .spawn(SpriteBundle {
            texture: sprite,
            // transform: transform.clone(), // Transform::from_scale(Vec3::new(1.5, 1.5, 1.5)), //transform.clone(),
            ..default()
        })
        // .insert(Velocity::angular(3.0))
        .id();
        // .insert(trash)
        // .add_child(text);
    commands.spawn(trash.clone())
        .insert(Transform::from_translation(Vec3::new(pos.x, pos.y, 0.0)))
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(trash.size.x, trash.size.y))
        .insert(ColliderMassProperties::Mass(1.0))
        .add_child(sprite)
        .add_child(text);
}


fn get_trash_sprite(trash_type: &TrashType, textures: &Res<TextureAssets>) -> Handle<Image> {
    match trash_type {
        TrashType::Bottle => textures.bottle.clone(),
        TrashType::Pizza => textures.pizza.clone(),
    }
}
//
// fn move_player(
//     time: Res<Time>,
//     actions: Res<Actions>,
//     mut player_query: Query<&mut Transform, With<Player>>,
// ) {
//     if actions.player_movement.is_none() {
//         return;
//     }
//     let speed = 150.;
//     let movement = Vec3::new(
//         actions.player_movement.unwrap().x * speed * time.delta_seconds(),
//         actions.player_movement.unwrap().y * speed * time.delta_seconds(),
//         0.,
//     );
//     for mut player_transform in &mut player_query {
//         player_transform.translation += movement;
//     }
// }

fn setup(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    window: Query<&Window>,
    typing_buffer: Res<TypingBuffer>,
) {
    commands
        .spawn((
            TextBundle::from_section(
                typing_buffer.0.clone(),
                TextStyle {
                    font_size: 50.0,
                    ..default()
                }
            )
            .with_text_alignment(TextAlignment::Center)
            .with_style(Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(5.0),
                    // right: Val::Px(5.0),
                    ..default()
                }),
            BufferText,
            )
        );


    let window = window.single();
    let max_x: f32 = window.width() / 2.0;
    let max_y = window.height() / 2.0;

    create_borders(&mut commands, &textures, max_x, max_y)
}


fn create_borders(commands: &mut Commands, textures: &Res<TextureAssets>, max_x: f32, max_y: f32) {
    let y_pos = (max_y * -1.0) + 16.0;

    for x in (0..=max_x as u32).step_by(48) {
        println!("Spawning x: {} y: {}", x, max_y);
        commands.spawn(
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(48.0, 48.0)),
                    ..default()
                },
                texture: textures.ground.clone(),
                transform: Transform::from_translation(Vec3::new(x as f32, y_pos, 0.0)),
                ..Default::default()
            })
            .insert(RigidBody::Fixed)
            .insert(Collider::cuboid(32.0, 32.0)
            // .insert(ColliderMassProperties::Mass(1.0)
        );

        commands.spawn(
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(48.0, 48.0)),
                    ..default()
                },
                texture: textures.ground.clone(),
                transform: Transform::from_translation(Vec3::new(x as f32 * -1.0, y_pos, 0.0)),
                ..Default::default()
            })
            .insert(RigidBody::Fixed)
            .insert(Collider::cuboid(32.0, 32.0)
            // .insert(ColliderMassProperties::Mass(1.0)
        );
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

fn typing(
    mut typing_buffer: ResMut<TypingBuffer>,
    keyboard_input: Res<Input<KeyCode>>,
) {

    if keyboard_input.just_pressed(KeyCode::Back) || keyboard_input.pressed(KeyCode::Back) {
        typing_buffer.0.pop();
    }


    for key in keyboard_input.get_just_pressed() {
        match key {
            KeyCode::A => typing_buffer.0.push('a'),
            KeyCode::B => typing_buffer.0.push('b'),
            KeyCode::C => typing_buffer.0.push('c'),
            KeyCode::D => typing_buffer.0.push('d'),
            KeyCode::E => typing_buffer.0.push('e'),
            KeyCode::F => typing_buffer.0.push('f'),
            KeyCode::G => typing_buffer.0.push('g'),
            KeyCode::H => typing_buffer.0.push('h'),
            KeyCode::I => typing_buffer.0.push('i'),
            KeyCode::J => typing_buffer.0.push('j'),
            KeyCode::K => typing_buffer.0.push('k'),
            KeyCode::L => typing_buffer.0.push('l'),
            KeyCode::M => typing_buffer.0.push('m'),
            KeyCode::N => typing_buffer.0.push('n'),
            KeyCode::O => typing_buffer.0.push('o'),
            KeyCode::P => typing_buffer.0.push('p'),
            KeyCode::Q => typing_buffer.0.push('q'),
            KeyCode::R => typing_buffer.0.push('r'),
            KeyCode::S => typing_buffer.0.push('s'),
            KeyCode::T => typing_buffer.0.push('t'),
            KeyCode::U => typing_buffer.0.push('u'),
            KeyCode::V => typing_buffer.0.push('v'),
            KeyCode::W => typing_buffer.0.push('w'),
            KeyCode::X => typing_buffer.0.push('x'),
            KeyCode::Y => typing_buffer.0.push('y'),
            KeyCode::Z => typing_buffer.0.push('z'),
            _ => {}
        }
    }
}


fn destroy_matching_trash(
    mut commands: Commands,
    trash_query: Query<(Entity, &Trash)>,
    mut typing_buffer: ResMut<TypingBuffer>,
) {
    let mut trash_to_destroy: Vec<Entity> = Vec::new();
    for (entity, trash) in &mut trash_query.iter() {
        if typing_buffer.0 == trash.word {
            trash_to_destroy.push(entity);
        }
    }
    for entity in &trash_to_destroy {
        commands.entity(*entity).despawn_recursive();
    }

    if trash_to_destroy.len() > 0 {
        typing_buffer.0 = "".to_string();
    }
}


fn get_random_word(available_words: &Res<AvailableWords>) -> String {
    let mut random = rand::thread_rng();
    let mut random_word: String = String::new();
    let mut random_letter: String = String::new();
    let mut random_words: Vec<String> = Vec::new();
    while random_words.len() == 0 {
        random_letter = random.gen_range('a' .. 'z').to_string();
        random_words = available_words.0.get(random_letter.as_str()).unwrap().clone();
    }
    random_word = random_words[random.gen_range(0..random_words.len())].clone();
    return random_word;
}


fn get_available_words_from_file() -> HashMap<String, Vec<String>> {
    let words: Vec<&str> = include_str!("../assets/words.txt").split("\n").collect();

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
