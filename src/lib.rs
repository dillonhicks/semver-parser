pub(crate) mod deps {
    pub(crate) use anyhow;
    pub(crate) use bytes;
    pub(crate) use smallvec;
    pub(crate) use thiserror;
}

pub mod ast;
mod error;
mod lexer;
mod parser;
mod source;
mod span;
mod token;

pub type Result<T> = std::result::Result<T, Error>;

pub use self::error::Error;
pub use self::lexer::Lexer;
pub use self::parser::{Parse, Parser, Peek};
pub use self::source::Source;
pub use self::span::{Span, Spanned};
pub use self::token::{Kind, Token};
