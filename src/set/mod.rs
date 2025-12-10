pub trait Cursor<T> {
    fn next(&mut self) -> Option<&T>;
}

#[macro_export] // needed for macros used outside this module
macro_rules! cursor {
    ($pat:pat in $expr:tt { $($body:tt)* }) => {{
        let mut __c = $expr.cursor();
        while let Some($pat) = __c.next() {
            { $($body)* }
        }
    }};
}

pub trait Set<T> {
    fn cursor(&self) -> impl Cursor<T>;
    fn get_next<'a>(&self, current: &'a mut Option<T>) -> &'a Option<T>;
}

pub struct BasicCursor<S: Set<T>, T> {
    current: Option<T>,
    set: S,
}

impl<S: Set<T>, T> BasicCursor<S, T> {
    pub fn new(set: S) -> Self {
        Self { current: None, set }
    }
}

impl<S: Set<T>, T> Cursor<T> for BasicCursor<S, T> {
    fn next(&mut self) -> Option<&T> {
        self.set.get_next(&mut self.current).as_ref()
    }
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
    fn cursor(&self) -> impl Cursor<usize> {
        BasicCursor::new(self.clone())
    }

    #[inline(always)]
    fn get_next<'a>(
        &self,
        current: &'a mut Option<usize>,
    ) -> &'a Option<usize> {
        match current {
            Some(atom) => {
                *atom += 1;
                if *atom < self.size {
                    current
                } else {
                    *current = None;
                    current
                }
            }
            None => {
                *current = Some(0);
                current
            }
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
    fn cursor(&self) -> impl Cursor<Vec<usize>> {
        BasicCursor::new(self.clone())
    }

    #[inline(always)]
    fn get_next<'a>(
        &self,
        current: &'a mut Option<Vec<usize>>,
    ) -> &'a Option<Vec<usize>> {
        if let Some(tuple) = current {
            for i in 0..self.sizes.len() {
                tuple[i] += 1;
                if tuple[i] < self.sizes[i] {
                    return current;
                } else {
                    tuple[i] = 0;
                }
            }
            *current = None;
            current
        } else {
            *current = Some(vec![0; self.sizes.len()]);
            current
        }
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
    fn cursor(&self) -> impl Cursor<Vec<usize>> {
        BasicCursor::new(self.clone())
    }

    #[inline(always)]
    fn get_next<'a>(
        &self,
        current: &'a mut Option<Vec<usize>>,
    ) -> &'a Option<Vec<usize>> {
        if let Some(function) = current {
            for i in 0..self.domain_size {
                function[i] += 1;
                if function[i] < self.target_size {
                    return current;
                } else {
                    function[i] = 0;
                }
            }
            *current = None;
            current
        } else {
            *current = Some(vec![0; self.domain_size]);
            current
        }
    }
}
