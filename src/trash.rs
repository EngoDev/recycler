use crate::loading::TextureAssets;
use crate::GameState;
use crate::trash_text::{TrashText, TrashTextBundle, highlight_characters, remove_highlight};
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::transform::TransformSystem;
use bevy::utils::HashMap;
use bevy_rapier2d::prelude::*;
use rand::Rng;

const BORDER_TILE_SIZE: f32 = 48.0;
const BORDER_TILE_SCALE: Vec2 = Vec2::new(BORDER_TILE_SIZE, BORDER_TILE_SIZE);


pub struct TrashPlugin;

#[derive(Clone, Debug)]
pub enum TrashType {
    Bottle,
    Pizza
}


#[derive(Component, Debug, Clone)]
pub struct Trash {
    pub trash_type: TrashType,
    pub size: Vec2,
}

#[derive(Component, Debug, Clone)]
pub struct BufferText;


#[derive(Component)]
pub struct TrashActive;

#[derive(Component)]
pub struct TrashMarked;

#[derive(Component)]
pub struct Wall;

// #[derive(Component, Debug, Clone)]
// pub struct TrashLabel;

#[derive(Resource)]
struct TrashSpawnTimer(Timer);

#[derive(Resource)]
struct BufferTextDeleteTimer(Timer);

#[derive(Resource)]
pub struct TypingBuffer(String);

#[derive(Resource)]
pub struct AvailableWords(HashMap<String, Vec<String>>);

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for TrashPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TrashSpawnTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        // .insert_resource(BufferTextDeleteTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .insert_resource(TypingBuffer("".to_string()))
        .insert_resource(AvailableWords(get_available_words_from_file()))
        .add_systems(OnEnter(GameState::Playing), setup)
        .add_systems(Update, (
                spawn_trash.run_if(in_state(GameState::Playing)),
                handle_trash_collision.before(typing),
                typing.after(handle_trash_collision),
                destroy_matching_trash.after(typing),
                update_buffer_text.after(typing),
                highlight_character.after(typing),
            )
        )
        .add_systems(PostStartup, fix_trash_label_rotation.before(TransformSystem::TransformPropagate))
        .add_systems(PostUpdate, fix_trash_label_rotation.before(TransformSystem::TransformPropagate));
    }
}

impl Trash {
    pub fn get_by_type(trash_type: TrashType) -> Self {
        match trash_type {
            TrashType::Bottle => Self::bottle(),
            TrashType::Pizza => Self::pizza(),
        }
    }

    pub fn bottle() -> Self {
        Self {
            trash_type: TrashType::Bottle,
            size: Vec2::new(15.0, 16.0),
        }
    }

    pub fn pizza() -> Self {
        Self {
            trash_type: TrashType::Pizza,
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

        let random_x: f32 = random.gen_range(-max_x + BORDER_TILE_SIZE .. max_x - BORDER_TILE_SIZE);
        println!("Random x: {}", random_x);
        let trash_type: TrashType = SPAWN_CHANCES[random.gen_range(0..SPAWN_CHANCES.len())].clone();

        // TODO: make sure the same word doesn't appear twice in a row
        // A solution might be to have search to search for a word as long as it's not in a list of
        // already used words which we can get from a query
        // Also we need to be able to to limit the amount of letters in a word
        let trash = Trash::get_by_type(trash_type);
        //     trash_type,
        //     word: 
        // };

        println!("Spawning trash: {:?}", trash);
        create_trash(&mut commands, &textures, trash, get_random_word(&available_words), Vec2::new(random_x as f32, y_pos));
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


pub fn create_trash(commands: &mut Commands, textures: &Res<TextureAssets>, trash: Trash, word: String, pos: Vec2) {
    let sprite = get_trash_sprite(&trash.trash_type, textures);
    let mut transform = Transform::from_translation(Vec3::new(pos.x, pos.y, 0.0));
    transform.scale = Vec3::new(1.5, 1.5, 1.5);
    transform.rotation = Quat::from_rotation_y(30.0);

    // let text = commands.spawn(Text2dBundle {
    //         text: Text::from_section(trash.word.clone(), TextStyle {
    //             color: Color::WHITE,
    //             font_size: 20.0,
    //             ..default()
    //
    //         }),//.with_alignment(TextAlignment::Center),
    //         // transform: Transform::from_translation(Vec3::new(pos.x, pos.y, 1.0)),
    //         // transform: transform.clone(),
    //         // Custom anchor point. Top left is `(-0.5, 0.5)`, center is `(0.0, 0.0)`. The value will
    //         text_anchor: Anchor::Custom(Vec2::new(0.0, -2.0)),
    //         ..default()
    //     })
    //     // .insert(RigidBody::Fixed)
    //     .id();

    commands
        .spawn(SpriteBundle {
            texture: sprite,
            // transform: transform.clone(), // Transform::from_scale(Vec3::new(1.5, 1.5, 1.5)), //transform.clone(),
            ..default()
        })
        // .insert(Velocity::angular(3.0))
        .insert(Transform::from_translation(Vec3::new(pos.x, pos.y, 0.0)))
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(trash.size.x, trash.size.y))
        .insert(ColliderMassProperties::Mass(1.0))
        .insert(Restitution::coefficient(0.7))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(TrashActive)
        .insert(trash.clone())
        .with_children(|parent| {
            parent.spawn(
                TrashTextBundle::new(
                    word,
                    Anchor::Custom(Vec2::new(0.0, -2.0)),
                    Color::GREEN,
                    TextStyle {
                        color: Color::WHITE,
                        font_size: 20.0,
                        ..default()
                    }
                )
            );
                // Text2dBundle {
                // text: Text::from_section(trash.word.clone(), TextStyle {
                //     color: Color::WHITE,
                //     font_size: 20.0,
                //     ..default()
                //
                // }),//.with_alignment(TextAlignment::Center),
                // // transform: Transform::from_translation(Vec3::new(pos.x, pos.y, 1.0)),
                // // transform: transform.clone(),
                // // Custom anchor point. Top left is `(-0.5, 0.5)`, center is `(0.0, 0.0)`. The value will
                // text_anchor: Anchor::Custom(Vec2::new(0.0, -2.0)),
                // ..default()
            // })
            // .insert(TrashLabel));
        });
        // .id();
        // .insert(trash)
        // .add_child(text);
    // commands.spawn(trash.clone())
    //     .insert(Transform::from_translation(Vec3::new(pos.x, pos.y, 0.0)))
    //     .insert(RigidBody::Dynamic)
    //     .insert(Collider::cuboid(trash.size.x, trash.size.y))
    //     .insert(ColliderMassProperties::Mass(1.0))
    //     .add_child(sprite)
    //     .add_child(text);
}


fn get_trash_sprite(trash_type: &TrashType, textures: &Res<TextureAssets>) -> Handle<Image> {
    match trash_type {
        TrashType::Bottle => textures.bottle.clone(),
        TrashType::Pizza => textures.pizza.clone(),
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

    commands.spawn((
        TextBundle::from_section(
            typing_buffer.0.clone(),
            TextStyle {
                font_size: 50.0,
                ..default()
            }
        )
        // .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
                // align_self: AlignSelf::FlexEnd,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                top: Val::Percent(50.0),
                left: Val::Percent(50.0),
                max_width: Val::Px(200.0),
                max_height: Val::Percent(100.0),
                flex_wrap: FlexWrap::WrapReverse,
                // flex_wrap: FlexWrap::Wrap,
                // bottom: Val::Px(5.0),
                // right: Val::Px(5.0),
                ..default()
            }),
        BufferText,
        )
    );

    // });


    let window = window.single();
    let max_x: f32 = window.width() / 2.0;
    let max_y = window.height() / 2.0;

    create_borders(&mut commands, &textures, max_x, max_y)
}


fn create_borders(commands: &mut Commands, textures: &Res<TextureAssets>, max_x: f32, max_y: f32) {
    let y_pos = (max_y * -1.0) + 16.0;
    let x_pos = (max_x * -1.0) + 16.0;
    let iterations_x = (max_x % BORDER_TILE_SIZE) + max_x;
    let iterations_y = (max_y % BORDER_TILE_SIZE) + max_y;

    for x in (0..=iterations_x as u32).step_by(BORDER_TILE_SIZE as usize) {
        commands.spawn(
            get_border_tile(Vec3::new(x as f32, y_pos, 1.0), textures.ground.clone(), BORDER_TILE_SCALE.clone())
        ).insert(Collider::cuboid(BORDER_TILE_SIZE / 2.0, BORDER_TILE_SIZE / 2.0));

        commands.spawn(
            get_border_tile(Vec3::new(x as f32 * -1.0, y_pos, 1.0), textures.ground.clone(), BORDER_TILE_SCALE.clone())
        ).insert(Collider::cuboid(BORDER_TILE_SIZE / 2.0, BORDER_TILE_SIZE / 2.0));
    }

    for y in (0..=iterations_y as u32).step_by(BORDER_TILE_SIZE as usize) {
        commands.spawn(
            get_border_tile(Vec3::new(x_pos, y as f32, 0.0), textures.wall.clone(), BORDER_TILE_SCALE.clone())
        )
        .insert(Collider::cuboid(BORDER_TILE_SIZE / 2.0, BORDER_TILE_SIZE / 2.0))
        .insert(Wall);

        commands.spawn(
            get_border_tile(Vec3::new(-x_pos, y as f32, 0.0), textures.wall.clone(), BORDER_TILE_SCALE.clone())
        )
        .insert(Collider::cuboid(BORDER_TILE_SIZE / 2.0, BORDER_TILE_SIZE / 2.0))
        .insert(Wall);

        commands.spawn(
            get_border_tile(Vec3::new(x_pos, y as f32 * -1.0, 0.0), textures.wall.clone(), BORDER_TILE_SCALE.clone())
        )
        .insert(Collider::cuboid(BORDER_TILE_SIZE / 2.0, BORDER_TILE_SIZE / 2.0))
        .insert(Wall);

        commands.spawn(
            get_border_tile(Vec3::new(-x_pos, y as f32 * -1.0, 0.0), textures.wall.clone(), BORDER_TILE_SCALE.clone())
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
    query_parents: Query<&Transform, (With<Trash>, Without<TrashText>, With<TrashActive>)>,
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

fn typing(
    mut commands: Commands,
    mut typing_buffer: ResMut<TypingBuffer>,
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

    // if keyboard_input.just_pressed(KeyCode::Back) {
    //     typing_buffer.0.pop();
    // }


    let mut buffer_word = typing_buffer.0.clone();

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
            KeyCode::Back => { let _ = buffer_word.pop(); },
            _ => {}
        }
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

    if is_existing_matching_word {
        typing_buffer.0 = buffer_word.clone();
    }

    for entity in to_be_removed {
        commands.entity(entity).remove::<TrashMarked>();
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

fn remove_trash_text(commands: &mut Commands, trash_entity: &Entity) {
    commands.entity(*trash_entity).remove::<TrashActive>();
    commands.entity(*trash_entity).remove::<TrashMarked>();
    commands.entity(*trash_entity).despawn_descendants();
}

fn handle_trash_collision(
    mut commands: Commands,
    mut typing_buffer: ResMut<TypingBuffer>,
    mut collision_events: EventReader<CollisionEvent>,
    mut trash_query: Query<Entity, With<TrashActive>>,
    walls_query: Query<Entity, With<Wall>>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity1, entity2, _) => {
                if let Ok(trash_entity) = trash_query.get_mut(*entity1) {
                    match trash_query.get_mut(*entity2) {
                        Ok(_) => (),
                        Err(_) => {
                            match walls_query.get(*entity2) {
                                Ok(_) => (),
                                Err(_) => {
                                    remove_trash_text(&mut commands, &trash_entity);
                                    typing_buffer.0 = "".to_string();
                                },
                            }
                        }
                    };
                } else if let Ok(trash_entity) = trash_query.get_mut(*entity2) {
                    match walls_query.get(*entity1) {
                        Ok(_) => (),
                        Err(_) => {
                            remove_trash_text(&mut commands, &trash_entity);
                            typing_buffer.0 = "".to_string();
                        },
                    }
                }
            }
            CollisionEvent::Stopped(_entity1, _entity2, _) => {},
        }
    }
}


fn destroy_matching_trash(
    mut commands: Commands,
    trash_query: Query<(&Parent, &TrashText)>,
    mut typing_buffer: ResMut<TypingBuffer>,
) {

    if !typing_buffer.is_changed() {
        return;
    }

    let mut trash_to_destroy: Vec<&Parent> = Vec::new();
    for (entity, trash_text) in &mut trash_query.iter() {
        if typing_buffer.0 == trash_text.word {
            trash_to_destroy.push(entity);
        }
    }
    for entity in &trash_to_destroy {
        commands.entity(entity.get()).despawn_recursive();
    }

    if trash_to_destroy.len() > 0 {
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
    let words: Vec<&str> = include_str!("../assets/easy.txt").split("\n").collect();

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
