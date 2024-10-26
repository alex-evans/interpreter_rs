
pub struct Token {
    text: String,
    token_type: String,
    literal: String,
    line: i32,
    error: bool,
}

impl Token {
    pub fn new(input_text: String) -> Token {
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
            _ => "UNKNOWN".to_string(),
        };
        let text = input_text;
        let literal = "null".to_string();
        let line = 1;
        let mut error = false;
        if token_type == "UNKNOWN" {
            error = true;
        }
        Token {
            text,
            token_type,
            literal,
            line,
            error,
        }
    }

    pub fn output(&self) {
        if self.error {
            eprintln!("[line {}] Error: Unexpected character: {}", self.line, self.text);
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
    
    let mut chars = file_contents.chars().peekable();

    while let Some(current_char) = chars.next() {
        match (current_char, chars.peek()) {
            ('=', Some(&'=')) => {
                let token = Token::new("==".to_string());
                tokens.push(token);
                chars.next();
            },
            (c, _) => {
                let token = Token::new(c.to_string());
                tokens.push(token);
            }
        }
    }
    
    for token in tokens {
        if token.error {
            return_code = 65;
        }
        token.output();
    }
    println!("EOF  null");
    
    return return_code;
}