pub use streaming_iterator::StreamingIterator;

pub mod basic_set;
pub mod product_set;
pub mod hom_set;
pub(crate) mod utils;

pub trait Set {
    type Element: Element;

    fn size(&self) -> usize;

    fn iter(&self) -> impl StreamingIterator<
        Item = Self::Element,
    >;
}

pub trait Element {
    fn index(&self) -> usize;
}
