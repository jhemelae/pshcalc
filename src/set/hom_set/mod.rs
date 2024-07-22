use streaming_iterator::StreamingIterator;

use crate::set::Set;
use crate::set::product_set::TupleStreamingIterator;
use crate::set::product_set::Tuple;

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

impl Set<Tuple> for HomSet {
    #[inline(always)]
    fn iter(&self) -> impl StreamingIterator<Item = Tuple> {
        let sizes = vec![self.target_size; self.source_size];
        TupleStreamingIterator::new(sizes)
    }
}
