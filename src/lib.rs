
pub mod tokens;


use crate::tokens::{Element, TokenFactory, OutputVisitor};

pub fn run(file_contents: String) -> i32 {
    let mut tokens: Vec<Box<dyn Element>> = Vec::new();
    let mut line = 1;

    let mut chars = file_contents.chars().peekable();

    while let Some(current_char) = chars.next() {
        match (current_char, chars.peek()) {
            ('=', Some(&'=')) => {
                let token: Box<dyn Element> = TokenFactory::create_token("==".to_string(), line);
                tokens.push(token);
                chars.next();
            },
            ('!', Some(&'=')) => {
                let token: Box<dyn Element> = TokenFactory::create_token("!=".to_string(), line);
                tokens.push(token);
                chars.next();
            },
            ('<', Some(&'=')) => {
                let token: Box<dyn Element> = TokenFactory::create_token("<=".to_string(), line);
                tokens.push(token);
                chars.next();
            },
            ('>', Some(&'=')) => {
                let token: Box<dyn Element> = TokenFactory::create_token(">=".to_string(), line);
                tokens.push(token);
                chars.next();
            },
            ('/', Some(&'/')) => {
                while let Some(c) = chars.next() {
                    if c == '\n' {
                        line += 1;
                        break;
                    }
                }
            },
            ('"', _) => {
                let mut literal = current_char.to_string();
                while let Some(c) = chars.next() {
                    literal.push(c);
                    if c == '\n' {
                        line += 1;
                    }
                    if c == '"' {
                        break;
                    }
                }
                let token: Box<dyn Element> = TokenFactory::create_token(literal, line);
                tokens.push(token);
            },
            (' ', _) | ('\t', _) => {},
            ('\n', _) => {
                line += 1;
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
                let token: Box<dyn Element> = TokenFactory::create_token(number, line);
                tokens.push(token);
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
                let token: Box<dyn Element> = TokenFactory::create_token(identifier, line);
                tokens.push(token);
            },
            (c, _) => {
                let token: Box<dyn Element> = TokenFactory::create_token(c.to_string(), line);
                tokens.push(token);
            }
        }
    }

    let mut output_visitor = OutputVisitor::new();

    for token in tokens {
        token.accept(&mut output_visitor);
    }

    println!("EOF  null");

    output_visitor.return_code
}