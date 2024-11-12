
use super::{
    Element, 
    KeywordToken, 
    IdentifierToken, 
    NumberToken, 
    PunctuatorToken,
    StringToken,
};

pub struct TokenFactory;

impl TokenFactory {
    pub fn create_token(input_text: String, line: i32) -> Box<dyn Element> {
        if input_text.starts_with("\"") {
            Box::new(StringToken::new(input_text, line))
        } else if input_text.chars().next().unwrap().is_digit(10) {
            Box::new(NumberToken::new(input_text, line))
        } else if Self::is_a_keyword(&input_text) {
            Box::new(KeywordToken::new(input_text, line))
        } else if Self::is_a_identifier(&input_text) {
            Box::new(IdentifierToken::new(input_text, line))
        } else {
            Box::new(PunctuatorToken::new(input_text, line))
        }
    }

    fn is_a_keyword(input_text: &str) -> bool {
        let keywords = vec![
            "and", "class", "else", "false", "for", "fun", "if", "nil", "or",
            "print", "return", "super", "this", "true", "var", "while"
        ];
        return keywords.contains(&input_text);
    }

    fn is_a_identifier(input_text: &str) -> bool {
        return input_text.chars().next().unwrap().is_alphabetic() || input_text.chars().next().unwrap() == '_';
    }
}