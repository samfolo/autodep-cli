use std::fmt::Debug;

pub trait Visitor<A>
where
    A: Clone + Debug + PartialEq,
{
    fn visit(&mut self, ast: &A);
}
