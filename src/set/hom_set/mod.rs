use streaming_iterator::StreamingIterator;
use crate::set::Set;
use crate::set::utils::VecStreamingIterator;

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
    type Element = Vec<usize>;

    fn size(&self) -> usize {
        self.sizes.iter().product()
    }

    #[inline(always)]
    fn iter(&'set self) -> impl StreamingIterator<Item = Self::Element> {
        VecStreamingIterator::new(&self.sizes)
    }
}
