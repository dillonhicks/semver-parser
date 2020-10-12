use crate::deps::bytes::Bytes;
use crate::{Span, Spanned};
use std::ops::RangeBounds;

#[derive(Clone)]
pub struct Source {
    bytes: Bytes,
}

impl Source {
    pub fn from_str(s: &str) -> Self {
        Self {
            bytes: Bytes::copy_from_slice(s.as_bytes()),
        }
    }

    pub fn slice(&self, range: impl RangeBounds<usize>) -> Bytes {
        self.bytes.slice(range)
    }

    pub fn iter(&self) -> std::slice::Iter<'_, u8> {
        self.bytes.iter()
    }

    pub fn text<S>(&self, spanned: &S) -> std::borrow::Cow<'_, str>
    where
        S: Spanned,
    {
        let span = spanned.span();
        String::from_utf8_lossy(&self.bytes.as_ref()[span.range()])
    }
}

impl Spanned for Source {
    fn span(&self) -> Span {
        Span::new(0, self.bytes.len() as u32)
    }
}
