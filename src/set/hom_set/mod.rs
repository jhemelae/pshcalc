use streaming_iterator::StreamingIterator;

use crate::set::Set;
use crate::set::product_set::ProductSet;
use crate::set::product_set::TupleStreamingIterator;
use crate::set::product_set::Tuple;

pub struct Function<
    'source,
    'target,
    SourceSet,
    TargetSet,
>
where
    SourceSet: Set<'source, >,
    TargetSet: Set<'target, >,
{
    pub source: &'source SourceSet,
    pub target: &'target TargetSet,
    pub entries: Vec<usize>,
}

pub struct HomSet {
    underlying_product_set: ProductSet,
}

impl HomSet {
    pub fn new(source_size: usize, target_size: usize) -> Self {
        let sizes = vec![target_size; source_size];
        let underlying_product_set = ProductSet::new(sizes);
        Self {
            underlying_product_set,
        }
    }
}

impl<'set> Set<'set> for HomSet {
    type Element = Tuple<'set>;

    fn size(&self) -> usize {
        self.underlying_product_set.size()
    }

    #[inline(always)]
    fn iter(&'set self) -> impl StreamingIterator<Item = Self::Element> {
        TupleStreamingIterator::new(&self.underlying_product_set)
    }
}
