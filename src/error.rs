use crate::deps::thiserror;
use crate::{Kind, Token};
use std::fmt;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("end of file")]
    Eof,
    #[error("runtime: {err}")]
    Runtime { err: Box<dyn std::error::Error> },
    #[error("unexpected token: expected `{expected:?}` got `{actual:?}`")]
    UnexpectedToken { expected: Kind, actual: Kind },
    #[error("parsed value `{value:?}` but tokens still reaming: `{next:?}`  ")]
    Incomplete {
        value: Box<dyn fmt::Debug>,
        next: Box<[Token]>,
    },
}

impl From<Box<dyn std::error::Error>> for Error {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        Error::Runtime { err }
    }
}
