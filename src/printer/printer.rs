
use crate::tokens::Token;

pub struct Printer {
    command_type: String,
}

impl Printer {
    pub fn new(command_type: String) -> Self {
        Printer {
            command_type,
        }
    }

    pub fn print(&self, tokens: Vec<Token>, expression: String) -> Result<i32, ()> {
        let mut return_code = 0;
        if self.command_type == "tokenize".to_string() {
            for token in tokens {
                match &token {
                    Token::Keyword(token) => {
                        if !token.error.is_empty() {
                            eprintln!("[line {}] Error: {}", token.line, token.error);
                            return_code = 65;
                        } else {
                            println!("{} {} {}", token.token_type, token.text, token.literal);
                        }
                    },
                    Token::Identifier(token) => {
                        if !token.error.is_empty() {
                            eprintln!("[line {}] Error: {}", token.line, token.error);
                            return_code = 65;
                        } else {
                            println!("IDENTIFIER {} {}", token.text, token.literal);
                        }
                    },
                    Token::Number(token) => {
                        if !token.error.is_empty() {
                            eprintln!("[line {}] Error: {}", token.line, token.error);
                            return_code = 65;
                        } else {
                            println!("NUMBER {} {}", token.text, token.literal);
                        }
                    },
                    Token::Punctuator(token) => {
                        if !token.error.is_empty() {
                            eprintln!("[line {}] Error: {}", token.line, token.error);
                            return_code = 65;
                        } else {
                            println!("{} {} {}", token.token_type, token.text, token.literal);
                        }
                    },
                    Token::String(token) => {
                        if !token.error.is_empty() {
                            eprintln!("[line {}] Error: {}", token.line, token.error);
                            return_code = 65;
                        } else {
                            println!("STRING {} {}", token.text, token.literal);
                        }
                    },
                }

            }
            println!("EOF  null");
        } else if self.command_type == "parse".to_string() {
            println!("{}", expression);
        }

        return Ok(return_code);
    }

}