use std::time::Instant;

use streaming_iterator::StreamingIterator;

enum IteratorState {
    Start,
    Running,
    End,
}

struct TupleStreamingIterator {
    state: IteratorState,
    current: Vec<usize>,
    sizes: Vec<usize>,
}

impl TupleStreamingIterator {
    fn new(sizes: Vec<usize>) -> Self {
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

    fn get(&self) -> Option<&Self::Item> {
        match self.state {
            IteratorState::Start => None,
            IteratorState::Running => Some(&self.current),
            IteratorState::End => None,
        }
    }
}

struct ProductSet {
    sizes: Vec<usize>,
}

impl ProductSet {
    fn new(sizes: Vec<usize>) -> Self {
        Self { sizes }
    }

    fn iter(&self) -> TupleStreamingIterator {
        TupleStreamingIterator::new(self.sizes.clone())
    }
}

struct HomSet {
    source_size: usize,
    target_size: usize,
}

impl HomSet {
    fn new(source_size: usize, target_size: usize) -> Self {
        Self {
            source_size,
            target_size,
        }
    }

    fn iter(&self) -> TupleStreamingIterator {
        let sizes = vec![self.target_size; self.source_size];
        TupleStreamingIterator::new(sizes)
    }
}



#[inline(always)]
fn get(s: &[usize], n: usize, i: usize, j: usize) -> usize {
        s[n * i + j]
}

#[inline(always)]
fn is_associative(s: &[usize], n: usize) -> bool {
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                let left = get(s, n, get(s, n, i, j), k);
                let right = get(s, n, i, get(s, n, j, k));
                if left != right {
                    return false;
                }
            }
        }
    }
    true
}

fn main() {
    let start = Instant::now();
    let n: usize = 4;

    let mut multiplication = HomSet::new(
        n*n,
        n
    ).iter();

    let mut count = 0;
    while let Some(array) = multiplication.next() {
        if is_associative(&array, n) {
            count += 1;
        }
    }
    println!("Count = {:?}", count);

    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}
