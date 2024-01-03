use std::fmt::Debug;

use crate::parsers::errors;

pub trait Parser<T>
where
    T: Clone + Debug + PartialEq,
{
    fn parse(&self, raw_file_path: &str) -> Result<T, errors::ParseError>;
}
