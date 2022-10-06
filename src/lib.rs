use serde::{Deserialize, Serialize};
use uuid::Uuid;
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    x: i32,
    y: i32,
    character_id: String,
    order: usize,
}

impl Location {
    fn new(x: i32, y: i32, character_id: String, order: usize) -> Self {
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
    fn new(message: String, character_id: String, order: usize) -> Self {
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
    fn new(hair_color: String, eye_color: String, skin_color: String, outfit: String) -> Self {
        let id = Uuid::new_v4().to_string();
        Self {
            hair_color,
            eye_color,
            skin_color,
            outfit,
            id,
        }
    }

    fn get_id(&self) -> String {
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
    fn new(characters: Vec<Character>, messages: Vec<Message>, locations: Vec<Location>) -> Self {
        Self {
            characters,
            messages,
            locations,
        }
    }
}

#[wasm_bindgen]
pub fn moch(buf: String) -> String {
    println!("{}", &buf);

    let mut order: usize = 0;

    let mut characters = Vec::<Character>::new();
    let mut messages = Vec::<Message>::new();
    let mut locations = Vec::<Location>::new();

    let count = rand::random::<usize>() % 10;

    for _ in 0..count {
        characters.push(Character::new(
            "hair".to_owned(),
            "eye".to_owned(),
            "skin".to_owned(),
            "outfit".to_owned(),
        ));
    }

    for i in 0..count {
        messages.push(Message::new(
            "message".to_owned(),
            characters[i].get_id(),
            order,
        ));
        order += 1;
    }

    let mut x: i32 = 1;
    let mut y: i32 = 1;
    for i in 0..count {
        locations.push(Location::new(x, y, characters[i].get_id(), order));
        if i % 2 == 0 {
            x *= i as i32;
            y *= -1 * i as i32;
        } else {
            x *= -1 * i as i32;
            y *= i as i32;
        }

        order += 1;
    }

    let response = Response::new(characters, messages, locations);
    serde_json::to_string(&response).unwrap()
}

#[wasm_bindgen]
pub fn compile(buf: String) -> Result<String, String> {
    println!("{}", &buf);
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moch_uniqueness() {
        let buf = String::from(" ");
        let moch_reponse0 = moch(buf.clone());
        let moch_reponse1 = moch(buf);

        assert_ne!(moch_reponse0, moch_reponse1);
    }
}
