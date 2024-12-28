
use super::{
    KeywordToken, 
    IdentifierToken, 
    NumberToken, 
    PunctuatorToken,
    StringToken,
    Token,
};

pub struct TokenFactory;

impl TokenFactory {
    pub fn create_token(input_text: String, line: i32) -> Token {
        if input_text.starts_with("\"") {
            Token::String(StringToken::new(input_text, line))
        } else if input_text.chars().next().unwrap().is_digit(10) {
            Token::Number(NumberToken::new(input_text, line))
        } else if Self::is_a_keyword(&input_text) {
            Token::Keyword(KeywordToken::new(input_text, line))
        } else if Self::is_a_identifier(&input_text) {
            Token::Identifier(IdentifierToken::new(input_text, line))
        } else {
            Token::Punctuator(PunctuatorToken::new(input_text, line))
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