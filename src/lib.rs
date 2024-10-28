
pub struct Token {
    text: String,
    token_type: String,
    literal: String,
    line: i32,
    error: String,
}

impl Token {
    pub fn new(input_text: String, line: i32) -> Token {
        
        let error;
        let text = input_text.clone();
        let literal ;
        let token_type;

        let token_type = if input_text.starts_with("\"") {
            (token_type, literal, error) = Self::handle_string(input_text);
            token_type
        } else if input_text.chars().next().unwrap().is_digit(10) {
            (token_type, literal, error) = Self::handle_number(input_text);
            token_type
        } else {
            (token_type, literal, error) = Self::handle_keyword(input_text);
            token_type
        };

        Token {
            text,
            token_type,
            literal,
            line,
            error,
        }
    }

    fn handle_string(
        input_text: String
    ) -> (String, String, String) {
        let literal;
        let error;
        let token_type;

        if input_text.len() > 1 && input_text.ends_with("\"") {
            literal = input_text[1..input_text.len()-1].to_string();  // remove the quotes
            error = "".to_string();
            token_type = "STRING".to_string();
        } else {
            literal = "null".to_string();
            error = "Unterminated string.".to_string();
            token_type = "UNKNOWN".to_string();
        }

        (token_type, literal, error)
    }

    fn handle_number(
        input_text: String
    ) -> (String, String, String) {

        let literal;
        let error;
        let token_type;

        let parts: Vec<&str> = input_text.split('.').collect();

        if parts.len() == 1 {
            literal = format!("{}.0", input_text);
            token_type = "NUMBER".to_string();
            error = "".to_string();
            return (token_type, literal, error);
        }

        if parts.len() > 2 {
            literal = "null".to_string();
            error = "Invalid number.".to_string();
            token_type = "UNKNOWN".to_string();
            return (token_type, literal, error);
        }

        let integer_part = parts[0];
        let decimal_part = parts[1].trim_end_matches('0');

        if decimal_part.is_empty() {
            literal = format!("{}.0", integer_part);
            error = "".to_string();
            token_type = "NUMBER".to_string();
            return (token_type, literal, error);
        }

        literal = format!("{}.{}", integer_part, decimal_part);
        error = "".to_string();
        token_type = "NUMBER".to_string();
        return (token_type, literal, error);
    
    }

    fn handle_keyword(
        input_text: String
    ) -> (String, String, String) {
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
                let token: Token = Token::new(number, line);
                tokens.push(token);
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