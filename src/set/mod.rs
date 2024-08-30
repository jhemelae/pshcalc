pub use streaming_iterator::StreamingIterator;

pub mod basic_set;
pub mod product_set;
pub mod hom_set;
pub(crate) mod utils;

pub trait Set<'set> {
    type Element: Element<'set>;

    fn size(&self) -> usize;

    #[allow(clippy::iter_not_returning_iterator)]
    fn iter(&'set self) -> impl StreamingIterator<
        Item = Self::Element,
    >;
}

pub trait Element<'set> { }

impl<'set> Element<'set> for usize { }
impl<'set> Element<'set> for Vec<usize> { }
