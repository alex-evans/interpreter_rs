
// use super::Element;
// use crate::tokens::KeywordToken;
use crate::tokens::Token;

pub struct Parser {}

impl Parser {
    pub fn new() -> Self {
        Parser {
        }
    }

    pub fn parse(&mut self, input_tokens: Vec<Token>) -> Result<String, String> {
        let mut tokens = input_tokens.iter().peekable();

        while let Some(current_token) = tokens.next() {
            match current_token {
                Token::Keyword(keyword_token) => {
                    return Ok(keyword_token.expression());
                },
                Token::Identifier(_identifier_token) => {
                    return Ok("Skipped Identifier".to_string());
                },
                Token::Number(_number_token) => {
                    return Ok("Skipped Number".to_string());
                },
                Token::Punctuator(_punctuator_token) => {
                    return Ok("Skipped Punctuator".to_string());
                },
                Token::String(_string_token) => {
                    return Ok("Skipped String".to_string());
                }
            }
        }

        return Ok("Parsing skipped".to_string());
        
    }
}