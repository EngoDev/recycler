use crate::GameState;
use crate::trash_text::TrashText;
use bevy::prelude::*;

pub struct TypingPlugin;

#[derive(Resource)]
pub struct TypingBuffer(pub String);

impl Plugin for TypingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TypingBuffer("".to_string()))
        .add_systems(Update, (
                typing.run_if(in_state(GameState::Playing)),
            ));
        // app.insert_resource(TrashSpawnTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        // app.add_systems(OnEnter(GameState::Playing), spawn_player)
        // .add_systems(Update, spawn_trash.run_if(in_state(GameState::Playing)));
    }
}


pub fn typing(
    // mut commands: Commands,
    mut typing_buffer: ResMut<TypingBuffer>,
    // mut combo_meter_query: Query<&mut ProgressBar, With<ComboMeter>>,
    // mut combo_modifier: ResMut<ComboModifier>,
    keyboard_input: Res<Input<KeyCode>>,
    trash_query: Query<(&Parent, &TrashText)>,
    // marked_trash_query: Query<Entity, With<TrashMarked>>,
) {

    if keyboard_input.pressed(KeyCode::ControlLeft) {
        if keyboard_input.just_pressed(KeyCode::Back) {
            typing_buffer.0 = "".to_string();
            // remove_all_marked_trash(commands, marked_trash_query);
            return
        }
    }

    let mut buffer_word = typing_buffer.0.clone();
    // let mut did_delete_letter = false;

    for key in keyboard_input.get_just_pressed() {
        match key {
            KeyCode::A => buffer_word.push('a'),
            KeyCode::B => buffer_word.push('b'),
            KeyCode::C => buffer_word.push('c'),
            KeyCode::D => buffer_word.push('d'),
            KeyCode::E => buffer_word.push('e'),
            KeyCode::F => buffer_word.push('f'),
            KeyCode::G => buffer_word.push('g'),
            KeyCode::H => buffer_word.push('h'),
            KeyCode::I => buffer_word.push('i'),
            KeyCode::J => buffer_word.push('j'),
            KeyCode::K => buffer_word.push('k'),
            KeyCode::L => buffer_word.push('l'),
            KeyCode::M => buffer_word.push('m'),
            KeyCode::N => buffer_word.push('n'),
            KeyCode::O => buffer_word.push('o'),
            KeyCode::P => buffer_word.push('p'),
            KeyCode::Q => buffer_word.push('q'),
            KeyCode::R => buffer_word.push('r'),
            KeyCode::S => buffer_word.push('s'),
            KeyCode::T => buffer_word.push('t'),
            KeyCode::U => buffer_word.push('u'),
            KeyCode::V => buffer_word.push('v'),
            KeyCode::W => buffer_word.push('w'),
            KeyCode::X => buffer_word.push('x'),
            KeyCode::Y => buffer_word.push('y'),
            KeyCode::Z => buffer_word.push('z'),
            KeyCode::Back => { 
                let _ = buffer_word.pop(); 
                // did_delete_letter = true;
            },
            _ => {}
        }
    }

    if buffer_word != typing_buffer.0 {
        typing_buffer.0 = buffer_word;
        // return;
    }


    // let mut to_be_removed = vec![];
    // let mut is_existing_matching_word = false;

    // for (_, trash_text) in trash_query.iter() {
    //     if trash_text.word.starts_with(&buffer_word) {
    //         // if marked_trash_query.get(parent.get()).is_err() {
    //             // commands.entity(parent.get()).insert(TrashMarked);
    //         // }
    //         typing_buffer.0 = buffer_word.clone();
    //         return;
    //     }

        // } else {
        //     if marked_trash_query.get(parent.get()).is_ok() {
        //         to_be_removed.push(parent.get());
        //     }
        // }
    // }

    // TODO: Make the combo meter reset only for words that the player tried to type and then
    // failed, not for every trash that lands. It's too hard

    // if is_existing_matching_word {
    //     typing_buffer.0 = buffer_word.clone();
    //     if !did_delete_letter {
    //         for mut progress_bar in &mut combo_meter_query.iter_mut() {
    //             progress_bar.increase_progress(0.1);
    //         }
    //     }
    //
    //     for entity in to_be_removed {
    //         commands.entity(entity).remove::<TrashMarked>();
    //     }
    //
    // } else {
    //     for mut progress_bar in &mut combo_meter_query.iter_mut() {
    //         progress_bar.reset();
    //         combo_modifier.0 = 1;
    //    }
    //
    // }

}
