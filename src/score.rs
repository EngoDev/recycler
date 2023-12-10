use bevy::prelude::*;
use bevy_progressbar::{ProgressBarMaterial, ProgressBar, ProgressBarBundle};

use crate::GameState;

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
        .add_systems(Update, (update_score, update_modifier));
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
    mut combo_modifier: ResMut<ComboModifier>,
    mut combo_meter_query: Query<&mut ProgressBar, With<ComboMeter>>,
) {
    for mut progress_bar in &mut combo_meter_query.iter_mut() {
        if progress_bar.is_finished() {
            combo_modifier.0 += 1;
            progress_bar.reset();
        }
    }
}

