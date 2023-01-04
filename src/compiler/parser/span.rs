use std::{
    ops::Range,
    fmt,
};
use chumsky::Span;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct TSpan {
    src: usize,
    range: (usize, usize),
}

impl TSpan {
    #[cfg(test)]
    pub fn empty() -> Self {
        Self::new(SrcId::empty(), 0..0)
    }

    pub fn src(&self) -> usize { self.src }

    pub fn range(&self) -> Range<usize> { self.start()..self.end() }

    pub fn union(self, other: Self) -> Self {
        assert_eq!(self.src, other.src, "attempted to union spans with different sources");
        Self {
            range: (self.start().min(other.start()), self.end().max(other.end())),
            ..self
        }
    }
}

impl fmt::Debug for TSpan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}:{:?}", self.src, self.range())
    }
}

impl chumsky::Span for TSpan {
    type Context = usize;
    type Offset = usize;

    fn new(src: usize, range: Range<usize>) -> Self {
        assert!(range.start <= range.end);
        Self { src, range: (range.start, range.end) }
    }

    fn context(&self) -> usize { self.src }
    fn start(&self) -> Self::Offset { self.range.0 }
    fn end(&self) -> Self::Offset { self.range.1 }
}

impl ariadne::Span for TSpan {
    type SourceId = usize;

    fn source(&self) -> &usize { &self.src }

    fn start(&self) -> usize { self.range.0 }
    fn end(&self) -> usize { self.range.1 }
}