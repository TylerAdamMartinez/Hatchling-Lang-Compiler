use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

type SerdeUuid = [u8; 16];

#[wasm_bindgen]
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    x: u32,
    y: u32,
    character_id: SerdeUuid,
    order: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    message: String,
    character_id: SerdeUuid,
    order: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Character {
    hair_color: String,
    eye_color: String,
    skin_color: String,
    outfit: String,
    pub id: SerdeUuid,
    pub position: Location,
}

#[wasm_bindgen]
pub fn moch() -> Result<String, String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
