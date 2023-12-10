use crate::actions::Actions;
use crate::loading::AudioAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

pub struct InternalAudioPlugin;

// This plugin is responsible to control the game audio
impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioPlugin)
            .add_systems(OnEnter(GameState::Menu), create_audio)
            .add_systems(OnEnter(GameState::Playing), start_game_audio)
            .add_systems(OnEnter(GameState::GameOver), start_game_over_audio);
            // .add_systems(
            //     Update,
            //     control_flying_sound
            //         .after(set_movement_actions)
            //         .run_if(in_state(GameState::Playing)),
            // );
    }
}

#[derive(Resource)]
struct FlyingAudio(Handle<AudioInstance>);

#[derive(Resource)]
struct GameAudio(Handle<AudioInstance>);

#[derive(Resource)]
struct GameOverAudio(Handle<AudioInstance>);

fn create_audio(mut commands: Commands, audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
    audio.pause();
    // let handle = audio
    //     .play(audio_assets.flying.clone())
    //     .looped()
    //     .with_volume(0.3)
    //     .handle();
    // commands.insert_resource(FlyingAudio(handle));

    let handle = audio
        .play(audio_assets.play.clone())
        .looped()
        .with_volume(0.3)
        .handle();
    commands.insert_resource(GameAudio(handle));

    audio.pause();
    let handle = audio
        .play(audio_assets.game_over.clone())
        .with_volume(0.3)
        .handle();
    commands.insert_resource(GameOverAudio(handle));
}

fn start_game_audio(
    game_audio: Res<GameAudio>,
    game_over_audio: Res<GameOverAudio>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    if let Some(instance) = audio_instances.get_mut(&game_over_audio.0) {
        instance.pause(AudioTween::default());
    }

    if let Some(instance) = audio_instances.get_mut(&game_audio.0) {
        instance.seek_to(0.0);
        instance.resume(AudioTween::default());
    }
}

fn start_game_over_audio(
    game_audio: Res<GameAudio>,
    game_over_audio: Res<GameOverAudio>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    if let Some(instance) = audio_instances.get_mut(&game_audio.0) {
        instance.pause(AudioTween::default());
    }

    if let Some(instance) = audio_instances.get_mut(&game_over_audio.0) {
        instance.seek_to(0.0);
        instance.resume(AudioTween::default());
    }
    // let handle = audio
    //     .play(audio_assets.game_over.clone())
    //     .with_volume(0.3);

}


#[allow(dead_code)]
fn control_flying_sound(
    actions: Res<Actions>,
    audio: Res<FlyingAudio>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    if let Some(instance) = audio_instances.get_mut(&audio.0) {
        match instance.state() {
            PlaybackState::Paused { .. } => {
                if actions.player_movement.is_some() {
                    instance.resume(AudioTween::default());
                }
            }
            PlaybackState::Playing { .. } => {
                if actions.player_movement.is_none() {
                    instance.pause(AudioTween::default());
                }
            }
            _ => {}
        }
    }
}
