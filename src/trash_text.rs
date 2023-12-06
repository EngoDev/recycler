use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::transform::TransformSystem;
use bevy::utils::HashMap;
use bevy_rapier2d::prelude::*;
use rand::Rng;


const FONT_SIZE: f32 = 100.0;


#[derive(Component, Default)]
pub struct TrashText {
    pub characters: Vec<String>,
    pub highlight_color: Color,
}

#[derive(Bundle, Default)]
pub struct TrashTextBundle {
    pub trash_text: TrashText,
    pub ui: Text2dBundle,
}


impl TrashText {
    pub fn new(text: String, highlight_color: Color) -> Self {
        Self {
            characters: text.chars().map(|c| c.to_string()).collect(),
            highlight_color
        }
    }
}

impl TrashTextBundle {
    pub fn new(text: String, anchor: Anchor, highlight_color: Color, style: TextStyle) -> Self {
        let sections = Self::create_sections_from_text(&text, style);

        Self {
            trash_text: TrashText::new(text.clone(), highlight_color),
            ui: Text2dBundle {
                text: Text::from_sections(sections),
                text_anchor: anchor,
                // transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
                ..default()
            },
        }
    }

    fn create_sections_from_text(text: &String, style: TextStyle) -> Vec<TextSection> {
        let mut sections = Vec::new();

        for character in text.chars() {
            sections.push(TextSection {
                value: character.to_string(),
                style: style.clone()
            })
        }

        sections
    }

    pub fn highlight_character(&mut self, character_index: usize) {
        let mut sections = self.ui.text.sections.clone();
        let section = sections.get_mut(character_index).unwrap();
        section.style.color = self.trash_text.highlight_color;
        self.ui.text.sections = sections;
    }
}
