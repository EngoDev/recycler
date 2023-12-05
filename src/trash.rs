use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

pub struct TrashPlugin;

pub enum TrashType {
    Bottle,
    Pizza
}


#[derive(Component)]
pub struct Trash {
    pub trash_type: TrashType,
}

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for TrashPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(OnEnter(GameState::Playing), spawn_player)
        app.add_systems(Update, spawn_trash.run_if(in_state(GameState::Playing)));
    }
}


fn spawn_trash(mut commands: Commands, textures: Res<TextureAssets>, keyboard_input: Res<Input<KeyCode>>, window: Query<&Window>) {
    //     match self {
    //         GameControl::Up => {
    //             keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up)
    //         }
    //         GameControl::Down => {
    //             keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down)
    //         }
    //         GameControl::Left => {
    //             keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left)
    //         }
    //         GameControl::Right => {
    //             keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right)
    //         }
    //     }
    // }

    let window = window.single();

    let mut random = rand::thread_rng();
    let max_x: f32 = window.width() / 2.0;
    let y_pos = window.height() / 2.0;

    let random_x: f32 = random.gen_range(-max_x .. max_x);

    match keyboard_input.pressed(KeyCode::B) {
        true => {
            let trash = Trash {
                trash_type: TrashType::Bottle
            };
            create_trash(&mut commands, &textures, trash, Vec2::new(random_x as f32, y_pos));
        }
        false => {}
    }

    match keyboard_input.pressed(KeyCode::P) {
        true => {
            let trash = Trash {
                trash_type: TrashType::Pizza
            };
            create_trash(&mut commands, &textures, trash, Vec2::new(random_x as f32, y_pos));
        }
        false => {}
    }

}


pub fn create_trash(commands: &mut Commands, textures: &Res<TextureAssets>, trash: Trash, pos: Vec2) {
    let sprite = get_trash_sprite(&trash.trash_type, textures);

    commands
        .spawn(SpriteBundle {
            texture: sprite,
            transform: Transform::from_translation(Vec3::new(pos.x, pos.y, 1.)),
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(0.5, 0.5))
        .insert(ColliderMassProperties::Mass(1.0))
        // .insert(GravityScale(1.0))
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
