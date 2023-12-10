use bevy::prelude::*;
use bevy::sprite::Anchor;


// pub struct TrashTextPlugin;

#[derive(Component, Default)]
pub struct TrashText {
    pub word: String,
    pub highlight_color: Color,
    pub color: Color,
    // pub ui: Text2dBundle,
}

#[derive(Bundle, Default)]
pub struct TrashTextBundle {
    pub trash_text: TrashText,
    pub ui: Text2dBundle,
}


// impl TrashText {
//     pub fn new(text: String, highlight_color: Color) -> Self {
//         Self {
//             characters: text.chars().map(|c| c.to_string()).collect(),
//             highlight_color
//         }
//     }
// }


// impl Plugin for TrashTextPlugin {
//     fn build(&self, app: &mut App) {
//         // app.insert_resource(TrashSpawnTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
//         // .insert_resource(TypingBuffer("".to_string()))
//         // .insert_resource(AvailableWords(get_available_words_from_file()))
//         // .add_systems(OnEnter(GameState::Playing), setup)
//         // .add_systems(Update, (
//         //         spawn_trash.run_if(in_state(GameState::Playing)),
//         //         typing.run_if(in_state(GameState::Playing)),
//         //         destroy_matching_trash.after(typing),
//         //         update_buffer_text.after(typing),
//         //         highlight_character.after(update_buffer_text),
//         //     )
//         // )
//         // .add_systems(PostStartup, fix_trash_label_rotation.before(TransformSystem::TransformPropagate))
//         // .add_systems(PostUpdate, fix_trash_label_rotation.before(TransformSystem::TransformPropagate));
//
//         app.add_systems(schedule, systems)
//     }
// }


pub fn highlight_characters(sections: &Vec<TextSection>, max_character_index: usize, highlight_color: Color) -> Vec<TextSection> {
    let mut sections = sections.clone();
    for i in 0..max_character_index {
        let section = sections.get_mut(i).unwrap();
        section.style.color = highlight_color;
    }

    for i in max_character_index..sections.len() {
        let section = sections.get_mut(i).unwrap();
        section.style.color = Color::WHITE;
    }
    // let section = sections.get_mut(character_index).unwrap();
    // section.style.color = self.highlight_color;
    sections
}

pub fn remove_highlight(sections: &Vec<TextSection>, color: Color) -> Vec<TextSection> {
    let mut sections = sections.clone();
    for section in sections.iter_mut() {
        section.style.color = color;
    }

    sections
}

impl TrashTextBundle {
    pub fn new(text: String, anchor: Anchor, highlight_color: Color, style: TextStyle) -> Self {
        let sections = Self::create_sections_from_text(&text, &style);

        Self {
            trash_text: TrashText {
                word: text.clone(),
                highlight_color,
                color: style.color,
            },
            // highlight_color,
            // color: style.color,
                // TrashText::new(text.clone(), highlight_color),
            ui: Text2dBundle {
                text: Text::from_sections(sections),
                text_anchor: anchor,
                // transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
                ..default()
            },
        }
    }

    fn create_sections_from_text(text: &String, style: &TextStyle) -> Vec<TextSection> {
        let mut sections = Vec::new();

        for character in text.chars() {
            sections.push(TextSection {
                value: character.to_string(),
                style: style.clone()
            })
        }

        sections
    }

    // pub fn highlight_characters(&mut self, max_character_index: usize) {
    //     let mut sections = self.ui.text.sections.clone();
    //     for i in 0..max_character_index {
    //         let section = sections.get_mut(i).unwrap();
    //         section.style.color = self.highlight_color;
    //     }
    //     // let section = sections.get_mut(character_index).unwrap();
    //     // section.style.color = self.highlight_color;
    //     self.ui.text.sections = sections;
    // }
    //
    // pub fn remove_highlight(&mut self) {
    //     let mut sections = self.ui.text.sections.clone();
    //     for section in sections.iter_mut() {
    //         section.style.color = self.color;
    //     }
    //
    //     self.ui.text.sections = sections;
    // }
}
