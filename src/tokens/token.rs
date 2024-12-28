
use super::{
    Element, 
    KeywordToken,
    IdentifierToken,
    NumberToken,
    PunctuatorToken,
    StringToken,
    Visitor,
};
use std::any::Any;

#[derive(Clone)]
#[derive(Debug)]
pub enum Token {
    Keyword(KeywordToken),
    Identifier(IdentifierToken),
    Number(NumberToken),
    Punctuator(PunctuatorToken),
    String(StringToken),
}

impl Element for Token {
    fn accept(&self, visitor: &mut dyn Visitor) {
        match self {
            Token::Keyword(token) => token.accept(visitor),
            Token::Identifier(token) => token.accept(visitor),
            Token::Number(token) => token.accept(visitor),
            Token::Punctuator(token) => token.accept(visitor),
            Token::String(token) => token.accept(visitor),
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

}
