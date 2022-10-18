use crate::definitions;

#[allow(dead_code)]
pub fn parser(buf: &str) -> definitions::Token {
    return definitions::Token::NumericLiteral(buf.parse::<f64>().unwrap());
}

#[cfg(test)]
mod parser_tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn simple_number() {
        let program = String::from("42");

        let result = parser(&program);
        assert_eq!(result, definitions::Token::NumericLiteral(42.0));
    }
}
