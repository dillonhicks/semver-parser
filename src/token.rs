use crate::Span;

#[derive(Copy, Clone, Debug)]
pub enum Kind {
    Plus,
    Dot,
    Dash,
    Ident,
}

#[derive(Copy, Clone, Debug)]
pub struct Token {
    pub kind: Kind,
    pub span: Span,
}
