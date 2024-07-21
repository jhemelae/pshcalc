pub use streaming_iterator::StreamingIterator;

pub mod product_set;
pub mod hom_set;

pub trait Set<T> {
    fn iter(&self) -> impl StreamingIterator<Item = T>;
}
