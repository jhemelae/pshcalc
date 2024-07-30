use streaming_iterator::StreamingIterator;

use crate::set::Set;
use crate::set::Element;
use crate::set::utils::advance_array_iterator;
use crate::set::product_set::ProductSet;
use crate::set::product_set::Tuple;

pub struct Function<'set> {
    pub entries: Vec<usize>,
    pub set: &'set HomSet,
}

impl<'set> Element for Function<'set> {
    fn index(&self) -> usize {
        let underlying_tuple = Tuple { 
            entries: self.entries, 
            set: &self.set.underlying_product_set
        };
        underlying_tuple.index()
    }
}


pub struct HomSet {
    underlying_product_set: ProductSet,
}

impl<'source, 'target> HomSet {
    pub fn new(
        source: &impl Set,
        target: &impl Set,
    ) -> Self {
        let source_size = source.size();
        let target_size = target.size();
        let sizes = vec![target_size; source_size];
        let underlying_product_set = ProductSet::from_sizes(sizes);
        Self {
            underlying_product_set,
        }
    }
}
