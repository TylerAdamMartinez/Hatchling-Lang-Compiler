use wasm_bindgen::prelude::*;
mod definitions;
mod parser;
mod tokenizer;

#[wasm_bindgen]
pub fn moch(buf: String) -> String {
    println!("{}", &buf);

    let mut order: usize = 0;

    let mut characters = Vec::<definitions::Character>::new();
    let mut messages = Vec::<definitions::Message>::new();
    let mut locations = Vec::<definitions::Location>::new();

    let count = rand::random::<usize>() % 10;

    for _ in 0..count {
        characters.push(definitions::Character::new(
            "hair".to_owned(),
            "eye".to_owned(),
            "skin".to_owned(),
            "outfit".to_owned(),
        ));
    }

    for i in 0..count {
        messages.push(definitions::Message::new(
            "message".to_owned(),
            characters[i].get_id(),
            order,
        ));
        order += 1;
    }

    let mut x: i32 = 1;
    let mut y: i32 = 1;
    for i in 0..count {
        locations.push(definitions::Location::new(
            x,
            y,
            characters[i].get_id(),
            order,
        ));
        if i % 2 == 0 {
            x *= i as i32;
            y *= -1 * i as i32;
        } else {
            x *= -1 * i as i32;
            y *= i as i32;
        }

        order += 1;
    }

    let response = definitions::Response::new(characters, messages, locations);
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
    use pretty_assertions::assert_ne;

    #[test]
    fn moch_uniqueness() {
        let buf = String::from(" ");
        let moch_reponse0 = moch(buf.clone());
        let moch_reponse1 = moch(buf);

        println!("moch reponse 0");
        println!("{:#?}", moch_reponse0);

        println!("moch reponse 1");
        println!("{:#?}", moch_reponse1);

        assert_ne!(moch_reponse0, moch_reponse1);
    }
}
