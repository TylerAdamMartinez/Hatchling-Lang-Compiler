use wasm_bindgen_test::*;
mod lib;

#[cfg(target_arch = "wasm32")]
mod web_test {
    use super::*;

    #[wasm_binden_test]
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
