use crate::{Parse, Parser, Peek, Result, Span, Spanned, Token};

#[derive(Clone, Debug)]
pub struct Pair<T, V> {
    pub left: T,
    pub right: V,
}

impl<T, V> Parse for Pair<T, V>
where
    T: Parse,
    V: Parse,
{
    fn parse(parser: &mut Parser) -> Result<Self> {
        Ok(Pair {
            left: parser.parse()?,
            right: parser.parse()?,
        })
    }
}

impl<T, V> Spanned for Pair<T, V>
where
    T: Spanned,
    V: Spanned,
{
    fn span(&self) -> Span {
        self.left.span().join(self.right.span())
    }
}

impl<T, V> Peek for Pair<T, V>
where
    T: Peek,
    V: Peek,
{
    fn peek(tokens: &[Token]) -> bool {
        match tokens {
            [tk, rest @ ..] => T::peek(&[*tk]) && V::peek(rest),
            &[] => false,
        }
    }
}
