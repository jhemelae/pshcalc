pub use streaming_iterator::StreamingIterator;

pub mod product_set;
pub mod hom_set;

pub trait Set<T: Element> {
    fn iter(&self) -> impl StreamingIterator<Item = T>;
}

pub trait Element {
    fn index(&self) -> usize;
}