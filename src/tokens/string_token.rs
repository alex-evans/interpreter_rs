
use std::any::Any;
use super::{Element, Visitor};

#[derive(Clone)]
#[derive(Debug)]
pub struct StringToken {
    pub text: String,
    pub literal: String,
    pub line: i32,
    pub error: String,
}

impl Element for StringToken {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_string_token(self);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl StringToken {
    pub fn new(input_text: String, line: i32) -> Self {
        let (literal, error) = Self::handle_string(input_text.clone());
        StringToken{
            text: input_text,
            literal,
            line,
            error,
        }
    }

    fn handle_string(input_text: String) -> (String, String) {
        if input_text.len() > 1 && input_text.ends_with("\"") {
            (input_text[1..input_text.len()-1].to_string(), String::new())
        } else {
            ("null".to_string(), "Unterminated string.".to_string())
        }
    }

    pub fn print(&self) {
        println!("STRING {} {}", self.text, self.literal);
    }
}