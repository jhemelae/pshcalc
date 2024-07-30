use streaming_iterator::StreamingIterator;
use crate::set::Set;
use crate::set::Element;
use crate::set::utils::advance_array_iterator;
use crate::set::utils::IteratorState;

pub struct Tuple<'a> {
    pub entries: &'a Vec<usize>,
    pub set: &'a ProductSet,
}

impl<'a> Element for Tuple<'a> {
    fn index(&self) -> usize {
        let mut index = 0;
        let mut factor = 1;
        // Convention:
        // the first element of the tuple is the least significant
        // (little-endian)
        for i in 0..self.entries.len() {
            index += self.entries[i] * factor;
            factor *= self.set.sizes[i];
        }
        index
    }
}

pub struct TupleStreamingIterator<'a> {
    state: IteratorState,
    entries: Vec<usize>,
    set: &'a ProductSet,
}

impl<'a> TupleStreamingIterator<'a> {
    pub fn new(set: &'a ProductSet) -> Self {
        let entries = vec![0; set.sizes.len()];
        Self { 
            state: IteratorState::Start,
            entries,
            set,
        }
    }
}

impl<'a> StreamingIterator for TupleStreamingIterator<'a> {
    type Item = Tuple<'a>;

    fn advance(&mut self) {
        advance_array_iterator(
            &mut self.state,
            &mut self.entries,
            &self.set.sizes,
        );
    }

    fn get(&self) -> Option<&Self::Item> {
        match self.state {
            IteratorState::Running => {
                match self.entries {
                    ref entries => Some(Tuple { 
                        entries,
                        set: &self.set,
                    }).as_ref(),
                }
            }
            _ => None,
        }
    }
}


pub struct ProductSet {
    sizes: Vec<usize>,
}

impl<'set> ProductSet {
    pub fn new(sets: &[&impl Set]) -> Self {
        let sizes = sets
            .iter()
            .map(|set| set.size())
            .collect();
        Self { sizes }
    }

    pub(crate) fn from_sizes(sizes: Vec<usize>) -> Self {
        Self { sizes }
    }
}

impl Set for ProductSet {
    fn size(&self) -> usize {
        self.sizes.iter().product()
    }

    fn iter(&self) -> impl StreamingIterator {
        TupleStreamingIterator::new(self)
    }
}
