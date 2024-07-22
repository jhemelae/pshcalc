use streaming_iterator::StreamingIterator;

use crate::set::Set;
use crate::set::product_set::ProductSet;
use crate::set::product_set::TupleStreamingIterator;
use crate::set::product_set::Tuple;

pub struct HomSet {
    underlying_product_set: ProductSet,
}

impl<'set> HomSet {
    pub fn new(source_size: usize, target_size: usize) -> Self {
        let sizes = vec![target_size; source_size];
        let underlying_product_set = ProductSet::new(sizes);
        Self {
            underlying_product_set,
        }
    }
}

impl<'set> Set<'set, Tuple<'set>> for HomSet {
    #[inline(always)]
    fn iter(&'set self) -> impl StreamingIterator<Item = Tuple<'set>> {
        TupleStreamingIterator::new(&self.underlying_product_set)
    }
}
