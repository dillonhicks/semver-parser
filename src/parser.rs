use crate::{Error, Lexer, Result, Source, Span, Spanned, Token};
use std::fmt;
pub type Lookahead = crate::deps::smallvec::SmallVec<[Token; 8]>;

pub trait Parse: 'static + Sized + fmt::Debug {
    fn parse(parser: &mut Parser) -> Result<Self>;
}

pub trait Peek: Sized {
    fn peek(tokens: &[Token]) -> bool;
}

pub struct Parser {
    lexer: Lexer,
    lookahead: Lookahead,
}

impl Parser {
    pub fn new(source: Source) -> Self {
        let mut parser = Parser {
            lexer: Lexer::new(source),
            lookahead: Lookahead::default(),
        };

        parser.refill_lookahead();
        parser
    }

    fn refill_lookahead(&mut self) {
        while self.lookahead.len() < self.lookahead.capacity() {
            match self.lexer.next_token() {
                Some(tk) => {
                    self.lookahead.push(tk);
                    self.lexer.advance(tk.span.len());
                }
                None => break,
            };
        }
    }

    fn next_token_nofail(&mut self) -> Option<Token> {
        if self.lookahead.is_empty() {
            return None;
        }

        self.lookahead.rotate_left(1);
        let token = self.lookahead.pop();
        self.refill_lookahead();

        token
    }

    pub fn next_token(&mut self) -> Result<Token> {
        self.next_token_nofail().ok_or_else(|| Error::Eof)
    }

    pub fn peek_token(&self) -> Option<Token> {
        self.lookahead.first().copied()
    }

    pub fn peek_token_eof(&self) -> Result<Token> {
        self.lookahead.first().copied().ok_or_else(|| Error::Eof)
    }

    pub fn parse<T: Parse>(&mut self) -> Result<T> {
        T::parse(self)
    }

    pub fn parse_all<T: Parse>(&mut self) -> Result<T> {
        T::parse(self).and_then(|value| {
            if self.lookahead.is_empty() {
                Ok(value)
            } else {
                Err(Error::Incomplete {
                    value: Box::new(value),
                    next: self.lookahead.to_vec().into_boxed_slice(),
                })
            }
        })
    }

    pub fn peek<T: Peek>(&self) -> bool {
        T::peek(&self.lookahead[..])
    }
}

impl<L, R> Parse for (L, R)
where
    L: Parse,
    R: Parse,
{
    fn parse(parser: &mut Parser) -> Result<Self> {
        parser
            .parse::<L>()
            .and_then(|l| parser.parse::<R>().map(|r| (l, r)))
    }
}

impl<L, R> Peek for (L, R)
where
    L: Peek,
{
    fn peek(tokens: &[Token]) -> bool {
        L::peek(tokens)
    }
}

impl<L, R> Spanned for (L, R)
where
    L: Spanned,
    R: Spanned,
{
    fn span(&self) -> Span {
        self.0.span().join(self.1.span())
    }
}

impl<T> Parse for Option<T>
where
    T: Parse + Peek,
{
    fn parse(parser: &mut Parser) -> Result<Self> {
        if parser.peek::<T>() {
            Ok(Some(parser.parse::<T>()?))
        } else {
            Ok(None)
        }
    }
}

impl<T> Spanned for Option<T>
where
    T: Spanned,
{
    fn span(&self) -> Span {
        self.as_ref().map(|s| s.span()).unwrap_or_default()
    }
}
