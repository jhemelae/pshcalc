use streaming_iterator::StreamingIterator;
use crate::set::{
    Set,
    Element,
};
use crate::set::utils::{
    IteratorState,
    ArrayTicker,
    Ticker,
};

pub struct Tuple<'set> {
    pub entries: Vec<usize>,
    pub sizes: &'set Vec<usize>,
}

impl<'set> Tuple<'set> {
    fn new(set: &'set ProductSet) -> Self {
        let sizes = &set.sizes;
        let entries = vec![0; sizes.len()];
        Self { 
            entries,
            sizes,
        }
    }
}

impl<'set> Element<'set> for Tuple<'set> {
    fn index(&self) -> usize {
        let mut index = 0;
        let mut factor = 1;
        // Convention:
        // the first element of the tuple is the least significant
        // (little-endian)
        for i in 0..self.entries.len() {
            index += self.entries[i] * factor;
            factor *= self.sizes[i];
        }
        index
    }
}

pub struct TupleStreamingIterator<'set> {
    state: IteratorState,
    element: Tuple<'set>,
}

impl<'set> TupleStreamingIterator<'set> {
    pub fn new(set: &'set ProductSet) -> Self {
        let element = Tuple::new(set);
        Self {
            state: IteratorState::Start,
            element,
        }
    }
}

impl<'set> StreamingIterator for TupleStreamingIterator<'set> {
    type Item = Tuple<'set>;

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
            IteratorState::Running => Some(&self.element),
            _ => None,
        }
    }
}

pub struct ProductSet {
    sizes: Vec<usize>,
}

impl<'set> ProductSet {
    pub fn new(sets: &[&impl Set<'set>]) -> Self {
        let sizes = sets
            .iter()
            .map(|set| set.size())
            .collect();
        Self { sizes }
    }
}

impl<'set> Set<'set> for ProductSet {
    type Element = Tuple<'set>;

    fn size(&self) -> usize {
        self.sizes.iter().product()
    }

    fn iter(&'set self) -> impl StreamingIterator<Item = Self::Element> {
        TupleStreamingIterator::new(self)
    }
}
