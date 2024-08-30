use streaming_iterator::StreamingIterator;
use crate::set::Set;
use crate::set::utils::VecStreamingIterator;


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
    type Element = Vec<usize>;

    fn size(&self) -> usize {
        self.sizes.iter().product()
    }

    fn iter(&'set self) -> impl StreamingIterator<Item = Self::Element> {
        VecStreamingIterator::new(&self.sizes)
    }
}
