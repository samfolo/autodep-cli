use std::fmt::Debug;

use crate::errors;

pub trait Parser<T>
where
    T: Clone + Debug + PartialEq,
{
    fn load_and_parse(&self, raw_file_path: &str) -> Result<T, errors::ParseError>;
}
