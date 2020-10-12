use crate::{Kind, Source, Span, Token};

pub struct Lexer {
    source: Source,
    pos: usize,
}

impl Lexer {
    pub const fn new(source: Source) -> Self {
        Self { source, pos: 0 }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        let mut it = self.source.iter().skip(self.pos);
        let (kind, len): (Kind, usize) = loop {
            let b = if let Some(b) = it.next() {
                b
            } else {
                return None;
            };
            match b {
                b'.' => {
                    break (Kind::Dot, 1);
                }
                b'+' => {
                    break (Kind::Plus, 1);
                }
                b'-' => {
                    break (Kind::Dash, 1);
                }
                ch => {
                    assert!(
                        ch.is_ascii_alphanumeric(),
                        "unexpected char {:?}",
                        char::from(*ch)
                    );
                    let len = it.clone().take_while(|b| b.is_ascii_alphanumeric()).count();
                    for _ in 0..len {
                        it.next();
                    }
                    break (Kind::Ident, len + 1);
                }
            }
        };

        Some(Token {
            kind,
            span: Span::new(self.pos as u32, (self.pos + len) as u32),
        })
    }
    pub fn advance(&mut self, len: usize) {
        self.pos += len
    }
}
