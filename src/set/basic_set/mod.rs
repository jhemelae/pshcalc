use streaming_iterator::StreamingIterator;

use crate::set::Set;
use crate::set::Element;
use crate::set::utils::IteratorState;

pub struct Index<'set> {
    pub index: usize,
    pub set: &'set BasicSet,
}

impl<'set> Index<'set> {
    fn new(set: &'set BasicSet) -> Self {
        Self {
            index: 0,
            set,
        }
    }
}

impl<'set> Element<'set> for Index<'set> {
    fn index(&self) -> usize {
        self.index
    }
}

struct USizeStreamingIterator<'set> {
    state: IteratorState,
    element: Index<'set>,
}

impl<'set> USizeStreamingIterator<'set> {
    pub fn new(set: &'set BasicSet) -> Self {
        let element = Index::new(set);
        Self {
            state: IteratorState::Start,
            element,
        }
    }
}

impl<'set> StreamingIterator for USizeStreamingIterator<'set> {
    type Item = Index<'set>;

    #[inline(always)]
    fn advance(&mut self) {
        match self.state {
            IteratorState::Start => {
                self.state = IteratorState::Running;
            }
            IteratorState::Running => {
                self.element.index += 1;
                if self.element.index == self.element.set.size {
                    self.state = IteratorState::End;
                }
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

pub struct BasicSet {
    pub size: usize,
}

impl BasicSet {
    pub fn new(size: usize) -> Self {
        Self {
            size,
        }
    }
}

impl<'set> Set<'set> for BasicSet {
    type Element = Index<'set>;

    fn size(&self) -> usize {
        self.size
    }

    fn iter(&'set self) -> impl StreamingIterator<
        Item = Self::Element,
    > {
        USizeStreamingIterator::new(self)
    }
}
