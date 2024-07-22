use streaming_iterator::StreamingIterator;
use crate::set::Set;
use crate::set::Element;

pub struct Tuple {
    pub current: Vec<usize>,
    sizes: Vec<usize>,
}

impl Tuple {
    fn new(sizes: Vec<usize>) -> Self {
        let current = vec![0; sizes.len()];
        Self { 
            current: current,
            sizes: sizes
        }
    }
}

impl Element for Tuple {
    fn index(&self) -> usize {
        let mut index = 0;
        let mut factor = 1;
        // Convention:
        // the first element of the tuple is the least significant
        // (little-endian)
        for i in 0..self.current.len() {
            index += self.current[i] * factor;
            factor *= self.sizes[i];
        }
        index
    }
}

enum IteratorState {
    Start,
    Running,
    End,
}

pub struct TupleStreamingIterator {
    state: IteratorState,
    element: Tuple,
}

impl TupleStreamingIterator {
    pub fn new(sizes: Vec<usize>) -> Self {
        let element = Tuple::new(sizes);
        Self {
            state: IteratorState::Start,
            element,
        }
    }
}

impl StreamingIterator for TupleStreamingIterator {
    type Item = Tuple;

    #[inline(always)]
    fn advance(&mut self) {
        match self.state {
            IteratorState::Start => {
                self.state = IteratorState::Running;
            }
            IteratorState::Running => {
                let array = &mut self.element.current;
                for i in 0..array.len() {
                    array[i] += 1;
                    if array[i] == self.element.sizes[i] {
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
            IteratorState::Running => Some(&self.element),
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

impl Set<Tuple> for ProductSet {
    fn iter(&self) -> impl StreamingIterator<Item = Tuple> {
        TupleStreamingIterator::new(self.sizes.clone())
    }
}
