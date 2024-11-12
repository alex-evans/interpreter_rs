
use super::{Element, Visitor};

pub struct NumberToken {
    pub text: String,
    pub literal: String,
    pub line: i32,
    pub error: String,
}

impl Element for NumberToken {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_number_token(self);
    }
}

impl NumberToken {
    pub fn new(input_text: String, line: i32) -> Self {
        let (literal, error) = Self::handle_number(input_text.clone());
        NumberToken{
            text: input_text,
            literal,
            line,
            error,
        }
    }

    fn handle_number(input_text: String) -> (String, String) {
        let parts: Vec<&str> = input_text.split('.').collect();

        if parts.len() == 1 {
            return (format!("{}.0", input_text), String::new());
        }

        if parts.len() > 2 {
            return ("null".to_string(), "Invalid number.".to_string());
        }

        let integer_part = parts[0];
        let decimal_part = parts[1].trim_end_matches('0');

        if decimal_part.is_empty() {
            return (format!("{}.0", integer_part), String::new());
        }

        return (format!("{}.{}", integer_part, decimal_part), String::new());
    }
}