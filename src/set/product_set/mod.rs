use streaming_iterator::StreamingIterator;
use crate::set::Set;
use crate::set::Element;
use crate::set::utils::IteratorState;

pub struct Tuple<'set> {
    pub entries: Vec<usize>,
    pub set: &'set ProductSet,
}

impl<'set> Tuple<'set> {
    fn new(set: &'set ProductSet) -> Self {
        let sizes = &set.sizes;
        let entries = vec![0; sizes.len()];
        Self { 
            entries,
            set,
        }
    }
}

impl<'set> Element<'set> for Tuple<'set> {
    fn index(&self) -> usize {
        let mut index = 0;
        let mut factor = 1;
        // Convention:
        // the first element of the tuple is the least significant
        // (little-endian)
        for i in 0..self.entries.len() {
            index += self.entries[i] * factor;
            factor *= self.set.sizes[i];
        }
        index
    }
}

pub struct TupleStreamingIterator<'set> {
    state: IteratorState,
    element: Tuple<'set>,
}

impl<'set> TupleStreamingIterator<'set> {
    pub fn new(set: &'set ProductSet) -> Self {
        let element = Tuple::new(set);
        Self {
            state: IteratorState::Start,
            element,
        }
    }
}

impl<'set> StreamingIterator for TupleStreamingIterator<'set> {
    type Item = Tuple<'set>;

    #[inline(always)]
    fn advance(&mut self) {
        match self.state {
            IteratorState::Start => {
                self.state = IteratorState::Running;
            }
            IteratorState::Running => {
                let array = &mut self.element.entries;
                for (i, entry) in array.iter_mut().enumerate(){
                    *entry += 1;
                    if *entry == self.element.set.sizes[i] {
                        *entry = 0;
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

    pub fn tuple(&self) -> Tuple {
        Tuple::new(self)
    }
}

impl<'set> Set<'set> for ProductSet {
    type Element = Tuple<'set>;

    fn size(&self) -> usize {
        self.sizes.iter().product()
    }

    fn iter(&'set self) -> impl StreamingIterator<Item = Self::Element> {
        TupleStreamingIterator::new(self)
    }
}
