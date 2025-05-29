pub use streaming_iterator::StreamingIterator;

pub(crate) mod utils;

use utils::{USizeStreamingIterator, VecStreamingIterator};

pub trait Element: PartialEq {}

impl Element for usize {}
impl Element for Vec<usize> {}

pub trait Set: Clone {
    type Element: Element;

    fn size(&self) -> usize;

    #[allow(clippy::iter_not_returning_iterator)]
    fn iter(&self) -> impl StreamingIterator<Item = Self::Element>;
}

#[derive(Clone)]
pub struct Basic {
    pub size: usize,
}

impl Basic {
    pub fn new(size: usize) -> Self {
        Self { size }
    }
}

impl Set for Basic {
    type Element = usize;

    fn size(&self) -> usize {
        self.size
    }

    fn iter(&self) -> impl StreamingIterator<Item = Self::Element> {
        USizeStreamingIterator::new(self.size)
    }
}

#[derive(Clone)]
pub struct Product {
    sizes: Vec<usize>,
}

impl Product {
    pub fn new(sets: &[&impl Set]) -> Self {
        let sizes = sets.iter().map(|set| set.size()).collect();
        Self { sizes }
    }
}

impl Set for Product {
    type Element = Vec<usize>;

    fn size(&self) -> usize {
        self.sizes.iter().product()
    }

    fn iter(&self) -> impl StreamingIterator<Item = Self::Element> {
        VecStreamingIterator::new(&self.sizes)
    }
}

#[derive(Clone)]
pub struct Hom {
    sizes: Vec<usize>,
}

impl Hom {
    pub fn new(source: &impl Set, target: &impl Set) -> Self {
        let sizes = vec![target.size(); source.size()];
        Self { sizes }
    }
}

impl Set for Hom {
    type Element = Vec<usize>;

    fn size(&self) -> usize {
        self.sizes.iter().product()
    }

    fn iter(&self) -> impl StreamingIterator<Item = Self::Element> {
        VecStreamingIterator::new(&self.sizes)
    }
}
