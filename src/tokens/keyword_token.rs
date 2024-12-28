
use std::any::Any;
use super::{Element, Visitor};

#[derive(Clone)]
pub struct KeywordToken {
    pub token_type: String,
    pub text: String,
    pub literal: String,
    pub line: i32,
    pub error: String,
}

impl Element for KeywordToken {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_keyword_token(self);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl KeywordToken {
    pub fn new(input_text: String, line: i32) -> Self {
        let (token_type, literal, error) = Self::handle_keyword(input_text.clone());
        KeywordToken{
            token_type,
            text: input_text,
            literal,
            line,
            error,
        }
    }

    fn handle_keyword(input_text: String) -> (String, String, String) {
        let literal = "null".to_string();
        let mut error = "".to_string();

        let token_type = match input_text.as_str() {
            "and" => "AND".to_string(),
            "class" => "CLASS".to_string(),
            "else" => "ELSE".to_string(),
            "false" => "FALSE".to_string(),
            "for" => "FOR".to_string(),
            "fun" => "FUN".to_string(),
            "if" => "IF".to_string(),
            "nil" => "NIL".to_string(),
            "or" => "OR".to_string(),
            "print" => "PRINT".to_string(),
            "return" => "RETURN".to_string(),
            "super" => "SUPER".to_string(),
            "this" => "THIS".to_string(),
            "true" => "TRUE".to_string(),
            "var" => "VAR".to_string(),
            "while" => "WHILE".to_string(),
            _ => {
                error = format!("Unexpected keyword: {}", input_text);
                "UNKNOWN".to_string()
            }
        };

        return (token_type, literal, error);
    }

    pub fn expression(&self) -> String {
        return self.text.clone();
    }

}