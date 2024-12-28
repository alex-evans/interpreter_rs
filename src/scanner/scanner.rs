
use super::tokens::TokenFactory;
use super::tokens::Token;

pub struct Scanner {
    input_text: String,
    tokens: Vec<Token>,
    line: i32,
}

impl Scanner {
    pub fn new(input_text: String) -> Self {
        Scanner {
            input_text,
            tokens: Vec::new(),
            line: 1,
        }
    }

    pub fn scan(&mut self) -> Result<Vec<Token>, String> {
        let mut chars = self.input_text.chars().peekable();

        while let Some(current_char) = chars.next() {
            match (current_char, chars.peek()) {
                ('=', Some(&'=')) => {
                    let token: Token = TokenFactory::create_token("==".to_string(), self.line);
                    self.tokens.push(token);
                    chars.next();
                },
                ('!', Some(&'=')) => {
                    let token: Token = TokenFactory::create_token("!=".to_string(), self.line);
                    self.tokens.push(token);
                    chars.next();
                },
                ('<', Some(&'=')) => {
                    let token: Token = TokenFactory::create_token("<=".to_string(), self.line);
                    self.tokens.push(token);
                    chars.next();
                },
                ('>', Some(&'=')) => {
                    let token: Token = TokenFactory::create_token(">=".to_string(), self.line);
                    self.tokens.push(token);
                    chars.next();
                },
                ('/', Some(&'/')) => {
                    while let Some(c) = chars.next() {
                        if c == '\n' {
                            self.line += 1;
                            break;
                        }
                    }
                },
                ('"', _) => {
                    let mut literal = current_char.to_string();
                    while let Some(c) = chars.next() {
                        literal.push(c);
                        if c == '\n' {
                            self.line += 1;
                        }
                        if c == '"' {
                            break;
                        }
                    }
                    let token: Token = TokenFactory::create_token(literal, self.line);
                    self.tokens.push(token);
                },
                (' ', _) | ('\t', _) => {},
                ('\n', _) => {
                    self.line += 1;
                },
                (c, _) if c.is_digit(10) => {
                    let mut number = c.to_string();
                    while let Some(&next_char) = chars.peek() {
                        if next_char == '.' && number.chars().filter(|&c| c == '.').count() > 0 {
                            break;
                        }

                        if next_char.is_digit(10) || next_char == '.' {
                            number.push(next_char);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    let token: Token = TokenFactory::create_token(number, self.line);
                    self.tokens.push(token);
                },
                (c, _) if c.is_alphabetic() || c == '_' => {
                    let mut identifier = c.to_string();
                    while let Some(&next_char) = chars.peek() {
                        if next_char.is_alphanumeric() || next_char == '_' {
                            identifier.push(next_char);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    let token: Token = TokenFactory::create_token(identifier, self.line);
                    self.tokens.push(token);
                },
                (c, _) => {
                    let token: Token = TokenFactory::create_token(c.to_string(), self.line);
                    self.tokens.push(token);
                }
            }
        }
        return Ok(self.tokens.clone());
    }
}