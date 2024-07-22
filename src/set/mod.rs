pub use streaming_iterator::StreamingIterator;

pub mod product_set;
pub mod hom_set;

pub trait Set<'set, T: Element<'set>> {
    fn iter(&'set self) -> impl StreamingIterator<Item = T>;
}

pub trait Element<'set> {
    fn index(&self) -> usize;
}
