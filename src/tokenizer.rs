use crate::definitions;
use lazy_static::lazy_static;
use regex::Regex;

pub fn tokenizer(buf: &str) -> Vec<definitions::Token> {
    let length = buf.len();
    let mut start = 0;
    let mut cursor = 1;
    let mut tokens = Vec::<definitions::Token>::new();

    lazy_static! {
        static ref REG_DIGIT: Regex = Regex::new(r"\d+").unwrap();
        static ref REG_STRING: Regex = Regex::new(r#""(.*?)""#).unwrap();
        static ref WHITESPACE: Regex = Regex::new(r"\s+").unwrap();
    }

    // need to splice buf from cursor
    while cursor < length {
        if &buf[start..cursor] == r" " {
            println!("hello I'm a space");
            let found = WHITESPACE.find(&buf[start..]).unwrap();
            println!("hello I'm a space from {} to {}", start, cursor);
            start += found.end();
            cursor = start + 1;
            println!("hello I'm now at start {} and cursor {}", start, cursor);
        } else if REG_DIGIT.is_match(&buf[start..cursor]) {
            println!("hello I'm a digit");
            println!(
                "hello I'm a digit before find start {} cursor {}",
                start, cursor
            );
            let found = REG_DIGIT.find(&buf[start..]).unwrap();
            cursor = start + found.end();
            println!("hello I'm a digit from {} to {}", start, cursor);
            tokens.push(definitions::Token::NumericLiteral(
                buf[start..cursor].parse::<f64>().unwrap(),
            ));
            start = cursor;
            cursor = start + 1;
            println!("hello I'm now at start {} and cursor {}", start, cursor);
        } else if &buf[start..cursor] == r#"""# {
            let found = REG_STRING.find(&buf[start..]).unwrap();
            cursor = start + found.end();
            tokens.push(definitions::Token::StringLiteral(
                buf[start..cursor].to_owned(),
            ));
            start = cursor;
            cursor = start + 1;
        } else {
            start = cursor;
            cursor = start + 1;
        }
    }
    tokens
}

#[cfg(test)]
mod tokenizer_tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn simple_numberic_regex() {
        let program = String::from("42");

        lazy_static! {
            static ref REG_DIGIT: Regex = Regex::new(r"\d+").unwrap();
        }

        let start = 0;
        let mut cursor = 1;
        if REG_DIGIT.is_match(&program[start..cursor]) {
            println!("hello I'm a digit as {}", cursor);
            /*cursor += 1;
            while REG_DIGIT.is_match(&program[start..cursor]) && !(cursor >= program.len()) {
                println!("hello I'm a digit as {}", cursor);
                cursor += 1;
            }*/
            let found = REG_DIGIT.find(&program[start..]).unwrap();
            cursor = found.end();
        }

        assert_eq!(&program[0..1], "4");
        assert_eq!(cursor, 2);
        assert_eq!(&program[start..cursor], "42");
        let result =
            definitions::Token::NumericLiteral(program[start..cursor].parse::<f64>().unwrap());
        assert_eq!(result, definitions::Token::NumericLiteral(42.0));
    }

    #[test]
    fn simple_numberic_literal() {
        let program = String::from("42");

        let result = tokenizer(&program);
        assert_eq!(result, vec![definitions::Token::NumericLiteral(42.0)]);
    }

    #[test]
    fn simple_numberic_literal_with_whitespace() {
        let program = String::from("   42  ");

        let result = tokenizer(&program);
        assert_eq!(result, vec![definitions::Token::NumericLiteral(42.0)]);
    }

    #[test]
    fn simple_string_regex() {
        let program = String::from(r#""This is it""#);

        lazy_static! {
            static ref REG_STRING: Regex = Regex::new(r#""(.*?)""#).unwrap();
        }

        let start = 0;
        let mut cursor = 1;
        if &program[start..cursor] == r#"""# {
            println!("Matched!");
            let found = REG_STRING.find(&program[start..]).unwrap();
            println!(
                "hello I'm a string from {} to {}",
                found.start(),
                found.end()
            );
            cursor = found.end();
        }

        assert_eq!(cursor, 12);
        assert_eq!(&program[start..cursor], r#""This is it""#);
    }

    #[test]
    fn simple_string_literal() {
        let program = String::from(r#""Hello World""#);

        let result = tokenizer(&program);
        assert_eq!(
            result,
            vec![definitions::Token::StringLiteral(
                r#""Hello World""#.to_owned()
            )]
        );
    }

    #[test]
    fn simple_numeric_plus_string_literal() {
        let program = String::from(r#"9080"Hello Sally"321"#);
        println!("{}", program);

        let result = tokenizer(&program);
        assert_eq!(
            result,
            vec![
                definitions::Token::NumericLiteral(9080.0),
                definitions::Token::StringLiteral(r#""Hello Sally""#.to_owned()),
                definitions::Token::NumericLiteral(321.0),
            ]
        );
    }
}
