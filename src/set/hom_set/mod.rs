use streaming_iterator::StreamingIterator;
use crate::set::{
    Set,
    Element,
};
use crate::set::utils::{
    IteratorState,
    ArrayTicker,
    Ticker,
    little_endian_index,
};

pub struct Function<'set>
{
    pub entries: Vec<usize>,
    pub sizes: &'set Vec<usize>,
}

impl<'set> Function<'set> {
    pub fn new(set: &'set HomSet) -> Self {
        let sizes = &set.sizes;
        let entries = vec![0; sizes.len()];
        Self {
            entries,
            sizes,
        }
    }
}

impl<'set> Element<'set> for Function<'set> {
    fn index(&self) -> usize {
        little_endian_index(&self.entries, self.sizes)
    }
}

pub struct FunctionStreamingIterator<'set> {
    state: IteratorState,
    element: Function<'set>,
}

impl<'set> FunctionStreamingIterator<'set> {
    pub fn new(set: &'set HomSet) -> Self {
        let element = Function::new(set);
        Self {
            state: IteratorState::Start,
            element,
        }
    }
}

impl<'set> StreamingIterator for FunctionStreamingIterator<'set> {
    type Item = Function<'set>;

    #[inline(always)]
    fn advance(&mut self) {
        let mut ticker = ArrayTicker::new(
            &mut self.state,
            &mut self.element.entries,
            self.element.sizes,
        );
        ticker.advance();
    }

    #[inline(always)]
    fn get(&self) -> Option<&Self::Item> {
        match self.state {
            IteratorState::End => None,
            _ => Some(&self.element),
        }
    }
}

pub struct HomSet {
    sizes: Vec<usize>,
}

impl<'set> HomSet {
    pub fn new(
        source: &impl Set<'set>,
        target: &impl Set<'set>,
    ) -> Self {
        let sizes = vec![target.size(); source.size()];
        Self {
            sizes,
        }
    }
}

impl<'set> Set<'set> for HomSet {
    type Element = Function<'set>;

    fn size(&self) -> usize {
        self.sizes.iter().product()
    }

    #[inline(always)]
    fn iter(&'set self) -> impl StreamingIterator<Item = Self::Element> {
        FunctionStreamingIterator::new(self)
    }
}
