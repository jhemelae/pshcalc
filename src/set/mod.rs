pub use streaming_iterator::StreamingIterator;

pub mod product_set;
pub mod hom_set;
pub(crate) mod utils;

pub trait Set<'set> {
    type Element: Element<'set>;

    fn size(&self) -> usize;

    fn iter(&'set self) -> impl StreamingIterator<
        Item = Self::Element,
    >;
}

pub trait Element<'set> {
    fn index(&self) -> usize;
}
