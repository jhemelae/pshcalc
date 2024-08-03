pub(crate) enum IteratorState {
    Start,
    Running,
    End,
}

pub(crate) trait Ticker {
    fn advance(&mut self);
}

pub(crate) struct IntTicker<'a> {
    state: &'a mut IteratorState,
    int: &'a mut usize,
    size: &'a usize,
}

impl<'a> IntTicker<'a> {
    pub fn new(
        state: &'a mut IteratorState,
        int: &'a mut usize,
        size: &'a usize
    ) -> Self {
        Self {
            state,
            int,
            size,
        }
    }
}

impl<'a> Ticker for IntTicker<'a> {
    #[inline(always)]
    fn advance(&mut self) {
        match self.state {
            IteratorState::Start => {
                *self.state = IteratorState::Running;
            }
            IteratorState::Running => {
                *self.int += 1;
                if *self.int == *self.size {
                    // Reset the counter
                    // (to be consistent with the array ticker behavior)
                    *self.int = 0;
                    *self.state = IteratorState::End;
                }
            }
            IteratorState::End => {}
        }
    }
}

pub(crate) struct ArrayTicker<'a> {
    state: &'a mut IteratorState,
    array: &'a mut Vec<usize>,
    sizes: &'a Vec<usize>,
}

impl<'a> ArrayTicker<'a> {
    pub fn new(
        state: &'a mut IteratorState,
        array: &'a mut Vec<usize>,
        sizes: &'a Vec<usize>
    ) -> Self {
        Self {
            state,
            array,
            sizes,
        }
    }
}

impl<'a> Ticker for ArrayTicker<'a> {
    #[inline(always)]
    fn advance(&mut self) {
        match self.state {
            IteratorState::Start => {
                *self.state = IteratorState::Running;
            }
            IteratorState::Running => {
                for (i, entry) 
                in self.array.iter_mut().enumerate() {
                    *entry += 1;
                    if *entry == self.sizes[i] {
                        *entry = 0;
                    } else {
                        return;
                    }
                }
                *self.state = IteratorState::End;
            }
            IteratorState::End => {}
        }
    }
}