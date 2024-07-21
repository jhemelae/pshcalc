use streaming_iterator::StreamingIterator;

use crate::set::Set;
use crate::set::product_set::TupleStreamingIterator;

pub struct HomSet {
    source_size: usize,
    target_size: usize,
}

impl HomSet {
    pub fn new(source_size: usize, target_size: usize) -> Self {
        Self {
            source_size,
            target_size,
        }
    }
}

impl Set<Vec<usize>> for HomSet {
    fn iter(&self) -> impl StreamingIterator<Item = Vec<usize>> {
        let sizes = vec![self.target_size; self.source_size];
        TupleStreamingIterator::new(sizes)
    }
}
