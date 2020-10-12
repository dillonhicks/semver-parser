use crate::{
    ast::{self, Dot, Ident, Punctuated},
    Parse, Parser, Peek, Result, Span, Spanned, Token,
};

#[derive(Clone, Debug)]
pub struct BuildMeta {
    pub plus: ast::Plus,
    pub parts: ast::Punctuated<ast::Dot, ast::Ident>,
}

impl Parse for BuildMeta {
    fn parse(parser: &mut Parser) -> Result<Self> {
        Ok(Self {
            plus: parser.parse()?,
            parts: parser.parse()?,
        })
    }
}

impl Peek for BuildMeta {
    fn peek(tokens: &[Token]) -> bool {
        match tokens {
            &[] => false,
            [a, rest @ ..] => ast::Plus::peek(&[*a]) && ast::Punctuated::<Dot, Ident>::peek(rest),
        }
    }
}

impl Spanned for BuildMeta {
    fn span(&self) -> Span {
        self.plus.span().join(self.parts.span())
    }
}
