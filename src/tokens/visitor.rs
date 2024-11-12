
use super::{
    IdentifierToken, 
    KeywordToken, 
    NumberToken, 
    PunctuatorToken, 
    StringToken
};

pub trait Visitor {
    fn visit_string_token(&mut self, token: &StringToken);
    fn visit_number_token(&mut self, token: &NumberToken);
    fn visit_keyword_token(&mut self, token: &KeywordToken);
    fn visit_identifier_token(&mut self, token: &IdentifierToken);
    fn visit_punctuator_token(&mut self, token: &PunctuatorToken);
}