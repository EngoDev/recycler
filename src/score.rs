use bevy::prelude::*;
use bevy_progressbar::{ProgressBarMaterial, ProgressBar, ProgressBarBundle};

use crate::GameState;
use crate::game::update_on_wrong_letter;
use crate::trash::{TrashMarked, handle_trash_collision};
use crate::typing::TypingBuffer;

pub struct ScorePlugin;


#[derive(Component)]
pub struct ScoreMarker;

#[derive(Component)]
pub struct ComboMeter;

#[derive(Resource)]
pub struct Score(pub usize);

#[derive(Resource)]
pub struct ComboModifier(pub usize);

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score(0))
        .insert_resource(ComboModifier(1))
        // .insert_resource(ProgressBarMaterial)
        .add_systems(OnEnter(GameState::Playing), setup)
        .add_systems(Update, (
                update_score,
                update_modifier.after(update_on_wrong_letter)
            ));
        //     .add_systems(Update, move_player.run_if(in_state(GameState::Playing)));
    }
}


fn setup(
    mut commands: Commands,
    mut score: ResMut<Score>,
    mut combo_modifier: ResMut<ComboModifier>,
    materials: ResMut<Assets<ProgressBarMaterial>>,
    // score: Res<Score>,
    // combo_modifier: Res<ComboModifier>,
) {
    score.0 = 0;
    combo_modifier.0 = 1;

    commands.spawn(TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: format!("Score: "),
                    style: TextStyle {
                        font_size: 40.0,
                        color: Color::WHITE,
                        ..default()
                    },
                },
                TextSection {
                    value: format!("{}", score.0),
                    style: TextStyle {
                        font_size: 40.0,
                        color: Color::RED,
                        ..default()
                    },
                },
                TextSection {
                    value: format!(" Combo: "),
                    style: TextStyle {
                        font_size: 40.0,
                        color: Color::WHITE,
                        ..default()
                    },
                },
                TextSection {
                    value: format!("{}", combo_modifier.0),
                    style: TextStyle {
                        font_size: 40.0,
                        color: Color::BLUE,
                        ..default()
                    },
                },
            ],
            ..default()
        },
        ..default()
    })
    .insert(ScoreMarker);

    create_combo_progress_bar(commands, materials);
}

fn create_combo_progress_bar(
    mut commands: Commands,
    mut materias: ResMut<Assets<ProgressBarMaterial>>
) {
    let bar = ProgressBar::single(Color::CYAN);
    let style = Style {
        position_type: PositionType::Absolute,
        width: Val::Percent(100.0),
        height: Val::Px(10.0),
        bottom: Val::Px(0.0),
        // top: Val::Px(400.0),
        // right: Val::Percent(50.0),
        ..default()
    };

    commands.spawn(
        ProgressBarBundle::new(style, bar, &mut materias)
    ).insert(ComboMeter);
}


fn update_score(
    score: Res<Score>,
    combo_modifier: Res<ComboModifier>,
    mut query: Query<&mut Text, With<ScoreMarker>>,
) {
    if !score.is_changed() && !combo_modifier.is_changed() {
        return;
    }

    for mut text in &mut query.iter_mut() {
        text.sections[1].value = format!("{}", score.0);
        text.sections[3].value = format!("{}", combo_modifier.0);
    }
}

fn update_modifier(
    typing_buffer: Res<TypingBuffer>,
    mut combo_modifier: ResMut<ComboModifier>,
    mut combo_meter_query: Query<&mut ProgressBar, With<ComboMeter>>,
    marked_trash_query: Query<Entity, With<TrashMarked>>,
) {
    if !typing_buffer.is_changed() {
        return;
    }

    for mut progress_bar in &mut combo_meter_query.iter_mut() {
        if progress_bar.is_finished() {
            combo_modifier.0 += 1;
            progress_bar.reset();
        } else if !marked_trash_query.is_empty() {
            progress_bar.increase_progress(0.1);
        }
    }
}

// fn update_combo_meter(
//     mut commands: Commands,
//     typing_buffer: Res<TypingBuffer>,
//     mut combo_meter_query: Query<&mut ProgressBar, With<ComboMeter>>,
//     mut combo_modifier: ResMut<ComboModifier>,
//     marked_trash_query: Query<Entity, With<TrashMarked>>,
// ) {
//     if !typing_buffer.is_changed() {
//         return;
//     }
//
//     // if typing_buffer.0.len() == 0 {
//     //     return;
//     // }
//
//     if !marked_trash_query.is_empty() {
//         for mut progress_bar in &mut combo_meter_query.iter_mut() {
//             progress_bar.increase_progress(0.1);
//             // if progress_bar.is_finished() {
//             //     combo_modifier.0 += 1;
//             //     progress_bar.reset();
//             // }
//         }
//     }
// }
//
