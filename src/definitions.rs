use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// # Definitions
///
/// This file contains all the definitions for the hatchling lang compiler

/// ## Location
///
/// x & y is relative to the window screen
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

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TypeKind {
    NumberType,
    StringType,
    BooleanType,
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    StatementList,
    BlockStatement,
    IfStatement,
    FunctionDefinition,
    Arguements,
    Indentifier(String),
    NumericLiteral(f64),
    StringLiteral(String),
    Operator(Operation),
    Symbol(Symbol),

    /// BinaryExpression(
    ///     Operator , BinaryExpression or NumericLiteral , BinaryExpression or NumericLiteral
    /// )
    BinaryExpression(Box<Token>, Box<Token>, Box<Token>),
    UnaryExpression,
}

#[allow(dead_code)]
pub enum Keyword {
    // Functional
    Function,
    Return,

    // BooleanType
    True,
    False,

    // Character
    Say,
    Move,
    Position,
    ID,
    Hair,
    Skin,
    Eyes,
    Outfit,
    Background,
    Left,
    Right,
    Up,
    Down,

    // Conditionals
    And,
    Or,
    Not,
    If,
    Elseif,
    Else,
    Isequal,
    Greaterthan,
    Lessthan,

    // Build in functions
    Powerof,
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum Operation {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulus,
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum Symbol {
    // Special
    ParensL,
    ParensR,
    SemiColon,

    // File
    Eos,
    Eof,

    // Error
    Invalid,
}

#[allow(dead_code)]
pub struct AST {
    node: Token,
    leaves: Vec<Box<AST>>,
}

#[allow(dead_code)]
pub enum ParseError {
    Unexpected(Token),
    DuplicateModifier(Token),
    Syntax(Token),
}
