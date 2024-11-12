use super::{
    Visitor, 
    StringToken, 
    NumberToken, 
    KeywordToken, 
    IdentifierToken,
    PunctuatorToken,
};

pub struct OutputVisitor {
    pub return_code: i32,
}

impl OutputVisitor {
    pub fn new() -> Self {
        OutputVisitor { return_code: 0 }
    }
}

impl Visitor for OutputVisitor {
    fn visit_string_token(&mut self, token: &StringToken) {
        if !token.error.is_empty() {
            eprintln!("[line {}] Error: {}", token.line, token.error);
            self.return_code = 65; // Set error code
        } else {
            println!("STRING {} {}", token.text, token.literal);
        }
    }

    fn visit_number_token(&mut self, token: &NumberToken) {
        if !token.error.is_empty() {
            eprintln!("[line {}] Error: {}", token.line, token.error);
            self.return_code = 65; // Set error code
        } else {
            println!("NUMBER {} {}", token.text, token.literal);
        }
    }

    fn visit_keyword_token(&mut self, token: &KeywordToken) {
        if !token.error.is_empty() {
            eprintln!("[line {}] Error: {}", token.line, token.error);
            self.return_code = 65; // Set error code
        } else {
            println!("{} {} {}", token.token_type, token.text, token.literal);
        }
    }

    fn visit_identifier_token(&mut self, token: &IdentifierToken) {
        if !token.error.is_empty() {
            eprintln!("[line {}] Error: {}", token.line, token.error);
            self.return_code = 65; // Set error code
        } else {
            println!("IDENTIFIER {} {}", token.text, token.literal);
        }
    }

    fn visit_punctuator_token(&mut self, token: &PunctuatorToken) {
        if !token.error.is_empty() {
            eprintln!("[line {}] Error: {}", token.line, token.error);
            self.return_code = 65; // Set error code
        } else {
            println!("{} {} {}", token.token_type, token.text, token.literal);
        }
    }
}