use crate::deps::smallvec::SmallVec;
use crate::{Parse, Parser, Peek, Result, Span, Spanned};

#[derive(Clone, Debug)]
pub struct Sequence<T> {
    items: SmallVec<[T; 1]>,
}

impl<T> Default for Sequence<T> {
    fn default() -> Self {
        Self {
            items: SmallVec::new(),
        }
    }
}

impl<T> Parse for Sequence<T>
where
    T: Parse + Peek,
{
    fn parse(parser: &mut Parser) -> Result<Self> {
        let mut seq = Sequence::default();

        while parser.peek::<T>() {
            seq.items.push(parser.parse()?);
        }

        Ok(seq)
    }
}

impl<T> Spanned for Sequence<T>
where
    T: Spanned,
{
    fn span(&self) -> Span {
        match &self.items[..] {
            &[] => Span::default(),
            [a] => a.span(),
            [a, .., b] => a.span().join(b.span()),
        }
    }
}
