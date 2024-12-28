
use std::any::Any;
use super::Visitor;

pub trait Element {
    fn accept(&self, visitor: &mut dyn Visitor);
    fn as_any(&self) -> &dyn Any;
}

