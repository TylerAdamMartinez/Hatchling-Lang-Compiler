use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    x: i32,
    y: i32,
    character_id: String,
    order: usize,
}

impl Location {
    pub fn new(x: i32, y: i32, character_id: String, order: usize) -> Self {
        Self {
            x,
            y,
            character_id,
            order,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    message: String,
    character_id: String,
    order: usize,
}

impl Message {
    pub fn new(message: String, character_id: String, order: usize) -> Self {
        Self {
            message,
            character_id,
            order,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Character {
    hair_color: String,
    eye_color: String,
    skin_color: String,
    outfit: String,
    id: String,
}

impl Character {
    pub fn new(hair_color: String, eye_color: String, skin_color: String, outfit: String) -> Self {
        let id = Uuid::new_v4().to_string();
        Self {
            hair_color,
            eye_color,
            skin_color,
            outfit,
            id,
        }
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    characters: Vec<Character>,
    messages: Vec<Message>,
    locations: Vec<Location>,
}

impl Response {
    pub fn new(
        characters: Vec<Character>,
        messages: Vec<Message>,
        locations: Vec<Location>,
    ) -> Self {
        Self {
            characters,
            messages,
            locations,
        }
    }
}
