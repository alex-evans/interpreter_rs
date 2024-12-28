
use super::Element;
use crate::tokens::KeywordToken;
use crate::tokens::Token;

pub struct Parser {
    // input_tokens: Vec<Box<dyn Element>>

}

impl Parser {
    pub fn new() -> Self {
        Parser {
            // input_tokens: Vec::new(),
        }
    }

    pub fn parse(&mut self, input_tokens: Vec<Token>) -> Result<String, String> {
        let mut tokens = input_tokens.iter().peekable();

        while let Some(current_token) = tokens.next() {
            if let Some(keyword_token) = current_token.as_any().downcast_ref::<KeywordToken>() {
                return Ok(keyword_token.expression());
            };
            
                // Token::Keyword(keyword) => {
                //     match keyword.as_str() {
                //         "if" => {
                //             // Parse if statement
                //         },
                //         "while" => {
                //             // Parse while statement
                //         },
                //         "for" => {
                //             // Parse for statement
                //         },
                //         _ => {
                //             return Err(format!("Unexpected keyword: {}", keyword));
                //         }
                //     }
                // },
                // Token::Identifier(identifier) => {
                //     // Parse identifier
                // },
                // Token::Literal(literal) => {
                //     // Parse literal
                // },
                // Token::Operator(operator) => {
                //     // Parse operator
                // },
                // Token::Punctuation(punctuation) => {
                //     // Parse punctuation
                // },
                // Token::Comment(comment) => {
                //     // Parse comment
                // },
                // Token::Whitespace(whitespace) => {
                //     // Parse whitespace
                // },
                // Token::Newline(newline) => {
                //     // Parse newline
                // },
        }

        return Ok("Parsing skipped".to_string());
        
    }
}