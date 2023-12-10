use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

/// This plugin loads all assets using [`AssetLoader`] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at <https://bevy-cheatbook.github.io/features/assets.html>
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading).continue_to_state(GameState::Menu),
        )
        .add_collection_to_loading_state::<_, AudioAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, TextureAssets>(GameState::Loading);
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see <https://github.com/NiklasEi/bevy_asset_loader>)

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    // #[asset(path = "audio/flying.ogg")]
    // pub flying: Handle<AudioSource>,
    #[asset(path = "audio/play.ogg")]
    pub play: Handle<AudioSource>,
    #[asset(path = "audio/game_over.ogg")]
    pub game_over: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub bevy: Handle<Image>,
    #[asset(path = "textures/github.png")]
    pub github: Handle<Image>,
    #[asset(path = "textures/bottle.png")]
    pub bottle: Handle<Image>,
    #[asset(path = "textures/pizza.png")]
    pub pizza: Handle<Image>,
    #[asset(path = "textures/big_box.png")]
    pub big_box: Handle<Image>,
    #[asset(path = "textures/glass_bottle.png")]
    pub glass_bottle: Handle<Image>,
    #[asset(path = "textures/news.png")]
    pub news: Handle<Image>,
    #[asset(path = "textures/shampo.png")]
    pub shampoo: Handle<Image>,
    #[asset(path = "textures/small_can.png")]
    pub small_can: Handle<Image>,
    #[asset(path = "textures/soda.png")]
    pub soda: Handle<Image>,
    #[asset(path = "textures/spray.png")]
    pub spray: Handle<Image>,
    #[asset(path = "textures/ground.png")]
    pub ground: Handle<Image>,
    #[asset(path = "textures/wall.png")]
    pub wall: Handle<Image>,
    #[asset(path = "textures/background-1.png")]
    pub background: Handle<Image>,
}
