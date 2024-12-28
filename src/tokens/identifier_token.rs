
use std::any::Any;
use super::{Element, Visitor};

#[derive(Clone)]
pub struct IdentifierToken {
    pub text: String,
    pub literal: String,
    pub line: i32,
    pub error: String,
}

impl Element for IdentifierToken {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_identifier_token(self);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl IdentifierToken {
    pub fn new(input_text: String, line: i32) -> Self {
        let (literal, error) = Self::handle_identifier(input_text.clone());
        IdentifierToken{
            text: input_text,
            literal,
            line,
            error,
        }
    }

    fn handle_identifier(_input_text: String) -> (String, String) {
        return ("null".to_string(), String::new())
    }

}