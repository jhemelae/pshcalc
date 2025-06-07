//! High-performance set theory and category theory operations.
//!
//! This module provides efficient implementations of fundamental mathematical structures
//! including sets, tuples, functions, and morphisms with optimized memory layout and
//! zero-allocation iteration patterns.
//!
//! # Architecture Overview
//!
//! The module is built around an ECS (Entity-Component-System) inspired design where:
//!
//! - **[`World`]**: Centralized data store with contiguous arrays for indices and sizes
//! - **Handles**: Lightweight references storing only offsets and lengths
//! - **Variables**: Efficient iterators through mathematical sets
//! - **Linear Indexing**: Fast conversion from multi-dimensional to linear indices
//!
//! # Core Design Principles
//!
//! ## 1. Cache Locality
//! All data is stored in contiguous arrays within the [`World`] struct, eliminating
//! pointer chasing and improving memory access patterns.
//!
//! ## 2. Zero-Cost Abstractions  
//! Handle types ([`AtomHandle`], [`TupleHandle`], [`FunctionHandle`]) store only
//! essential metadata, with actual data accessed through the [`World`].
//!
//! ## 3. Compile-Time Optimization
//! Stack-allocated types like [`StackTuple`] and [`BinaryTuple`] enable aggressive
//! compiler optimizations for known-size operations.
//!
//! ## 4. Mathematical Correctness
//! The type system enforces mathematical invariants through traits like [`Element`]
//! and [`LinearIndexable`].
//!
//! # Performance Characteristics
//!
//! - **Memory**: Minimal allocations, excellent locality
//! - **CPU**: Optimized hot paths with specialized binary cases
//! - **Cache**: Contiguous data layout minimizes cache misses
//! - **Scalability**: Efficient for both small and large mathematical structures
//!
//! # Usage Examples
//!
//! ## Basic Set Operations
//!
//! ```
//! use pshcalc::set::{World, AtomSet, Variable};
//!
//! let mut world = World::new();
//! let set = AtomSet::new(3); // {0, 1, 2}
//!
//! let mut var = set.create_variable(&mut world);
//! var.initialize(&mut world);
//! while let Some(atom) = var.get() {
//!     println!("Element: {}", atom.get_index(&world));
//!     var.advance(&mut world);
//! }
//! ```
//!
//! ## Product Sets (Cartesian Products)
//!
//! ```
//! use pshcalc::set::{World, AtomSet, ProductSet, Variable};
//!
//! let mut world = World::new();
//! let a = AtomSet::new(2);
//! let b = AtomSet::new(3);
//! let product = ProductSet::new(&[a, b]); // {0,1} × {0,1,2}
//!
//! assert_eq!(product.size(), 6); // 2 × 3 = 6 elements
//! ```
//!
//! ## Function Application
//!
//! ```
//! use pshcalc::set::{World, FunctionHandle, BinaryTuple};
//!
//! let mut world = World::new();
//! let f = FunctionHandle::new(&mut world, 4, 2); // f: 2×2 → 2
//!
//! // Apply function to (1, 0)
//! let mut input = BinaryTuple::new_binary(2);
//! input.set(1, 0);
//! let result = f.apply(&world, &input);
//! ```
//!

//!
//! # Mathematical Background
//!
//! This module implements core concepts from:
//!
//! - **Set Theory**: Finite sets, Cartesian products, function spaces
//! - **Category Theory**: Objects, morphisms, composition
//! - **Algebra**: Binary operations, associativity, semigroups
//! - **Combinatorics**: Enumeration, counting, systematic generation
//!
//! The implementation is particularly well-suited for:
//!
//! - Enumerating algebraic structures (semigroups, groups, etc.)
//! - Testing mathematical properties (associativity, commutativity, etc.)
//! - High-performance categorical computations
//! - Combinatorial algorithms requiring efficient set operations

/// ECS-style World struct holding all data in contiguous arrays.
///
/// This design pattern centralizes memory management and improves cache locality
/// by storing all indices and sizes in separate, contiguous vectors.
/// Handles store start positions and lengths to access their data slices.
///
/// # Design Philosophy
///
/// The World struct acts as a centralized data store that eliminates pointer
/// chasing and improves memory locality. All handles (`TupleHandle`, `FunctionHandle`)
/// store only offsets and lengths, making them lightweight and copyable.
///
/// # Example
///
/// ```
/// use pshcalc::set::World;
///
/// let mut world = World::new();
/// let start = world.alloc_indices(3);
/// world.get_indices_mut(start, 3).copy_from_slice(&[1, 2, 3]);
/// assert_eq!(world.get_indices(start, 3), &[1, 2, 3]);
/// ```
#[derive(Debug)]
pub struct World {
    indices: Vec<usize>,
    sizes: Vec<usize>,
    next_index_slot: usize,
    next_size_slot: usize,
}

impl World {
    /// Creates a new empty World with no allocated data.
    ///
    /// This is a zero-cost constructor that initializes empty vectors
    /// and resets allocation counters.
    pub fn new() -> Self {
        Self {
            indices: Vec::new(),
            sizes: Vec::new(),
            next_index_slot: 0,
            next_size_slot: 0,
        }
    }

    /// Allocates a contiguous block of indices and returns the starting position.
    ///
    /// This method grows the indices vector to accommodate the requested count
    /// and returns the offset where the new block begins. All new indices are
    /// initialized to zero.
    ///
    /// # Arguments
    ///
    /// * `count` - Number of indices to allocate
    ///
    /// # Returns
    ///
    /// Starting position of the allocated block
    ///
    /// # Example
    ///
    /// ```
    /// # use pshcalc::set::World;
    /// let mut world = World::new();
    /// let start1 = world.alloc_indices(3);
    /// let start2 = world.alloc_indices(2);
    /// assert_eq!(start1, 0);
    /// assert_eq!(start2, 3);
    /// ```
    pub fn alloc_indices(&mut self, count: usize) -> usize {
        let start = self.next_index_slot;
        self.indices.resize(self.next_index_slot + count, 0);
        self.next_index_slot += count;
        start
    }

    /// Allocates a contiguous block of sizes and returns the starting position.
    ///
    /// Similar to `alloc_indices` but for the sizes array. This is used to store
    /// dimension sizes for multi-dimensional handles.
    ///
    /// # Arguments
    ///
    /// * `count` - Number of size slots to allocate
    ///
    /// # Returns
    ///
    /// Starting position of the allocated block
    pub fn alloc_sizes(&mut self, count: usize) -> usize {
        let start = self.next_size_slot;
        self.sizes.resize(self.next_size_slot + count, 0);
        self.next_size_slot += count;
        start
    }

    /// Returns an immutable slice of indices for the specified range.
    ///
    /// # Arguments
    ///
    /// * `start` - Starting position in the indices array
    /// * `len` - Number of indices to include in the slice
    ///
    /// # Panics
    ///
    /// Panics if `start + len` exceeds the length of the indices array.
    #[inline(always)]
    pub fn get_indices(&self, start: usize, len: usize) -> &[usize] {
        &self.indices[start..start + len]
    }

    /// Returns a mutable slice of indices for the specified range.
    ///
    /// # Arguments
    ///
    /// * `start` - Starting position in the indices array
    /// * `len` - Number of indices to include in the slice
    ///
    /// # Panics
    ///
    /// Panics if `start + len` exceeds the length of the indices array.
    #[inline(always)]
    pub fn get_indices_mut(
        &mut self,
        start: usize,
        len: usize,
    ) -> &mut [usize] {
        &mut self.indices[start..start + len]
    }

    /// Returns an immutable slice of sizes for the specified range.
    ///
    /// # Arguments
    ///
    /// * `start` - Starting position in the sizes array
    /// * `len` - Number of sizes to include in the slice
    ///
    /// # Panics
    ///
    /// Panics if `start + len` exceeds the length of the sizes array.
    #[inline(always)]
    pub fn get_sizes(&self, start: usize, len: usize) -> &[usize] {
        &self.sizes[start..start + len]
    }

    /// Returns a mutable slice of sizes for the specified range.
    #[inline(always)]
    pub fn get_sizes_mut(&mut self, start: usize, len: usize) -> &mut [usize] {
        &mut self.sizes[start..start + len]
    }

    /// Sets a single index value at the specified position.
    ///
    /// # Arguments
    ///
    /// * `position` - Absolute position in the indices array
    /// * `value` - New value to store
    ///
    /// # Panics
    ///
    /// Panics if `position` exceeds the length of the indices array.
    #[inline(always)]
    pub fn set_index(&mut self, position: usize, value: usize) {
        self.indices[position] = value;
    }

    /// Gets a single index value at the specified position.
    ///
    /// # Arguments
    ///
    /// * `position` - Absolute position in the indices array
    ///
    /// # Returns
    ///
    /// The value stored at the given position
    ///
    /// # Panics
    ///
    /// Panics if `position` exceeds the length of the indices array.
    #[inline(always)]
    pub fn get_index(&self, position: usize) -> usize {
        self.indices[position]
    }
}

/// Marker trait for elements that can be stored and compared.
///
/// This trait provides type safety by ensuring that only proper mathematical
/// elements can be used in the set theory operations. All elements must support
/// equality comparison, cloning, and debugging.
///
/// # Implementors
///
/// - [`AtomHandle`] - Represents atomic elements
/// - [`TupleHandle`] - Represents tuples/product elements  
/// - [`FunctionHandle`] - Represents functions/morphisms
pub trait Element: PartialEq + Clone + std::fmt::Debug {}

/// Trait providing efficient conversion from multi-dimensional indices to linear indices.
///
/// This trait is the core abstraction for indexing operations in the system.
/// It enables efficient function application and array indexing by converting
/// multi-dimensional coordinates to single linear indices.
///
/// # Performance Notes
///
/// Implementations should be highly optimized as this operation is called
/// frequently in hot loops. The system provides specialized fast paths for
/// binary cases (N=2) which are common in many mathematical contexts.
///
/// # Example
///
/// ```
/// # use pshcalc::set::{World, BinaryTuple, LinearIndexable};
/// let world = World::new();
/// let mut tuple = BinaryTuple::new_binary(3);
/// tuple.set(1, 2);
/// let linear = tuple.to_linear_index(&world);
/// assert_eq!(linear, 1 + 2 * 3); // i + j * size
/// ```
pub trait LinearIndexable {
    /// Calculate linear index from multi-dimensional indices.
    ///
    /// Converts the current multi-dimensional position to a single linear
    /// index suitable for array access or function application.
    ///
    /// # Arguments
    ///
    /// * `world` - Reference to the World containing size and index data
    ///
    /// # Returns
    ///
    /// Linear index corresponding to the current multi-dimensional position
    fn to_linear_index(&self, world: &World) -> usize;
}

// Atom handle - represents a single atomic element in a set
#[derive(Clone, Debug, PartialEq)]
pub struct AtomHandle {
    /// Index position in World's indices array where this atom's current value is stored
    pub(crate) index_position: usize,
}

impl Element for AtomHandle {}

impl AtomHandle {
    /// Creates a new AtomHandle.
    ///
    /// # Arguments
    ///
    /// * `world` - Mutable reference to World for allocation
    /// * `_size` - Size of the set this atom belongs to (for validation)
    ///
    /// # Returns
    ///
    /// New AtomHandle with index initialized to 0
    pub fn new(world: &mut World, _size: usize) -> Self {
        let index_position = world.alloc_indices(1);
        world.set_index(index_position, 0);

        Self { index_position }
    }

    /// Gets the current index value of this atom.
    ///
    /// # Arguments
    ///
    /// * `world` - Reference to World containing the data
    ///
    /// # Returns
    ///
    /// Current index value
    #[inline(always)]
    pub fn get_index(&self, world: &World) -> usize {
        world.get_index(self.index_position)
    }

    /// Sets the current index value of this atom.
    ///
    /// # Arguments
    ///
    /// * `world` - Mutable reference to World containing the data
    /// * `index` - New index value
    #[inline(always)]
    pub fn set_index(&self, world: &mut World, index: usize) {
        world.set_index(self.index_position, index);
    }
}

impl LinearIndexable for AtomHandle {
    /// For atoms, the linear index is simply the atom's current index value.
    ///
    /// This allows AtomHandle to be used directly with function application
    /// via the `apply` method, treating the atom as a 1-dimensional input.
    ///
    /// # Returns
    ///
    /// The current index value of this atom
    #[inline(always)]
    fn to_linear_index(&self, world: &World) -> usize {
        self.get_index(world)
    }
}

// Tuple handle
#[derive(Clone, Debug, PartialEq)]
pub struct TupleHandle {
    /// Starting position in World's indices array
    pub(crate) indices_start: usize,
    /// Starting position in World's sizes array  
    pub(crate) sizes_start: usize,
    /// Number of dimensions in this tuple
    pub(crate) len: usize,
}

impl Element for TupleHandle {}

impl TupleHandle {
    /// Creates a new TupleHandle with the specified dimension sizes.
    ///
    /// Allocates space in both the indices and sizes arrays of the World.
    /// All indices are initialized to 0, and the sizes are copied from
    /// the provided slice.
    ///
    /// # Arguments
    ///
    /// * `world` - Mutable reference to World for allocation
    /// * `sizes` - Slice containing the size of each dimension
    ///
    /// # Returns
    ///
    /// New TupleHandle with all indices initialized to 0
    ///
    /// # Example
    ///
    /// ```
    /// # use pshcalc::set::{World, TupleHandle};
    /// let mut world = World::new();
    /// let tuple = TupleHandle::new(&mut world, &[3, 4]);
    /// assert_eq!(tuple.len(), 2);
    /// ```
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

    /// Returns the number of dimensions in this tuple.
    ///
    /// # Returns
    ///
    /// Number of dimensions (length of the tuple)
    pub fn len(&self) -> usize {
        self.len
    }
}

impl LinearIndexable for TupleHandle {
    /// Optimized linear index calculation with specialized paths for common cases
    #[inline(always)]
    fn to_linear_index(&self, world: &World) -> usize {
        let indices = world.get_indices(self.indices_start, self.len);
        let sizes = world.get_sizes(self.sizes_start, self.len);

        // Specialized optimization for binary tuples (common case)
        if self.len == 2 {
            indices[0] + indices[1] * sizes[0]
        } else {
            calculate_linear_index(indices, sizes)
        }
    }
}

// Function handle
#[derive(Clone, Debug, PartialEq)]
pub struct FunctionHandle {
    /// Starting position of function values in World's indices array
    pub(crate) indices_start: usize,
    /// Starting position of dimension sizes in World's sizes array
    pub(crate) sizes_start: usize,
    /// Size of the domain (number of input elements)
    pub(crate) len: usize,
    /// Size of the codomain (for convenience and validation)
    target_size: usize,
}

impl Element for FunctionHandle {}

impl FunctionHandle {
    /// Creates a new FunctionHandle representing a function f: A → B.
    ///
    /// Allocates space for `domain_size` function values and initializes
    /// all values to 0. Also stores the dimension sizes (all equal to
    /// `target_size` for functions from product sets).
    ///
    /// # Arguments
    ///
    /// * `world` - Mutable reference to World for allocation
    /// * `domain_size` - Number of elements in the domain
    /// * `target_size` - Number of elements in the codomain
    ///
    /// # Returns
    ///
    /// New FunctionHandle with all values initialized to 0
    ///
    /// # Example
    ///
    /// ```
    /// # use pshcalc::set::{World, FunctionHandle};
    /// let mut world = World::new();
    /// let f = FunctionHandle::new(&mut world, 9, 3); // f: 3×3 → 3
    /// assert_eq!(f.domain_size(), 9);
    /// assert_eq!(f.target_size(), 3);
    /// ```
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

    /// Returns the number of dimensions in the domain.
    ///
    /// For functions from product sets, this equals the domain size.
    ///
    /// # Returns
    ///
    /// Number of elements in the domain
    pub fn len(&self) -> usize {
        self.len
    }

    /// Gets the function value at a specific linear position.
    ///
    /// This is a low-level method for direct access to function values.
    /// For evaluation with multi-dimensional inputs, use [`apply`](Self::apply).
    ///
    /// # Arguments
    ///
    /// * `world` - Reference to World containing the data
    /// * `position` - Linear position in the domain (0 <= position < domain_size)
    ///
    /// # Returns
    ///
    /// Function value at the given position
    #[inline(always)]
    pub fn get_value(&self, world: &World, position: usize) -> usize {
        world.get_index(self.indices_start + position)
    }

    /// Sets the function value at a specific linear position.
    ///
    /// This is a low-level method for direct modification of function values.
    /// For most use cases, consider using [`HomSetVariable`] to iterate through
    /// all possible functions systematically.
    ///
    /// # Arguments
    ///
    /// * `world` - Mutable reference to World containing the data
    /// * `position` - Linear position in the domain (0 <= position < domain_size)
    /// * `value` - New function value (should be < target_size)
    ///
    /// # Panics
    ///
    /// Does not validate that position < domain_size or value < target_size.
    /// Callers should ensure validity.
    #[inline(always)]
    pub fn set_value(&self, world: &mut World, position: usize, value: usize) {
        world.set_index(self.indices_start + position, value);
    }

    /// Returns the size of the domain.
    ///
    /// # Returns
    ///
    /// Number of elements in the domain
    pub fn domain_size(&self) -> usize {
        self.len
    }

    /// Returns the size of the codomain.
    ///
    /// # Returns
    ///
    /// Number of elements in the codomain
    pub fn target_size(&self) -> usize {
        self.target_size
    }

    /// Applies the function to a multi-dimensional input.
    ///
    /// This is the primary method for function evaluation. It converts the
    /// input to a linear index using the [`LinearIndexable`] trait and returns
    /// the corresponding function value.
    ///
    /// # Type Parameters
    ///
    /// * `T` - Type implementing [`LinearIndexable`] (e.g., [`TupleHandle`], [`BinaryTuple`])
    ///
    /// # Arguments
    ///
    /// * `world` - Reference to World containing the data
    /// * `tuple` - Multi-dimensional input implementing [`LinearIndexable`]
    ///
    /// # Returns
    ///
    /// Function value f(tuple)
    ///
    /// # Example
    ///
    /// ```
    /// # use pshcalc::set::{World, FunctionHandle, BinaryTuple};
    /// let mut world = World::new();
    /// let f = FunctionHandle::new(&mut world, 4, 2);
    ///
    /// let mut input = BinaryTuple::new_binary(2);
    /// input.set(1, 0);
    /// let result = f.apply(&world, &input);
    /// ```
    #[inline(always)]
    pub fn apply<T: LinearIndexable>(
        &self,
        world: &World,
        tuple: &T,
    ) -> StackAtom {
        let linear_index = tuple.to_linear_index(world);
        let result_index = world.get_index(self.indices_start + linear_index);
        StackAtom::new(result_index, self.target_size)
    }
}

impl LinearIndexable for FunctionHandle {
    /// Optimized linear index calculation with specialized paths for common cases
    #[inline(always)]
    fn to_linear_index(&self, world: &World) -> usize {
        let indices = world.get_indices(self.indices_start, self.len);
        let sizes = world.get_sizes(self.sizes_start, self.len);

        // Specialized optimization for binary functions (common case)
        if self.len == 2 {
            indices[0] + indices[1] * sizes[0]
        } else {
            calculate_linear_index(indices, sizes)
        }
    }
}

/// Stack-allocated tuple for zero-allocation performance.
///
/// `StackTuple<N>` is a compile-time sized tuple that stores its indices and sizes
/// directly in the struct rather than in the World's arrays. This provides zero-allocation
/// performance for operations that need temporary tuples.
///
/// # Performance Benefits
///
/// - **Zero allocation**: No heap allocation or World interaction needed
/// - **Stack locality**: All data stored directly in the struct
/// - **Compile-time optimization**: Size known at compile time enables better optimization
/// - **Fast indexing**: Specialized implementations for common cases (N=2)
///
/// # Type Parameters
///
/// * `N` - Number of dimensions (compile-time constant)
///
/// # Example
///
/// ```
/// # use pshcalc::set::{StackTuple, LinearIndexable, World};
/// let world = World::new();
/// let mut tuple = StackTuple::<3>::new([4, 5, 6]);
/// tuple.set_index(1, 3);
///
/// let linear = tuple.to_linear_index(&world);
/// // Calculates: 0 + 3*4 + 0*4*5 = 12
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct StackTuple<const N: usize> {
    indices: [usize; N],
    sizes: [usize; N],
}

impl<const N: usize> StackTuple<N> {
    /// Creates a new StackTuple with the specified dimension sizes.
    ///
    /// All indices are initialized to 0.
    ///
    /// # Arguments
    ///
    /// * `sizes` - Array containing the size of each dimension
    ///
    /// # Returns
    ///
    /// New StackTuple with all indices set to 0
    pub fn new(sizes: [usize; N]) -> Self {
        Self {
            indices: [0; N],
            sizes,
        }
    }

    /// Sets the index at a specific position.
    ///
    /// # Arguments
    ///
    /// * `position` - Dimension to modify (0 <= position < N)
    /// * `value` - New index value
    ///
    /// # Panics
    ///
    /// Panics if position >= N
    #[inline(always)]
    pub fn set_index(&mut self, position: usize, value: usize) {
        self.indices[position] = value;
    }

    /// Returns a reference to the indices array.
    ///
    /// # Returns
    ///
    /// Reference to the N-element indices array
    #[inline(always)]
    pub fn get_indices_array(&self) -> &[usize; N] {
        &self.indices
    }

    /// Returns a reference to the sizes array.
    ///
    /// # Returns
    ///
    /// Reference to the N-element sizes array
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

/// Convenience type alias for binary (2-dimensional) tuples.
///
/// `BinaryTuple` is the most common case of [`StackTuple`] and represents
/// elements of Cartesian products A × B. It provides specialized methods
/// for efficient manipulation of 2D coordinates.
///
/// # Performance
///
/// Binary tuples have highly optimized linear indexing using the formula:
/// `linear_index = i + j * size_i`
///
/// # Example
///
/// ```
/// # use pshcalc::set::{BinaryTuple, LinearIndexable, World};
/// let world = World::new();
/// let mut tuple = BinaryTuple::new_binary(5);
/// tuple.set(2, 3);
///
/// let linear = tuple.to_linear_index(&world);
/// assert_eq!(linear, 2 + 3 * 5); // = 17
/// ```
pub type BinaryTuple = StackTuple<2>;

/// Stack-allocated atom for zero-allocation performance.
///
/// `StackAtom` is a lightweight wrapper around an index and size that implements
/// `LinearIndexable` for efficient function application without any World allocation.
/// It stores both the element's index within its set and the size of that set for
/// proper array indexing when used in multi-dimensional contexts.
///
/// # Performance Benefits
///
/// - **Zero allocation**: No heap allocation or World interaction needed
/// - **Direct indexing**: Simply returns the contained index value
/// - **Minimal overhead**: Effectively zero-cost abstraction
/// - **Type safety**: Carries size information for validation and array operations
///
/// # Example
///
/// ```
/// # use pshcalc::set::{StackAtom, LinearIndexable, World};
/// let world = World::new();
/// let atom = StackAtom::new(42, 100);
/// assert_eq!(atom.to_linear_index(&world), 42);
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct StackAtom {
    index: usize,
    size: usize,
}

impl StackAtom {
    /// Creates a new StackAtom with the specified index and size.
    ///
    /// # Arguments
    ///
    /// * `index` - The index value for this atom
    /// * `size` - The size of the set this atom belongs to
    ///
    /// # Returns
    ///
    /// New StackAtom with the given index and size
    #[inline(always)]
    pub fn new(index: usize, size: usize) -> Self {
        Self { index, size }
    }

    /// Gets the index value of this atom.
    ///
    /// # Returns
    ///
    /// The current index value
    #[inline(always)]
    pub fn get_index(&self) -> usize {
        self.index
    }

    /// Sets the index value of this atom.
    ///
    /// # Arguments
    ///
    /// * `index` - New index value
    #[inline(always)]
    pub fn set_index(&mut self, index: usize) {
        self.index = index;
    }
}

impl LinearIndexable for StackAtom {
    /// For atoms, the linear index is simply the contained index value.
    #[inline(always)]
    fn to_linear_index(&self, _world: &World) -> usize {
        self.index
    }
}

// LinearIndexable implementation for arrays of StackAtom
impl<const N: usize> LinearIndexable for [StackAtom; N] {
    /// Calculate linear index for arrays of StackAtoms using row-major order.
    ///
    /// For an array [a₀, a₁, ..., aₙ₋₁] where each atom has the same size,
    /// the linear index is: a₀ + a₁×size + a₂×size² + ... + aₙ₋₁×sizeⁿ⁻¹
    #[inline(always)]
    fn to_linear_index(&self, _world: &World) -> usize {
        if self.is_empty() {
            return 0;
        }

        let size = self[0].size;
        let mut result = 0;
        let mut multiplier = 1;

        for atom in self.iter() {
            result += atom.index * multiplier;
            multiplier *= size;
        }

        result
    }
}

impl BinaryTuple {
    /// Creates a new binary tuple with equal dimensions.
    ///
    /// Both dimensions will have the specified size, representing
    /// an element of the Cartesian product A × A where |A| = size.
    ///
    /// # Arguments
    ///
    /// * `size` - Size of both dimensions
    ///
    /// # Returns
    ///
    /// New BinaryTuple initialized to (0, 0)
    ///
    /// # Example
    ///
    /// ```
    /// # use pshcalc::set::BinaryTuple;
    /// let tuple = BinaryTuple::new_binary(3);
    /// // Represents element of {0,1,2} × {0,1,2}
    /// ```
    #[inline(always)]
    pub fn new_binary(size: usize) -> Self {
        Self::new([size, size])
    }

    /// Sets both coordinates simultaneously.
    ///
    /// Convenience method for setting both dimensions of a binary tuple
    /// in a single call.
    ///
    /// # Arguments
    ///
    /// * `i` - First coordinate
    /// * `j` - Second coordinate
    ///
    /// # Example
    ///
    /// ```
    /// # use pshcalc::set::BinaryTuple;
    /// let mut tuple = BinaryTuple::new_binary(3);
    /// tuple.set(1, 2);
    /// assert_eq!(tuple.get_indices_array(), &[1, 2]);
    /// ```
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

/// Trait for iterating through all elements of a set.
///
/// The `Variable` trait provides a uniform interface for iterating through
/// mathematical sets. It supports initialization, advancement, and element
/// retrieval with efficient state management.
///
/// # Type Parameters
///
/// * `T` - Type of elements yielded by this variable (must implement [`Element`])
///
/// # Iterator Pattern
///
/// Variables follow a specific iteration pattern:
/// 1. Call [`initialize`](Self::initialize) to reset to the first element
/// 2. Call [`get`](Self::get) to retrieve the current element (returns `Some(T)`)
/// 3. Call [`advance`](Self::advance) to move to the next element
/// 4. Repeat steps 2-3 until [`get`](Self::get) returns `None`
///
/// # Example
///
/// ```
/// # use pshcalc::set::{World, AtomSet, Variable};
/// let mut world = World::new();
/// let set = AtomSet::new(3);
/// let mut var = set.create_variable(&mut world);
///
/// var.initialize(&mut world);
/// while let Some(atom) = var.get() {
///     println!("Atom index: {}", atom.get_index(&world));
///     var.advance(&mut world);
/// }
/// ```
pub trait Variable<T: Element> {
    /// Initializes the variable to point to the first element.
    ///
    /// Resets internal state and positions the variable at the beginning
    /// of the iteration sequence.
    ///
    /// # Arguments
    ///
    /// * `world` - Mutable reference to World for state manipulation
    fn initialize(&mut self, world: &mut World);

    /// Advances the variable to the next element.
    ///
    /// Moves the internal state to point to the next element in the
    /// iteration sequence. After advancing past the last element,
    /// [`get`](Self::get) will return `None`.
    ///
    /// # Arguments
    ///
    /// * `world` - Mutable reference to World for state manipulation
    fn advance(&mut self, world: &mut World);

    /// Returns the current element, if any.
    ///
    /// # Returns
    ///
    /// - `Some(&T)` if positioned at a valid element
    /// - `None` if iteration is complete or not initialized
    fn get(&self) -> Option<&T>;
}

/// Trait for mathematical sets that can serve as domains or codomains.
///
/// This trait provides a uniform interface for different types of sets,
/// enabling them to be used interchangeably in contexts like function
/// definitions (HomSets) and other mathematical constructions.
///
/// # Implementors
///
/// - [`AtomSet`] - Finite sets of atomic elements
/// - [`ProductSet`] - Cartesian products of other sets
///
/// # Mathematical Interpretation
///
/// In category theory and set theory, this trait represents the concept
/// of a "set" or "object" that can participate in morphisms and other
/// mathematical constructions.
pub trait Set {
    /// Returns the cardinality (number of elements) of this set.
    ///
    /// # Returns
    ///
    /// Number of elements in the set
    fn size(&self) -> usize;
}

/// Mathematical set of atomic elements.
///
/// An `AtomSet` represents a finite set A = {0, 1, 2, ..., size-1} of atomic
/// elements. It can create variables for iterating through all elements.
///
/// # Mathematical Interpretation
///
/// In set theory, this represents a finite set with a canonical enumeration.
/// All elements are atomic (indivisible) and identified by their index.
///
/// # Example
///
/// ```
/// # use pshcalc::set::{World, AtomSet, Variable};
/// let mut world = World::new();
/// let set = AtomSet::new(3); // Set {0, 1, 2}
/// let mut var = set.create_variable(&mut world);
///
/// var.initialize(&mut world);
/// let mut count = 0;
/// while let Some(_) = var.get() {
///     count += 1;
///     var.advance(&mut world);
/// }
/// assert_eq!(count, 3);
/// ```
#[derive(Clone)]
pub struct AtomSet {
    size: usize,
}

impl AtomSet {
    /// Creates a new AtomSet with the specified number of elements.
    ///
    /// # Arguments
    ///
    /// * `size` - Number of elements in the set (elements will be 0..size-1)
    ///
    /// # Returns
    ///
    /// New AtomSet representing {0, 1, 2, ..., size-1}
    pub fn new(size: usize) -> Self {
        Self { size }
    }

    /// Returns the number of elements in this set.
    ///
    /// # Returns
    ///
    /// Size of the set (number of elements)
    pub fn size(&self) -> usize {
        self.size
    }

    /// Creates a variable for iterating through all elements of this set.
    ///
    /// # Arguments
    ///
    /// * `world` - Mutable reference to World for allocation
    ///
    /// # Returns
    ///
    /// New [`AtomSetVariable`] for iterating through this set
    pub fn create_variable(&self, world: &mut World) -> AtomSetVariable {
        AtomSetVariable::new(world, self.size)
    }
}

impl Set for AtomSet {
    #[inline(always)]
    fn size(&self) -> usize {
        self.size
    }
}

/// Iterator over atoms in an [`AtomSet`].
///
/// This iterator yields [`StackAtom`] instances for each element in the set,
/// allowing zero-allocation iteration while maintaining compatibility with
/// the [`LinearIndexable`] trait for function application.
pub struct AtomSetIterator {
    current: usize,
    size: usize,
}

impl Iterator for AtomSetIterator {
    type Item = StackAtom;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.size {
            let atom = StackAtom::new(self.current, self.size);
            self.current += 1;
            Some(atom)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.size - self.current;
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for AtomSetIterator {
    fn len(&self) -> usize {
        self.size - self.current
    }
}

impl IntoIterator for &AtomSet {
    type Item = StackAtom;
    type IntoIter = AtomSetIterator;

    fn into_iter(self) -> Self::IntoIter {
        AtomSetIterator {
            current: 0,
            size: self.size,
        }
    }
}

/// Variable for iterating through elements of an [`AtomSet`].
///
/// This variable generates [`AtomHandle`] instances representing each element
/// in the associated atom set, from index 0 to size-1.
///
/// # Example
///
/// ```
/// # use pshcalc::set::{World, AtomSet, Variable};
/// let mut world = World::new();
/// let set = AtomSet::new(3);
/// let mut var = set.create_variable(&mut world);
///
/// var.initialize(&mut world);
/// let atom = var.get().unwrap();
/// assert_eq!(atom.get_index(&world), 0);
///
/// var.advance(&mut world);
/// let atom = var.get().unwrap();
/// assert_eq!(atom.get_index(&world), 1);
/// ```
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

impl Set for ProductSet {
    #[inline(always)]
    fn size(&self) -> usize {
        self.sizes.iter().product()
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
    /// Creates a new HomSet representing functions from any set type to any set type.
    ///
    /// This method accepts any types implementing the [`Set`] trait, making it
    /// flexible for different kinds of domains and codomains. In category theory,
    /// this represents the hom-set Hom(A, B) of morphisms from object A to object B.
    ///
    /// # Arguments
    ///
    /// * `source` - Domain set (any type implementing [`Set`])
    /// * `target` - Codomain set (any type implementing [`Set`])
    ///
    /// # Returns
    ///
    /// New HomSet with size target_size^domain_size
    ///
    /// # Examples
    ///
    /// ```
    /// # use pshcalc::set::{AtomSet, ProductSet, HomSet};
    /// let atom_set = AtomSet::new(3);
    /// let product_set = ProductSet::new(&[AtomSet::new(2), AtomSet::new(2)]);
    ///
    /// // Both are now valid:
    /// let hom1 = HomSet::new(&product_set, &atom_set); // ProductSet → AtomSet
    /// let hom2 = HomSet::new(&atom_set, &atom_set);    // AtomSet → AtomSet
    /// ```
    #[inline(always)]
    pub fn new<S: Set, T: Set>(source: &S, target: &T) -> Self {
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
/// Optimized linear index calculation shared across handle types.
///
/// This function converts multi-dimensional indices to a single linear index
/// using row-major order (rightmost index varies fastest). It provides
/// optimized paths for common cases.
///
/// # Algorithm
///
/// For indices [i₀, i₁, ..., iₙ₋₁] and sizes [s₀, s₁, ..., sₙ₋₁]:
/// ```text
/// linear_index = i₀ + i₁×s₀ + i₂×s₀×s₁ + ... + iₙ₋₁×s₀×s₁×...×sₙ₋₂
/// ```
///
/// # Performance
///
/// - **Fast path**: Binary case (len=2) uses direct formula: `i + j×size_i`
/// - **General case**: Loop with running multiplier for arbitrary dimensions
/// - **Inline**: Aggressive inlining for hot loop performance
///
/// # Arguments
///
/// * `indices` - Multi-dimensional indices [i₀, i₁, ..., iₙ₋₁]
/// * `sizes` - Dimension sizes [s₀, s₁, ..., sₙ₋₁]
///
/// # Returns
///
/// Linear index suitable for array access
///
/// # Panics
///
/// Behavior is undefined if `indices.len() != sizes.len()` or if any
/// `indices[i] >= sizes[i]`.
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
