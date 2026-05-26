pub struct Variable<T> {
    pub value: T,
    pub ongoing: bool,
}


pub trait Set<T> {
    fn allocate(&self) -> Variable<T>;
    fn reset(&self, current: &mut Variable<T>);
    fn next(&self, current: &mut Variable<T>);
    fn size(&self) -> usize {
        let mut var = self.allocate();
        let mut count = 0;
        self.reset(&mut var);
        while var.ongoing {
            self.next(&mut var);
            count += 1;
        }
        count
    }
}

#[macro_export]
macro_rules! cursor {
    ($var:tt in $iter:expr => { $($body:tt)* }) => {{
        let mut __element = $iter.allocate();
        $iter.reset(&mut __element);
        while __element.ongoing {
            let $var = &__element.value;
            { let _ = $var; } // allow unused variable
            $($body)*
            $iter.next(&mut __element);
        }
    }};
}


#[derive(Clone)]
pub struct AtomSet {
    size: usize,
}

impl AtomSet {
    #[inline(always)]
    pub fn new(size: usize) -> Self {
        Self { size }
    }

    #[inline(always)]
    pub fn size(&self) -> usize {
        self.size
    }
}

impl Set<usize> for AtomSet {
    #[inline(always)]
    fn allocate(&self) -> Variable<usize> {
        Variable {
           value: 0,
           ongoing: false
        } 
    }

    #[inline(always)]
    fn next(&self, current: &mut Variable<usize>) {
        current.value += 1;
        current.ongoing = current.value < self.size
    }

    #[inline(always)]
    fn reset(&self, current: &mut Variable<usize>) {
        current.value = 0;
        current.ongoing = self.size > 0;
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProductSet {
    sizes: Vec<usize>,
}

impl ProductSet {
    pub fn new(atom_sets: &[AtomSet]) -> Self {
        let sizes = atom_sets.iter().map(AtomSet::size).collect();
        Self { sizes }
    }

    #[inline(always)]
    pub fn get(&self, value: &[usize]) -> usize {
        let mut index = 0;
        let mut multiplier = 1;
        for i in 0..self.sizes.len() {
            index += value[i] * multiplier;
            multiplier *= self.sizes[i];
        }

        index
    }
}

impl Set<Vec<usize>> for ProductSet {
    #[inline(always)]
    fn allocate(&self) -> Variable<Vec<usize>> {
        Variable {
            value: vec![0; self.sizes.len()],
            ongoing: false
        }
    }

    #[inline(always)]
    fn next(&self, current: &mut Variable<Vec<usize>>) {
        for i in 0..self.sizes.len() {
            current.value[i] += 1;
            if current.value[i] < self.sizes[i] {
                current.ongoing = true;
                return;
            } else {
                current.value[i] = 0;
            }
        }
        current.ongoing = false;
    }

    #[inline(always)]
    fn reset(&self, current: &mut Variable<Vec<usize>>) {
        for i in 0..self.sizes.len() {
            current.value[i] = 0;
        }
        current.ongoing = true;
    }
}

impl From<ProductSet> for AtomSet {
    fn from(product_set: ProductSet) -> Self {
        let size = product_set.sizes.iter().product();
        AtomSet::new(size)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct HomSet {
    domain_size: usize,
    target_size: usize,
}

impl HomSet {
    #[inline(always)]
    pub fn new(source: &AtomSet, target: &AtomSet) -> Self {
        Self {
            domain_size: source.size(),
            target_size: target.size(),
        }
    }

    #[inline(always)]
    #[allow(dead_code)]
    pub fn get(&self, value: &[usize]) -> usize {
        let mut index = 0;
        let mut multiplier = 1;
        for &img in value {
            index += img * multiplier;
            multiplier *= self.target_size;
        }
        index
    }
}

impl Set<Vec<usize>> for HomSet {
    #[inline(always)]
    fn allocate(&self) -> Variable<Vec<usize>> {
        Variable {
            value: vec![0; self.domain_size],
            ongoing: false
        }
    }

    #[inline(always)]
    fn next(&self, current: &mut Variable<Vec<usize>>) {
        for i in 0..self.domain_size {
            current.value[i] += 1;
            if current.value[i] < self.target_size {
                current.ongoing = true;
                return;
            } else {
                current.value[i] = 0;
            }
        }
        current.ongoing = false;
    }

    #[inline(always)]
    fn reset(&self, current: &mut Variable<Vec<usize>>) {
        for i in 0..self.domain_size {
            current.value[i] = 0;
        }
        current.ongoing = true
    }
}
