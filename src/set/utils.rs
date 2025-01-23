use streaming_iterator::StreamingIterator;

pub(crate) enum IteratorState {
    Start,
    Running,
    End,
}

pub(crate) struct USizeStreamingIterator {
    state: IteratorState,
    element: usize,
    size: usize,
}

impl USizeStreamingIterator {
    pub fn new(size: usize) -> Self {
        Self {
            state: IteratorState::Start,
            element: 0,
            size,
        }
    }
}

impl StreamingIterator for USizeStreamingIterator {
    type Item = usize;

    #[inline(always)]
    fn advance(&mut self) {
        match self.state {
            IteratorState::Start => {
                self.state = IteratorState::Running;
            }
            IteratorState::Running => {
                self.element += 1;
                if self.element == self.size {
                    // Reset the counter
                    // (to be consistent with the array behavior)
                    self.element = 0;
                    self.state = IteratorState::End;
                }
            }
            IteratorState::End => {}
        }
    }

    #[inline(always)]
    fn get(&self) -> Option<&Self::Item> {
        match self.state {
            IteratorState::Running => Some(&self.element),
            _ => None,
        }
    }
}

pub struct VecStreamingIterator<'set> {
    state: IteratorState,
    entries: Vec<usize>,
    sizes: &'set Vec<usize>,
}

impl<'set> VecStreamingIterator<'set> {
    pub fn new(sizes: &'set Vec<usize>) -> Self {
        let entries = vec![0; sizes.len()];
        Self {
            state: IteratorState::Start,
            entries,
            sizes,
        }
    }
}

impl<'set> StreamingIterator for VecStreamingIterator<'set> {
    type Item = Vec<usize>;

    #[inline(always)]
    fn advance(&mut self) {
        match self.state {
            IteratorState::Start => {
                self.state = IteratorState::Running;
            }
            IteratorState::Running => {
                for (i, entry) in self.entries.iter_mut().enumerate() {
                    *entry += 1;
                    if *entry == self.sizes[i] {
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
            IteratorState::End => None,
            _ => Some(&self.entries),
        }
    }
}

// #[inline(always)]
// pub(crate) fn little_endian_index(
//     entries: &[usize],
//     sizes: &[usize]
// ) -> usize {
//     let mut index = 0;
//     let mut factor = 1;
//     // Convention:
//     // the first element of the tuple is the least significant
//     // (little-endian)
//     for i in 0..entries.len() {
//         index += entries[i] * factor;
//         factor *= sizes[i];
//     }
//     index
// }
