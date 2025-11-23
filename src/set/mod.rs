#[derive(Copy, Clone, Debug)]
pub struct DataRange {
    start: usize,
    len: usize,
}

impl DataRange {
    #[inline(always)]
    pub fn new(start: usize, len: usize) -> Self {
        Self { start, len }
    }

    #[inline(always)]
    pub fn start(&self) -> usize {
        self.start
    }
}

impl From<DataRange> for std::ops::Range<usize> {
    #[inline(always)]
    fn from(range: DataRange) -> Self {
        range.start..range.start + range.len
    }
}

#[derive(Debug)]
pub struct World {
    values: Vec<usize>,
    sizes: Vec<usize>,
}

impl World {
    pub fn new() -> Self {
        Self {
            values: Vec::new(),
            sizes: Vec::new(),
        }
    }

    pub fn alloc(&mut self, count: usize) -> DataRange {
        let start = self.values.len();
        self.values.resize(start + count, 0);
        self.sizes.resize(start + count, 0);
        DataRange::new(start, count)
    }

    #[inline(always)]
    pub fn get_values(&self, range: &DataRange) -> &[usize] {
        &self.values[range.start..range.start + range.len]
    }

    #[inline(always)]
    pub fn get_values_mut(&mut self, range: &DataRange) -> &mut [usize] {
        &mut self.values[range.start..range.start + range.len]
    }

    #[inline(always)]
    pub fn get_sizes(&self, range: &DataRange) -> &[usize] {
        &self.sizes[range.start..range.start + range.len]
    }

    #[inline(always)]
    pub fn get_sizes_mut(&mut self, range: &DataRange) -> &mut [usize] {
        &mut self.sizes[range.start..range.start + range.len]
    }

    #[inline(always)]
    pub fn advance_counter(&mut self, range: &DataRange) -> bool {
        for i in range.start..range.start + range.len {
            self.values[i] += 1;
            if self.values[i] < self.sizes[i] {
                return false;
            }
            self.values[i] = 0;
        }
        true
    }
}

pub trait LinearIndexable {
    fn to_linear_index(&self) -> usize;
    fn to_linear_size(&self) -> usize;
}

#[derive(Clone, Debug, PartialEq)]
pub struct Tuple<'a> {
    indices: &'a [usize],
    sizes: &'a [usize],
}

impl<'a> Tuple<'a> {
    pub fn new(indices: &'a [usize], sizes: &'a [usize]) -> Self {
        Self { indices, sizes }
    }
}

impl LinearIndexable for Tuple<'_> {
    #[inline(always)]
    fn to_linear_index(&self) -> usize {
        let mut index = 0;
        let mut multiplier = 1;
        for i in 0..self.indices.len() {
            index += self.indices[i] * multiplier;
            multiplier *= self.sizes[i];
        }
        index
    }

    #[inline(always)]
    fn to_linear_size(&self) -> usize {
        self.sizes.iter().product()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Atom {
    pub index: usize,
    pub size: usize,
}

impl Atom {
    /// Creates a new `Atom` with the specified index and size.
    #[inline(always)]
    pub fn new(index: usize, size: usize) -> Self {
        Self { index, size }
    }
}

impl LinearIndexable for Atom {
    /// For atoms, the linear index is simply the contained index value.
    #[inline(always)]
    fn to_linear_index(&self) -> usize {
        self.index
    }

    #[inline(always)]
    fn to_linear_size(&self) -> usize {
        self.size
    }
}

impl<T, const N: usize> LinearIndexable for [T; N]
where
    T: LinearIndexable,
    T: std::fmt::Debug,
{
    /// Calculate linear index for slices of `Atoms` using row-major order.
    #[inline(always)]
    fn to_linear_index(&self) -> usize {
        if self.is_empty() {
            return 0;
        }

        let mut result = 0;
        let mut multiplier = 1;

        for element in self.iter() {
            result += element.to_linear_index() * multiplier;
            multiplier *= element.to_linear_size();
        }

        result
    }

    #[inline(always)]
    fn to_linear_size(&self) -> usize {
        self.iter().map(|atom| atom.to_linear_size()).product()
    }
}

#[derive(Debug)]
pub struct Function<'a> {
    values: &'a [usize],
    sizes: &'a [usize],
}

impl<'a> Function<'a> {
    /// Creates a new Function wrapper around the given values.
    #[inline(always)]
    pub fn new(values: &'a [usize], sizes: &'a [usize]) -> Self {
        Self { values, sizes }
    }

    #[inline(always)]
    pub fn apply<T: LinearIndexable>(&self, input: &T) -> Atom {
        let linear_index = input.to_linear_index();
        let result_index = self.values[linear_index];
        let result_size = self.sizes[linear_index];
        Atom::new(result_index, result_size)
    }

    #[inline(always)]
    pub fn domain(&self) -> AtomSet {
        AtomSet::new(self.sizes.len())
    }
}

pub trait Set {
    fn size(&self) -> usize;
}

#[derive(Clone)]
pub struct AtomSet {
    size: usize,
}

impl AtomSet {
    pub fn new(size: usize) -> Self {
        Self { size }
    }

    #[inline(always)]
    pub fn iter(&self) -> AtomSetIterator {
        AtomSetIterator {
            current: 0,
            size: self.size,
        }
    }
}

impl Set for AtomSet {
    #[inline(always)]
    fn size(&self) -> usize {
        self.size
    }
}

pub struct AtomSetIterator {
    current: usize,
    size: usize,
}

impl Iterator for AtomSetIterator {
    type Item = Atom;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.size {
            let atom = Atom::new(self.current, self.size);
            self.current += 1;
            Some(atom)
        } else {
            None
        }
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.size - self.current;
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for AtomSetIterator {
    #[inline(always)]
    fn len(&self) -> usize {
        self.size - self.current
    }
}

impl IntoIterator for &AtomSet {
    type Item = Atom;
    type IntoIter = AtomSetIterator;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
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
}

impl Set for ProductSet {
    #[inline(always)]
    fn size(&self) -> usize {
        self.sizes.iter().product()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct HomSet {
    domain_size: usize,
    target_size: usize,
}

impl HomSet {
    /// Creates a new `HomSet` representing functions from source to target.
    #[inline(always)]
    pub fn new<S: Set, T: Set>(source: &S, target: &T) -> Self {
        Self {
            domain_size: source.size(),
            target_size: target.size(),
        }
    }

    pub fn create_variable(&self, world: &mut World) -> HomSetVariable {
        HomSetVariable::new(world, self.domain_size, self.target_size)
    }
}

impl Set for HomSet {
    #[inline(always)]
    fn size(&self) -> usize {
        self.target_size
            .pow(u32::try_from(self.domain_size).unwrap())
    }
}

pub struct HomSetVariable {
    done: bool,
    range: DataRange,
}

impl HomSetVariable {
    /// Creates a new HomSetVariable.
    pub fn new(
        world: &mut World,
        domain_size: usize,
        target_size: usize,
    ) -> Self {
        let range = world.alloc(domain_size);
        let sizes = world.get_sizes_mut(&range);
        for size in sizes.iter_mut() {
            *size = target_size;
        }
        let done = false;

        Self { done, range }
    }

    #[inline(always)]
    pub fn initialize(&mut self, world: &mut World) {
        self.done = false;
        world.get_values_mut(&self.range).fill(0);
    }

    #[inline(always)]
    pub fn advance(&mut self, world: &mut World) {
        if self.done {
            return;
        }

        self.done = world.advance_counter(&self.range);
    }

    #[inline(always)]
    pub fn get<'a>(&self, world: &'a World) -> Option<Function<'a>> {
        if self.done {
            None
        } else {
            let values = world.get_values(&self.range);
            let sizes = world.get_sizes(&self.range);
            Some(Function::new(values, sizes))
        }
    }
}
