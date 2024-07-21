use streaming_iterator::StreamingIterator;
use crate::set::Set;

enum IteratorState {
    Start,
    Running,
    End,
}

pub struct TupleStreamingIterator {
    state: IteratorState,
    current: Vec<usize>,
    sizes: Vec<usize>,
}

impl TupleStreamingIterator {
    pub fn new(sizes: Vec<usize>) -> Self {
        let current = vec![0; sizes.len()];
        Self {
            state: IteratorState::Start,
            current,
            sizes,
        }
    }
}

impl StreamingIterator for TupleStreamingIterator {
    type Item = Vec<usize>;

    #[inline(always)]
    fn advance(&mut self) {
        match self.state {
            IteratorState::Start => {
                self.state = IteratorState::Running;
            }
            IteratorState::Running => {
                let array = &mut self.current;
                for i in 0..array.len() {
                    array[i] += 1;
                    if array[i] == self.sizes[i] {
                        array[i] = 0;
                    } else {
                        return;
                    }
                }
                self.state = IteratorState::End;
            }
            IteratorState::End => {}
        }
    }

    #[inline(always)]
    fn get(&self) -> Option<&Self::Item> {
        match self.state {
            IteratorState::Start => None,
            IteratorState::Running => Some(&self.current),
            IteratorState::End => None,
        }
    }
}

pub struct ProductSet {
    sizes: Vec<usize>,
}

impl ProductSet {
    pub fn new(sizes: Vec<usize>) -> Self {
        Self { sizes }
    }
}

impl Set<Vec<usize>> for ProductSet {
    fn iter(&self) -> impl StreamingIterator<Item = Vec<usize>> {
        TupleStreamingIterator::new(self.sizes.clone())
    }
}