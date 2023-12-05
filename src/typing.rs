use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

// pub struct TrashPlugin;

// #[derive(Resource)]
// pub struct TypingBuffer(String);

// impl Plugin for TrashPlugin {
//     fn build(&self, app: &mut App) {
//         app.insert_resource(TrashSpawnTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
//         // app.add_systems(OnEnter(GameState::Playing), spawn_player)
//         .add_systems(Update, spawn_trash.run_if(in_state(GameState::Playing)));
//     }
// }
