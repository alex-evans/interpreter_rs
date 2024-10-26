

pub struct Token {
    text: String,
    token_type: String,
    literal: String,
}

impl Token {
    pub fn new(input_text: String) -> Token {
        let token_type = match input_text.as_str() {
            "(" => "LEFT_PAREN".to_string(),
            ")" => "RIGHT_PAREN".to_string(),
            "{" => "LEFT_BRACE".to_string(),
            "}" => "RIGHT_BRACE".to_string(),
            _ => "UNKNOWN".to_string(),
        };
        let text = input_text;
        let literal = "null".to_string();
        Token {
            text,
            token_type,
            literal,
        }
    }

    pub fn output(&self) -> String {
        format!("{} {} {}", self.token_type, self.text, self.literal)
    }
}

pub fn run(
    file_contents: String
) {
    let mut tokens: Vec<Token> = Vec::new();
    for c in file_contents.chars() {
        let token = Token::new(c.to_string());
        tokens.push(token);
    }
    for token in tokens {
        println!("{}", token.output());
    }
    println!("EOF  null");
}