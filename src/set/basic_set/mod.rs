use streaming_iterator::StreamingIterator;

use crate::set::{
    Set,
    Element,
};
use crate::set::utils::{
    IteratorState,
    IntTicker,
    Ticker,
};

pub struct Index<'set> {
    pub index: usize,
    pub set: &'set BasicSet,
}

impl<'set> Index<'set> {
    fn new(set: &'set BasicSet) -> Self {
        Self {
            index: 0,
            set,
        }
    }
}

impl<'set> Element<'set> for Index<'set> {
    fn index(&self) -> usize {
        self.index
    }
}

struct IndexStreamingIterator<'set> {
    state: IteratorState,
    element: Index<'set>,
}

impl<'set> IndexStreamingIterator<'set> {
    pub fn new(set: &'set BasicSet) -> Self {
        let element = Index::new(set);
        Self {
            state: IteratorState::Start,
            element,
        }
    }
}

impl<'set> StreamingIterator for IndexStreamingIterator<'set> {
    type Item = Index<'set>;

    #[inline(always)]
    fn advance(&mut self) {
        let mut ticker = IntTicker::new(
            &mut self.state,
            &mut self.element.index,
            &self.element.set.size,
        );
        ticker.advance();
    }

    #[inline(always)]
    fn get(&self) -> Option<&Self::Item> {
        match self.state {
            IteratorState::Running => Some(&self.element),
            _ => None,
        }
    }
}

pub struct BasicSet {
    pub size: usize,
}

impl BasicSet {
    pub fn new(size: usize) -> Self {
        Self {
            size,
        }
    }
}

impl<'set> Set<'set> for BasicSet {
    type Element = Index<'set>;

    fn size(&self) -> usize {
        self.size
    }

    fn iter(&'set self) -> impl StreamingIterator<
        Item = Self::Element,
    > {
        IndexStreamingIterator::new(self)
    }
}
