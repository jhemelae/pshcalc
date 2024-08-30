use streaming_iterator::StreamingIterator;

use crate::set::Set;
use crate::set::utils::USizeStreamingIterator;

pub struct BasicSet {
    pub size: usize,
}

impl BasicSet {
    pub fn new(size: usize) -> Self {
        Self {
            size,
        }
    }
}

impl<'set> Set<'set> for BasicSet {
    type Element = usize;

    fn size(&self) -> usize {
        self.size
    }

    fn iter(&'set self) -> impl StreamingIterator<
        Item = Self::Element,
    > {
        USizeStreamingIterator::new(self.size)
    }
}
