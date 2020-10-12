use crate::{ast, Parse, Parser, Peek, Result, Span, Spanned, Token};

#[derive(Clone, Debug)]
pub struct Punctuated<T, V> {
    pub first: V,
    pub rest: ast::Sequence<(T, V)>,
}

impl<T, V> Parse for Punctuated<T, V>
where
    T: Parse + Peek,
    V: Parse + Peek,
{
    fn parse(parser: &mut Parser) -> Result<Self> {
        Ok(Punctuated {
            first: parser.parse()?,
            rest: parser.parse()?,
        })
    }
}

impl<T, V> Spanned for Punctuated<T, V>
where
    T: Spanned,
    V: Spanned,
{
    fn span(&self) -> Span {
        self.first.span().join(self.rest.span())
    }
}

impl<T, V> Peek for Punctuated<T, V>
where
    T: Peek,
    V: Peek,
{
    fn peek(tokens: &[Token]) -> bool {
        V::peek(tokens)
    }
}
