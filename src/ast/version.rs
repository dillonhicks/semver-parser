use crate::{
    ast::{self, Dot, Ident, Pair, Plus},
    Parse, Parser, Result,
};

#[derive(Clone, Debug)]
pub struct Version {
    pub major: Ident,
    pub minor: Option<Pair<Dot, Ident>>,
    pub patch: Option<Pair<Dot, Ident>>,
    pub pre: Option<ast::Pre>,
    pub build_meta: Option<ast::BuildMeta>,
}

impl Parse for Version {
    fn parse(parser: &mut Parser) -> Result<Self> {
        Ok(Version {
            major: parser.parse()?,
            minor: parser.parse()?,
            patch: parser.parse()?,
            pre: parser.parse()?,
            build_meta: parser.parse()?,
        })
    }
}
