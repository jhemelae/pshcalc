pub use streaming_iterator::StreamingIterator;

pub(crate) mod utils;

use utils::{
    USizeStreamingIterator,
    VecStreamingIterator,
};

pub trait Element<'set> { }

impl<'set> Element<'set> for usize { }
impl<'set> Element<'set> for Vec<usize> { }

pub trait Set<'set> {
    type Element: Element<'set>;

    fn size(&self) -> usize;

    #[allow(clippy::iter_not_returning_iterator)]
    fn iter(&'set self) -> impl StreamingIterator<
        Item = Self::Element,
    >;
}

pub struct Basic {
    pub size: usize,
}

impl Basic {
    pub fn new(size: usize) -> Self {
        Self {
            size,
        }
    }
}

impl<'set> Set<'set> for Basic {
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

pub struct Product {
    sizes: Vec<usize>,
}

impl<'set> Product {
    pub fn new(sets: &[&impl Set<'set>]) -> Self {
        let sizes = sets
            .iter()
            .map(|set| set.size())
            .collect();
        Self { sizes }
    }
}

impl<'set> Set<'set> for Product {
    type Element = Vec<usize>;

    fn size(&self) -> usize {
        self.sizes.iter().product()
    }

    fn iter(&'set self) -> impl StreamingIterator<Item = Self::Element> {
        VecStreamingIterator::new(&self.sizes)
    }
}

pub struct Hom {
    sizes: Vec<usize>,
}

impl<'set> Hom {
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

impl<'set> Set<'set> for Hom {
    type Element = Vec<usize>;

    fn size(&self) -> usize {
        self.sizes.iter().product()
    }

    fn iter(&'set self) -> impl StreamingIterator<Item = Self::Element> {
        VecStreamingIterator::new(&self.sizes)
    }
}
