use bevy::prelude::*;

use crate::GameState;

pub struct ScorePlugin;


#[derive(Component)]
pub struct ScoreMarker;

#[derive(Resource)]
pub struct Score(pub usize);

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score(0))
        .add_systems(OnEnter(GameState::Playing), setup)
        .add_systems(Update, update_score);
        //     .add_systems(Update, move_player.run_if(in_state(GameState::Playing)));
    }
}


fn setup(
    mut commands: Commands,
    score: Res<Score>,
) {
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
            ],
            ..default()
        },
        ..default()
    })
    .insert(ScoreMarker);
}


fn update_score(
    score: Res<Score>,
    mut query: Query<&mut Text, With<ScoreMarker>>,
) {
    for mut text in &mut query.iter_mut() {
        text.sections[1].value = format!("{}", score.0);
    }
}
