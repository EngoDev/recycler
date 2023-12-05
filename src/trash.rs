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
}

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
        // app.add_systems(OnEnter(GameState::Playing), spawn_player)
        .add_systems(Update, spawn_trash.run_if(in_state(GameState::Playing)));
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
        let trash = Trash {
            trash_type,
            word: get_random_word(&available_words)
        };

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

    commands
        .spawn(SpriteBundle {
            texture: sprite,
            transform: Transform::from_translation(Vec3::new(pos.x, pos.y, 0.0)),
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(0.5, 0.5))
        .insert(ColliderMassProperties::Mass(1.0))
        .insert(Text2dBundle {
            text: Text::from_section(trash.word.clone(), TextStyle {
                color: Color::WHITE,
                font_size: 20.0,
                ..default()

            }),//.with_alignment(TextAlignment::Center),
            transform: Transform::from_translation(Vec3::new(pos.x, pos.y, 1.0)),
            // Custom anchor point. Top left is `(-0.5, 0.5)`, center is `(0.0, 0.0)`. The value will
            text_anchor: Anchor::Custom(Vec2::new(0.0, -2.0)),
            ..default()
        })
        .insert(trash);
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
    for entity in trash_to_destroy {
        commands.entity(entity).despawn();
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
