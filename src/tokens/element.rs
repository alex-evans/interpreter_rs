use super::Visitor;

pub trait Element {
    fn accept(&self, visitor: &mut dyn Visitor);
}