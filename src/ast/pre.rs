use crate::{
    ast::{self, Dot, Ident, Punctuated},
    Parse, Parser, Peek, Result, Span, Spanned, Token,
};

#[derive(Clone, Debug)]
pub struct Pre {
    pub dash: ast::Dash,
    pub parts: ast::Punctuated<ast::Dot, ast::Ident>,
}

impl Parse for Pre {
    fn parse(parser: &mut Parser) -> Result<Self> {
        Ok(Self {
            dash: parser.parse()?,
            parts: parser.parse()?,
        })
    }
}

impl Peek for Pre {
    fn peek(tokens: &[Token]) -> bool {
        match tokens {
            &[] => false,
            [a, rest @ ..] => ast::Dash::peek(&[*a]) && ast::Punctuated::<Dot, Ident>::peek(rest),
        }
    }
}

impl Spanned for Pre {
    fn span(&self) -> Span {
        self.dash.span().join(self.parts.span())
    }
}
