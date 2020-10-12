macro_rules! define_token {
    ($($name:ident,)+) => {
        $(
            define_token!(@imp $name);
        )+

        define_token!(@any $($name,)+);
    };
    (@imp $name:ident) => {
        #[derive(Copy, Clone, Debug)]
        #[repr(transparent)]
        pub struct $name(crate::Token);

        impl crate::Parse for $name {
            fn parse(parser: &mut crate::Parser) -> crate::Result<Self> {
                let t1 = parser.peek_token_eof()?;
                if matches!(t1.kind, crate::Kind::$name) {
                    Ok($name(parser.next_token()?))
                } else {
                    Err(crate::Error::UnexpectedToken {expected: crate::Kind::$name, actual: t1.kind } )?
                }
            }
        }

        impl crate::Peek for $name {
            fn peek(tokens: &[crate::Token]) -> bool {
                if let Some(t1) = tokens.get(0) {
                    matches!(t1.kind, crate::Kind::$name)
                } else {
                    false
                }
            }
        }

        impl crate::Spanned for $name {
            fn span(&self) -> crate::Span {
                self.0.span
            }
        }
    };
    (@any $($name:ident,)+) => {
        #[derive(Copy, Clone, Debug)]
        pub enum Wildcard {
            $(
                $name($name),
            )+
        }

         impl crate::Parse for Wildcard {
            fn parse(parser: &mut crate::Parser) -> crate::Result<Self> {
                let t1 = parser.peek_token_eof()?;
                match t1.kind {
                    $(
                        crate::Kind::$name => Ok(Wildcard::$name(parser.parse()?)),
                    )+
                }
            }
        }

        impl crate::Peek for Wildcard {
            fn peek(tokens: &[crate::Token]) -> bool {
                tokens.get(0).is_some()
            }
        }

        impl crate::Spanned for Wildcard {
            fn span(&self) -> crate::Span {
                match self {
                    $(
                        Self::$name(t)=> t.span(),
                    )+
                }
            }
        }

    }

}

define_token!(Ident, Dot, Dash, Plus,);
