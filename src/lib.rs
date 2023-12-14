#![allow(clippy::type_complexity)]

mod actions;
mod audio;
mod loading;
mod menu;
mod player;
mod trash;
mod typing;
mod trash_text;
mod score;
mod clone_entity;
mod game;
mod game_over;
// mod consts;

use crate::actions::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
// use crate::player::PlayerPlugin;

use bevy::app::App;

use bevy::prelude::*;
use bevy_progressbar::ProgressBarPlugin;

use self::game::PlayPlugin;
use self::game_over::GameOverPlugin;
use self::score::ScorePlugin;
use self::trash::TrashPlugin;
use self::typing::TypingPlugin;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
    GameOver,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>().add_plugins((
            LoadingPlugin,
            MenuPlugin,
            ActionsPlugin,
            InternalAudioPlugin,
            ProgressBarPlugin,
            TypingPlugin,
            PlayPlugin,
            GameOverPlugin,
            ScorePlugin,
            TrashPlugin,
            // PlayerPlugin,
        ));

        #[cfg(debug_assertions)]
        {
            // app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()));
        }
    }
}
