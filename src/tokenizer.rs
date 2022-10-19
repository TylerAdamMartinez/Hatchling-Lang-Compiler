use crate::definitions;
use lazy_static::lazy_static;
use regex::Regex;

#[allow(dead_code)]
pub fn tokenizer(buf: &str) -> Vec<definitions::Token> {
    let length = buf.len();
    let mut start = 0;
    let mut cursor = 1;
    let mut tokens = Vec::<definitions::Token>::new();

    lazy_static! {
        static ref DIGIT: Regex = Regex::new(r"\d+").unwrap();
        static ref STRING: Regex = Regex::new(r#""(.*?)""#).unwrap();
        static ref WHITESPACE: Regex = Regex::new(r"\s+").unwrap();
        static ref COMMENTS: Regex = Regex::new(r"//.*").unwrap();
        static ref MULTI_LINE_COMMENTS: Regex = Regex::new(r##"/\*[\s\S]*?\*/"##).unwrap();
        static ref OPERATORS: Regex = Regex::new(r"[\+\-\*/\%]").unwrap();
    }

    // need to splice buf from cursor
    while cursor < length {
        if &buf[start..cursor] == r" " {
            let found = WHITESPACE.find(&buf[start..]).unwrap();

            start += found.end();
            cursor = start + 1;
        } else if &buf[start..cursor + 1] == r"//" {
            let found = COMMENTS.find(&buf[start..]).unwrap();
            start += found.end();
            cursor = start + 1;
        } else if &buf[start..cursor + 1] == r"/*" {
            let found = MULTI_LINE_COMMENTS.find(&buf[start..]).unwrap();
            start += found.end();
            cursor = start + 1;
        } else if OPERATORS.is_match(&buf[start..cursor]) {
            if &buf[start..cursor] == "+" {
                tokens.push(definitions::Token::Operator(definitions::Operation::Plus));
            } else if &buf[start..cursor] == "-" {
                tokens.push(definitions::Token::Operator(definitions::Operation::Minus));
            } else if &buf[start..cursor] == "*" {
                tokens.push(definitions::Token::Operator(
                    definitions::Operation::Multiply,
                ));
            } else if &buf[start..cursor] == "/" {
                tokens.push(definitions::Token::Operator(definitions::Operation::Divide));
            } else if &buf[start..cursor] == "%" {
                tokens.push(definitions::Token::Operator(
                    definitions::Operation::Modulus,
                ));
            }

            start = cursor;
            cursor = start + 1;
        } else if DIGIT.is_match(&buf[start..cursor]) {
            let found = DIGIT.find(&buf[start..]).unwrap();
            cursor = start + found.end();

            tokens.push(definitions::Token::NumericLiteral(
                buf[start..cursor].parse::<f64>().unwrap(),
            ));

            start = cursor;
            cursor = start + 1;
        } else if &buf[start..cursor] == r#"""# {
            let found = STRING.find(&buf[start..]).unwrap();
            cursor = start + found.end();

            tokens.push(definitions::Token::StringLiteral(
                buf[start..cursor].to_owned(),
            ));

            start = cursor;
            cursor = start + 1;
        } else {
            //Should Error out if this is the case
            println!("Should Error out if this is the case");
            start = cursor;
            cursor = start + 1;
        }
    }
    tokens
}

#[cfg(test)]
mod tokenizer_tests_simple {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn simple_numberic_regex() {
        let program = String::from("42");

        lazy_static! {
            static ref DIGIT: Regex = Regex::new(r"\d+").unwrap();
        }

        let start = 0;
        let mut cursor = 1;
        if DIGIT.is_match(&program[start..cursor]) {
            let found = DIGIT.find(&program[start..]).unwrap();
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
            static ref STRING: Regex = Regex::new(r#""(.*?)""#).unwrap();
        }

        let start = 0;
        let mut cursor = 1;
        if &program[start..cursor] == r#"""# {
            let found = STRING.find(&program[start..]).unwrap();
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
    fn simple_string_literal_with_whitespace() {
        let program = String::from(r#"     "   42  "    "#);

        let result = tokenizer(&program);
        assert_eq!(
            result,
            vec![definitions::Token::StringLiteral(r#""   42  ""#.to_owned())]
        );
    }

    #[test]
    fn simple_numeric_plus_string_literal() {
        let program = String::from(r#"9080"Hello Sally"321"#);

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

    #[test]
    fn simple_numeric_plus_string_literal_with_whitespace() {
        let program = String::from(r#" 9781 " Howdy Y'all  "    124  "#);

        let result = tokenizer(&program);
        assert_eq!(
            result,
            vec![
                definitions::Token::NumericLiteral(9781.0),
                definitions::Token::StringLiteral(r#"" Howdy Y'all  ""#.to_owned()),
                definitions::Token::NumericLiteral(124.0),
            ]
        );
    }

    #[test]
    fn simple_comment() {
        let program = String::from(r#"//     "  lorem   "    "#);

        let result = tokenizer(&program);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn simple_string_literal_with_comment() {
        let program = String::from(r#""     This is a string"    // TODO: lorem      "#);

        let result = tokenizer(&program);
        assert_eq!(
            result,
            vec![definitions::Token::StringLiteral(
                r#""     This is a string""#.to_owned()
            )]
        );
    }

    #[test]
    fn simple_numberic_literal_with_comment() {
        let program = String::from(r#"     7489327    // TODO: write more test      "#);

        let result = tokenizer(&program);
        assert_eq!(result, vec![definitions::Token::NumericLiteral(7489327.0)]);
    }
}

#[cfg(test)]
mod tokenizer_tests_multi {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn multi_numberic_string_literal_with_single_comment() {
        let program = String::from(
            r#"     7489327    
            // TODO: write more test 
            //
            //
             " Another string literal" "#,
        );

        let result = tokenizer(&program);
        assert_eq!(
            result,
            vec![
                definitions::Token::NumericLiteral(7489327.0),
                definitions::Token::StringLiteral(r#"" Another string literal""#.to_owned())
            ]
        );
    }

    #[test]
    fn multi_numberic_string_literal_with_multi_comment() {
        let program = String::from(
            r#" " Tyler was here " 
            /* TODO: write more test 
            *   Also make sure to add multi-line doc comments 
            *
            */
            "president " 45 "#,
        );

        let result = tokenizer(&program);
        assert_eq!(
            result,
            vec![
                definitions::Token::StringLiteral(r#"" Tyler was here ""#.to_owned()),
                definitions::Token::StringLiteral(r#""president ""#.to_owned()),
                definitions::Token::NumericLiteral(45.0),
            ]
        );
    }
}

#[cfg(test)]
mod tokenizer_tests_simple_symbols_and_operators {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn symbol_parens_l() {
        let program = String::from(r#"("#);

        let result = tokenizer(&program);
        assert_eq!(
            result,
            vec![definitions::Token::Symbol(definitions::Symbol::ParensL)]
        );
    }

    #[test]
    fn symbol_parens_r() {
        let program = String::from(r#")"#);

        let result = tokenizer(&program);
        assert_eq!(
            result,
            vec![definitions::Token::Symbol(definitions::Symbol::ParensR)]
        );
    }

    #[test]
    fn symbol_semicolon() {
        let program = String::from(r#";"#);

        let result = tokenizer(&program);
        assert_eq!(
            result,
            vec![definitions::Token::Symbol(definitions::Symbol::SemiColon)]
        );
    }

    #[test]
    fn operator_modulus() {
        let program = String::from(r#"%"#);

        let result = tokenizer(&program);
        assert_eq!(
            result,
            vec![definitions::Token::Operator(
                definitions::Operation::Modulus
            )]
        );
    }

    #[test]
    fn operator_divide() {
        let program = String::from(r#"/"#);

        let result = tokenizer(&program);
        assert_eq!(
            result,
            vec![definitions::Token::Operator(definitions::Operation::Divide)]
        );
    }

    #[test]
    fn operator_multiply() {
        let program = String::from(r#"*"#);

        let result = tokenizer(&program);
        assert_eq!(
            result,
            vec![definitions::Token::Operator(
                definitions::Operation::Multiply
            )]
        );
    }

    #[test]
    fn operator_minus() {
        let program = String::from(r#"-"#);

        let result = tokenizer(&program);
        assert_eq!(
            result,
            vec![definitions::Token::Operator(definitions::Operation::Minus)]
        );
    }

    #[test]
    fn operator_plus() {
        let program = String::from(r#"+"#);

        let result = tokenizer(&program);
        assert_eq!(
            result,
            vec![definitions::Token::Operator(definitions::Operation::Plus)]
        );
    }
}
