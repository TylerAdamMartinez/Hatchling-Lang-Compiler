#[allow(unused_imports)]
use crate::definitions;

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn parser(buf: &Vec<definitions::Token>) {
    todo!()
}

#[cfg(test)]
mod parser_tests_simple {
    #[allow(unused_imports)]
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::assert_eq;

    #[test]
    fn simple_parse() {}
}
