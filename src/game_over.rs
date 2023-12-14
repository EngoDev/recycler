use bevy::prelude::*;
use bevy_rapier2d::dynamics::Velocity;

use crate::GameState;
use crate::menu::{ButtonColors, ChangeState};
use crate::score::Score;
use crate::trash::{Trash, TrashActionActive};


pub struct GameOverPlugin;


#[derive(Component, Default)]
pub struct GameOverLine;

#[derive(Component, Default)]
pub struct GameOver;


impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOver), spawn_game_over_menu)
        .add_systems(OnExit(GameState::GameOver), delete_all_gameover_entities)
        .add_systems(Update, (
                click_restart_button.run_if(in_state(GameState::GameOver)),
            ));
        // app.insert_resource(TypingBuffer("".to_string()))
        // .add_systems(Update, (
        //         typing.run_if(in_state(GameState::Playing)),
        //     ));
        // app.insert_resource(TrashSpawnTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        // app.add_systems(OnEnter(GameState::Playing), spawn_player)
        // .add_systems(Update, spawn_trash.run_if(in_state(GameState::Playing)));
    }
}




pub fn is_game_over(
    entity: &Entity,
    other: &Entity,
    inactive_trash_query: &Query<(Entity, &Velocity), (Without<TrashActionActive>, With<Trash>)>,
    game_over_query: &Query<Entity, With<GameOverLine>>,
) -> bool {
        if game_over_query.get(*entity).is_ok() || game_over_query.get(*other).is_ok() {
            if inactive_trash_query.get(*entity).is_ok() || inactive_trash_query.get(*other).is_ok() {
                return true;
            }
        }

    return false;
}

fn delete_all_gameover_entities(
    mut commands: Commands,
    game_over_entities: Query<Entity, With<GameOver>>,
) {
    for entity in &mut game_over_entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn spawn_game_over_menu(
    mut commands: Commands,
    score: Res<Score>,
) {
    let style = Style {
        position_type: PositionType::Absolute,
        top: Val::Percent(30.0),
        left: Val::Px(120.0),
        ..default()
    };
    commands.spawn(
        TextBundle {
            text: Text::from_section(
                "Game Over".to_string(),
                TextStyle {
                    font_size: 100.0,
                    color: Color::RED,
                    ..default()
                }
            ),
            style,
            ..default()
        },
    )
    .insert(Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)))
    .insert(GameOver);

    commands.spawn(
        TextBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Score: ".to_string(),
                        style: TextStyle {
                            font_size: 50.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    },
                    TextSection {
                        value: score.0.to_string(),
                        style: TextStyle {
                            font_size: 50.0,
                            color: Color::GREEN,
                            ..default()
                        },
                    },
                ],
                ..default()
            },
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(100.0),
                left: Val::Px(120.0),
                ..default()
            },
            ..default()
        },
    )
    .insert(Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)))
    .insert(GameOver);


    let button_colors = ButtonColors::default();
    commands.spawn((
        ButtonBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Percent(50.0),
                left: Val::Px(260.0),
                ..default()
            },
            background_color: button_colors.normal.into(),
            ..default()
        },
        ButtonColors::default(),
        ChangeState(GameState::Playing),
    ))
    .insert(Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)))
    .insert(GameOver)
    .with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "Restart",
            TextStyle {
                font_size: 40.0,
                color: Color::rgb(0.9, 0.9, 0.9),
                ..default()
            },
        ));
    });
}

fn click_restart_button(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (
        &Interaction,
        &mut BackgroundColor,
        &mut ButtonColors,
        Option<&ChangeState>,
    ),
    (Changed<Interaction>, With<Button>)
    >,
) {
    for (interaction, mut color, button_colors, change_state) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if let Some(state) = change_state {
                    next_state.set(state.0.clone());
                } 
            }
            Interaction::Hovered => {
                *color = button_colors.hovered.into();
            }
            Interaction::None => {
                *color = button_colors.normal.into();
            }
        }
    }

}
