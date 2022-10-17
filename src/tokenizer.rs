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
        static ref REG_STRING: Regex = Regex::new(r#"(.*?)"#).unwrap();
    }

    // need to splice buf from cursor
    while cursor < length {
        if REG_DIGIT.is_match(&buf[start..cursor]) {
            while REG_DIGIT.is_match(&buf[start..cursor]) && !(cursor >= length) {
                cursor += 1;
            }
            tokens.push(definitions::Token::NumericLiteral(
                buf[start..cursor].parse::<f64>().unwrap(),
            ));
            start = cursor;
        } else if &buf.chars().nth(start).unwrap() == &'\"' {
            loop {
                cursor += 1;
                if cursor >= length {
                    break;
                } else if &buf.chars().nth(cursor).unwrap() == &'\"' {
                    break;
                } else if &buf.chars().nth(cursor).unwrap() == &'\n' {
                    break;
                }
            }
            tokens.push(definitions::Token::StringLiteral(
                buf[start..cursor].to_owned(),
            ));
        } else {
            cursor += 1;
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
            cursor += 1;
            while REG_DIGIT.is_match(&program[start..cursor]) && !(cursor >= program.len()) {
                println!("hello I'm a digit as {}", cursor);
                cursor += 1;
            }
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
    fn simple_string_literal() {
        let program = String::from(r#"Hello World"#);

        let result = tokenizer(&program);
        assert_eq!(
            result,
            vec![definitions::Token::StringLiteral(
                r#"Hello World"#.to_owned()
            )]
        );
    }
}
