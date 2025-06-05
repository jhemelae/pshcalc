// ECS-style World struct holding all data in contiguous arrays
#[derive(Debug)]
pub struct World {
    indices: Vec<usize>,
    sizes: Vec<usize>,
    next_index_slot: usize,
    next_size_slot: usize,
}

impl World {
    pub fn new() -> Self {
        Self {
            indices: Vec::new(),
            sizes: Vec::new(),
            next_index_slot: 0,
            next_size_slot: 0,
        }
    }

    pub fn alloc_indices(&mut self, count: usize) -> usize {
        let start = self.next_index_slot;
        self.indices.resize(self.next_index_slot + count, 0);
        self.next_index_slot += count;
        start
    }

    pub fn alloc_sizes(&mut self, count: usize) -> usize {
        let start = self.next_size_slot;
        self.sizes.resize(self.next_size_slot + count, 0);
        self.next_size_slot += count;
        start
    }

    #[inline(always)]
    pub fn get_indices(&self, start: usize, len: usize) -> &[usize] {
        &self.indices[start..start + len]
    }

    #[inline(always)]
    pub fn get_indices_mut(
        &mut self,
        start: usize,
        len: usize,
    ) -> &mut [usize] {
        &mut self.indices[start..start + len]
    }

    #[inline(always)]
    pub fn get_sizes(&self, start: usize, len: usize) -> &[usize] {
        &self.sizes[start..start + len]
    }

    #[inline(always)]
    pub fn get_sizes_mut(&mut self, start: usize, len: usize) -> &mut [usize] {
        &mut self.sizes[start..start + len]
    }

    #[inline(always)]
    pub fn set_index(&mut self, position: usize, value: usize) {
        self.indices[position] = value;
    }

    #[inline(always)]
    pub fn get_index(&self, position: usize) -> usize {
        self.indices[position]
    }
}

// Element trait for type safety
pub trait Element: PartialEq + Clone + std::fmt::Debug {}

// LinearIndexable trait provides a common interface for converting multi-dimensional indices to linear indices
// This is the core operation needed for efficient function application and array indexing
pub trait LinearIndexable {
    /// Calculate linear index from multi-dimensional indices
    fn to_linear_index(&self, world: &World) -> usize;
}

// Atom handle
#[derive(Clone, Debug, PartialEq)]
pub struct AtomHandle {
    index_pos: usize,
    size: usize,
}

impl Element for AtomHandle {}

impl AtomHandle {
    pub fn new(world: &mut World, size: usize) -> Self {
        let index_pos = world.alloc_indices(1);
        world.set_index(index_pos, 0);
        Self { index_pos, size }
    }

    #[inline(always)]
    pub fn get_index(&self, world: &World) -> usize {
        world.get_index(self.index_pos)
    }

    #[inline(always)]
    pub fn set_index(&self, world: &mut World, value: usize) {
        world.set_index(self.index_pos, value);
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

// Tuple handle
#[derive(Clone, Debug, PartialEq)]
pub struct TupleHandle {
    pub(crate) indices_start: usize,
    pub(crate) sizes_start: usize,
    pub(crate) len: usize,
}

impl Element for TupleHandle {}

impl TupleHandle {
    pub fn new(world: &mut World, sizes: &[usize]) -> Self {
        let len = sizes.len();
        let indices_start = world.alloc_indices(len);
        let sizes_start = world.alloc_sizes(len);

        world.get_indices_mut(indices_start, len).fill(0);
        world.get_sizes_mut(sizes_start, len).copy_from_slice(sizes);

        Self {
            indices_start,
            sizes_start,
            len,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl LinearIndexable for TupleHandle {
    #[inline(always)]
    fn to_linear_index(&self, world: &World) -> usize {
        let indices = world.get_indices(self.indices_start, self.len);
        let sizes = world.get_sizes(self.sizes_start, self.len);
        calculate_linear_index(indices, sizes)
    }
}

// Function handle
#[derive(Clone, Debug, PartialEq)]
pub struct FunctionHandle {
    pub(crate) indices_start: usize,
    pub(crate) sizes_start: usize,
    pub(crate) len: usize,
    target_size: usize, // Keep for convenience, but sizes array is the source of truth
}

impl Element for FunctionHandle {}

impl FunctionHandle {
    pub fn new(
        world: &mut World,
        domain_size: usize,
        target_size: usize,
    ) -> Self {
        let len = domain_size;
        let indices_start = world.alloc_indices(len);
        let sizes_start = world.alloc_sizes(len);

        // Initialize indices to 0 and sizes to target_size for each dimension
        world.get_indices_mut(indices_start, len).fill(0);
        world.get_sizes_mut(sizes_start, len).fill(target_size);

        Self {
            indices_start,
            sizes_start,
            len,
            target_size,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    #[inline(always)]
    pub fn get_value(&self, world: &World, position: usize) -> usize {
        world.get_index(self.indices_start + position)
    }

    pub fn domain_size(&self) -> usize {
        self.len
    }

    pub fn target_size(&self) -> usize {
        self.target_size
    }

    // High-performance apply method for any LinearIndexable type
    #[inline(always)]
    pub fn apply<T: LinearIndexable>(&self, world: &World, tuple: &T) -> usize {
        let linear_index = tuple.to_linear_index(world);
        world.get_index(self.indices_start + linear_index)
    }
}

impl LinearIndexable for FunctionHandle {
    #[inline(always)]
    fn to_linear_index(&self, world: &World) -> usize {
        let indices = world.get_indices(self.indices_start, self.len);
        let sizes = world.get_sizes(self.sizes_start, self.len);
        calculate_linear_index(indices, sizes)
    }
}

// Stack-allocated tuple for zero-allocation performance
#[derive(Clone, Debug, PartialEq)]
pub struct StackTuple<const N: usize> {
    indices: [usize; N],
    sizes: [usize; N],
}

impl<const N: usize> StackTuple<N> {
    pub fn new(sizes: [usize; N]) -> Self {
        Self {
            indices: [0; N],
            sizes,
        }
    }

    #[inline(always)]
    pub fn set_index(&mut self, position: usize, value: usize) {
        self.indices[position] = value;
    }

    #[inline(always)]
    pub fn get_indices_array(&self) -> &[usize; N] {
        &self.indices
    }

    #[inline(always)]
    pub fn get_sizes_array(&self) -> &[usize; N] {
        &self.sizes
    }
}

impl<const N: usize> LinearIndexable for StackTuple<N> {
    #[inline(always)]
    fn to_linear_index(&self, _world: &World) -> usize {
        // Optimized implementation that ignores world parameter
        if N == 2 {
            // Optimized path for binary case
            self.indices[0] + self.indices[1] * self.sizes[0]
        } else {
            let mut index = 0;
            let mut multiplier = 1;
            for i in 0..N {
                index += self.indices[i] * multiplier;
                multiplier *= self.sizes[i];
            }
            index
        }
    }
}

// Binary tuple convenience type
pub type BinaryTuple = StackTuple<2>;

impl BinaryTuple {
    #[inline(always)]
    pub fn new_binary(size: usize) -> Self {
        Self::new([size, size])
    }

    #[inline(always)]
    pub fn set(&mut self, i: usize, j: usize) {
        self.indices[0] = i;
        self.indices[1] = j;
    }
}

// Shared counter advance logic
#[inline(always)]
fn advance_counter(data: &mut [usize], limits: &[usize]) -> bool {
    for i in 0..data.len() {
        data[i] += 1;
        if data[i] < limits[i] {
            return false;
        }
        data[i] = 0;
    }
    true
}

pub trait Variable<T: Element> {
    fn initialize(&mut self, world: &mut World);
    fn advance(&mut self, world: &mut World);
    fn get(&self) -> Option<&T>;
}

// AtomSet generates AtomHandles
#[derive(Clone)]
pub struct AtomSet {
    size: usize,
}

impl AtomSet {
    pub fn new(size: usize) -> Self {
        Self { size }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn create_variable(&self, world: &mut World) -> AtomSetVariable {
        AtomSetVariable::new(world, self.size)
    }
}

pub struct AtomSetVariable {
    done: bool,
    current: AtomHandle,
    size: usize,
}

impl AtomSetVariable {
    pub fn new(world: &mut World, size: usize) -> Self {
        Self {
            done: false,
            current: AtomHandle::new(world, size),
            size,
        }
    }
}

impl Variable<AtomHandle> for AtomSetVariable {
    #[inline(always)]
    fn initialize(&mut self, world: &mut World) {
        self.done = false;
        self.current.set_index(world, 0);
    }

    #[inline(always)]
    fn advance(&mut self, world: &mut World) {
        if self.done {
            return;
        }

        let new_index = self.current.get_index(world) + 1;
        if new_index >= self.size {
            self.done = true;
        } else {
            self.current.set_index(world, new_index);
        }
    }

    #[inline(always)]
    fn get(&self) -> Option<&AtomHandle> {
        if self.done {
            None
        } else {
            Some(&self.current)
        }
    }
}

// ProductSet generates TupleHandles
#[derive(Clone, Debug, PartialEq)]
pub struct ProductSet {
    sizes: Vec<usize>,
}

impl ProductSet {
    pub fn new(atom_sets: &[AtomSet]) -> Self {
        let sizes = atom_sets.iter().map(|set| set.size()).collect();
        Self { sizes }
    }

    pub fn size(&self) -> usize {
        self.sizes.iter().product()
    }

    pub fn sizes(&self) -> &[usize] {
        &self.sizes
    }

    pub fn create_variable(&self, world: &mut World) -> ProductSetVariable {
        ProductSetVariable::new(world, &self.sizes)
    }
}

pub struct ProductSetVariable {
    done: bool,
    current: TupleHandle,
    sizes: Vec<usize>,
}

impl ProductSetVariable {
    pub fn new(world: &mut World, sizes: &[usize]) -> Self {
        Self {
            done: false,
            current: TupleHandle::new(world, sizes),
            sizes: sizes.to_vec(),
        }
    }
}

impl Variable<TupleHandle> for ProductSetVariable {
    #[inline(always)]
    fn initialize(&mut self, world: &mut World) {
        self.done = false;
        world
            .get_indices_mut(self.current.indices_start, self.current.len)
            .fill(0);
    }

    #[inline(always)]
    fn advance(&mut self, world: &mut World) {
        if self.done {
            return;
        }

        let indices =
            world.get_indices_mut(self.current.indices_start, self.current.len);
        self.done = advance_counter(indices, &self.sizes);
    }

    #[inline(always)]
    fn get(&self) -> Option<&TupleHandle> {
        if self.done {
            None
        } else {
            Some(&self.current)
        }
    }
}

// HomSet generates FunctionHandles
#[derive(Clone, Debug, PartialEq)]
pub struct HomSet {
    domain_size: usize,
    target_size: usize,
}

impl HomSet {
    pub fn new(source: &ProductSet, target: &AtomSet) -> Self {
        Self {
            domain_size: source.size(),
            target_size: target.size(),
        }
    }

    pub fn size(&self) -> usize {
        self.target_size.pow(self.domain_size as u32)
    }

    pub fn create_variable(&self, world: &mut World) -> HomSetVariable {
        HomSetVariable::new(world, self.domain_size, self.target_size)
    }
}

pub struct HomSetVariable {
    done: bool,
    current: FunctionHandle,
    sizes: Vec<usize>,
}

impl HomSetVariable {
    pub fn new(
        world: &mut World,
        domain_size: usize,
        target_size: usize,
    ) -> Self {
        let sizes = vec![target_size; domain_size];
        Self {
            done: false,
            current: FunctionHandle::new(world, domain_size, target_size),
            sizes,
        }
    }
}

impl Variable<FunctionHandle> for HomSetVariable {
    #[inline(always)]
    fn initialize(&mut self, world: &mut World) {
        self.done = false;
        world
            .get_indices_mut(self.current.indices_start, self.current.len)
            .fill(0);
    }

    #[inline(always)]
    fn advance(&mut self, world: &mut World) {
        if self.done {
            return;
        }

        let indices =
            world.get_indices_mut(self.current.indices_start, self.current.len);
        self.done = advance_counter(indices, &self.sizes);
    }

    #[inline(always)]
    fn get(&self) -> Option<&FunctionHandle> {
        if self.done {
            None
        } else {
            Some(&self.current)
        }
    }
}

// Shared linear indexing implementation for multi-dimensional handles
#[inline(always)]
fn calculate_linear_index(indices: &[usize], sizes: &[usize]) -> usize {
    if indices.len() == 2 {
        // Optimized path for binary case
        indices[0] + indices[1] * sizes[0]
    } else {
        let mut index = 0;
        let mut multiplier = 1;
        for i in 0..indices.len() {
            index += indices[i] * multiplier;
            multiplier *= sizes[i];
        }
        index
    }
}

// Zero-allocation AssociativityChecker using stack tuples
pub struct AssociativityChecker {
    tuple_ij: BinaryTuple,
    tuple_jk: BinaryTuple,
    tuple_fij_k: BinaryTuple,
    tuple_i_fjk: BinaryTuple,
    n: usize,
}

impl AssociativityChecker {
    pub fn new(_world: &mut World, n: usize) -> Self {
        Self {
            tuple_ij: BinaryTuple::new_binary(n),
            tuple_jk: BinaryTuple::new_binary(n),
            tuple_fij_k: BinaryTuple::new_binary(n),
            tuple_i_fjk: BinaryTuple::new_binary(n),
            n,
        }
    }

    #[inline(always)]
    pub fn is_associative(
        &mut self,
        world: &World,
        f: &FunctionHandle,
    ) -> bool {
        for i in 0..self.n {
            for j in 0..self.n {
                for k in 0..self.n {
                    // Using the unified trait interface - clean and performant
                    self.tuple_ij.set(i, j);
                    let f_i_j = f.apply(world, &self.tuple_ij);

                    self.tuple_jk.set(j, k);
                    let f_j_k = f.apply(world, &self.tuple_jk);

                    self.tuple_fij_k.set(f_i_j, k);
                    let left = f.apply(world, &self.tuple_fij_k);

                    self.tuple_i_fjk.set(i, f_j_k);
                    let right = f.apply(world, &self.tuple_i_fjk);

                    if left != right {
                        return false;
                    }
                }
            }
        }
        true
    }
}
