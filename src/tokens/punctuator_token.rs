
use std::any::Any;
use super::{Element, Visitor};

#[derive(Clone)]
pub struct PunctuatorToken {
    pub token_type: String,
    pub text: String,
    pub literal: String,
    pub line: i32,
    pub error: String,
}

impl Element for PunctuatorToken {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_punctuator_token(self);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl PunctuatorToken {
    pub fn new(input_text: String, line: i32) -> Self {
        let (token_type, literal, error) = Self::handle_punctuators(input_text.clone());
        PunctuatorToken{
            token_type,
            text: input_text,
            literal,
            line,
            error,
        }
    }

    fn handle_punctuators(input_text: String) -> (String, String, String) {
        let literal = "null".to_string();
        let mut error = "".to_string();

        let token_type = match input_text.as_str() {
            "(" => "LEFT_PAREN".to_string(),
            ")" => "RIGHT_PAREN".to_string(),
            "{" => "LEFT_BRACE".to_string(),
            "}" => "RIGHT_BRACE".to_string(),
            "*" => "STAR".to_string(),
            "." => "DOT".to_string(),
            "," => "COMMA".to_string(),
            "+" => "PLUS".to_string(),
            "-" => "MINUS".to_string(),
            ";" => "SEMICOLON".to_string(),
            "==" => "EQUAL_EQUAL".to_string(),
            "=" => "EQUAL".to_string(),
            "!" => "BANG".to_string(),
            "!=" => "BANG_EQUAL".to_string(),
            "<" => "LESS".to_string(),
            "<=" => "LESS_EQUAL".to_string(),
            ">" => "GREATER".to_string(),
            ">=" => "GREATER_EQUAL".to_string(),
            "/" => "SLASH".to_string(),
            _ => {
                error = format!("Unexpected character: {}", input_text);
                "UNKNOWN".to_string()
            }
        };

        (token_type, literal, error)
    }

    pub fn print(&self) {
        println!("{} {} {}", self.token_type, self.text, self.literal);
    }
}

