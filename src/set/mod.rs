pub use streaming_iterator::StreamingIterator;

pub mod basic_set;
pub mod product_set;
pub mod hom_set;
pub(crate) mod utils;

pub trait Set {
    fn size(&self) -> usize;

    fn iter(&self) -> impl StreamingIterator;
}

pub trait Element {
    fn index(&self) -> usize;
}
