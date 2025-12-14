pub struct Variable<T> {
    value: T,
    ongoing: bool,
}

impl<T> Variable<T> {
    #[inline(always)]
    pub fn uninitialized(value: T) -> Self {
        Variable {
            value,
            ongoing: false,
        }
    }

    #[inline(always)]
    pub fn get_uninitialized(&self) -> &T {
        &self.value
    }

    #[inline(always)]
    pub fn get_current(&self) -> Option<&T> {
        match self.ongoing {
            true => Some(&self.value),
            false => None,
        }
    }

    #[inline(always)]
    pub fn advance<S: Set<T>>(&mut self, set: &S) {
        self.ongoing = set.next(&mut self.value) && self.ongoing;
    }

    #[inline(always)]
    pub fn initialize<S>(&mut self, set: &S)
    where
        S: Set<T>,
    {
        self.ongoing = set.reset(&mut self.value);
    }
}

pub trait Set<T> {
    fn allocate(&self) -> Variable<T>;
    fn reset<'a>(&self, current: &'a mut T) -> bool;
    fn next<'a>(&self, current: &'a mut T) -> bool;
}

#[macro_export]
macro_rules! cursor {
    ($x:tt in $iter:expr => { $($body:tt)* }) => {{
        let mut __element = $iter.allocate();
        __element.initialize($iter);
        while let Some(__data) = __element.get_current() {
            let $x = __data;
            $($body)*
            __element.advance($iter);
        }
    }};
}

#[macro_export]
macro_rules! traverse {
    ($var:tt in $iter:expr => { $($body:tt)* }) => {{
        $var.initialize($iter);
        while let Some(__element) = $var.get_current() {
            {
                let $var = __element;
                { let _ = $var; } // allow unused variable
                $($body)*
            }
            $var.advance($iter);
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
        Variable::uninitialized(0)
    }

    #[inline(always)]
    fn next<'a>(&self, current: &'a mut usize) -> bool {
        *current += 1;
        if *current < self.size {
            true
        } else {
            false
        }
    }

    #[inline(always)]
    fn reset<'a>(&self, current: &'a mut usize) -> bool {
        *current = 0;
        if *current < self.size {
            true
        } else {
            false
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BinaryProductSet {
    left: usize,
    right: usize,
}

impl BinaryProductSet {
    pub fn new(left: &AtomSet, right: &AtomSet) -> Self {
        Self {
            left: left.size(),
            right: right.size(),
        }
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
        Variable::uninitialized(vec![0; self.sizes.len()])
    }

    #[inline(always)]
    fn next<'a>(&self, current: &'a mut Vec<usize>) -> bool {
        for i in 0..self.sizes.len() {
            current[i] += 1;
            if current[i] < self.sizes[i] {
                return true;
            } else {
                current[i] = 0;
            }
        }
        false
    }

    #[inline(always)]
    fn reset<'a>(&self, current: &'a mut Vec<usize>) -> bool {
        for i in 0..self.sizes.len() {
            current[i] = 0;
        }
        true
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
        Variable::uninitialized(vec![0; self.domain_size])
    }

    #[inline(always)]
    fn next<'a>(&self, current: &'a mut Vec<usize>) -> bool {
        for i in 0..self.domain_size {
            current[i] += 1;
            if current[i] < self.target_size {
                return true;
            } else {
                current[i] = 0;
            }
        }
        false
    }

    #[inline(always)]
    fn reset<'a>(&self, current: &'a mut Vec<usize>) -> bool {
        for i in 0..self.domain_size {
            current[i] = 0;
        }
        true
    }
}
