
pub struct Token {
    text: String,
    token_type: String,
    literal: String,
    line: i32,
    error: String,
}

impl Token {
    pub fn new(input_text: String, line: i32) -> Token {
        
        let mut error = "".to_string();
        let text = input_text.clone();
        let mut literal = "null".to_string();

        let token_type = if input_text.starts_with("\"") {
            if input_text.len() > 1 && input_text.ends_with("\"") {
                literal = input_text[1..input_text.len()-1].to_string();  // remove the quotes
                "STRING".to_string()
            } else {
                error = "Unterminated string.".to_string();
                "UNKNOWN".to_string()
            }
        } else {
            match input_text.as_str() {
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
            }
        };

        Token {
            text,
            token_type,
            literal,
            line,
            error,
        }
    }

    pub fn output(&self) {
        if self.error != ""{
            eprintln!("[line {}] Error: {}", self.line, self.error);
        } else {
            println!("{} {} {}", self.token_type, self.text, self.literal)
        }
    }
}

pub fn run(
    file_contents: String
) -> i32 {
    let mut return_code = 0;
    let mut tokens: Vec<Token> = Vec::new();
    let mut line = 1;
    
    let mut chars = file_contents.chars().peekable();

    while let Some(current_char) = chars.next() {
        match (current_char, chars.peek()) {
            ('=', Some(&'=')) => {
                let token = Token::new("==".to_string(), line);
                tokens.push(token);
                chars.next();
            },
            ('!', Some(&'=')) => {
                let token = Token::new("!=".to_string(), line);
                tokens.push(token);
                chars.next();
            },
            ('<', Some(&'=')) => {
                let token = Token::new("<=".to_string(), line);
                tokens.push(token);
                chars.next();
            },
            ('>', Some(&'=')) => {
                let token = Token::new(">=".to_string(), line);
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
                let token = Token::new(literal, line);
                tokens.push(token);
            },
            (' ', _) | ('\t', _) => {},
            ('\n', _) => {
                line += 1;
            },
            (c, _) => {
                let token = Token::new(c.to_string(), line);
                tokens.push(token);
            }
        }
    }
    
    for token in tokens {
        if token.error != "" {
            return_code = 65;
        }
        token.output();
    }
    println!("EOF  null");
    
    return return_code;
}