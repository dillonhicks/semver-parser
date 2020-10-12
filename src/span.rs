pub trait Spanned {
    fn span(&self) -> Span;
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

impl Span {
    pub const fn new(start: u32, end: u32) -> Self {
        Span { start, end }
    }

    pub const fn join(&self, other: Span) -> Self {
        if other.is_empty() {
            *self
        } else {
            Span {
                start: self.start,
                end: other.end,
            }
        }
    }

    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub const fn len(&self) -> usize {
        (self.end - self.start) as usize
    }

    pub const fn range(&self) -> std::ops::Range<usize> {
        (self.start as usize)..(self.end as usize)
    }
}
