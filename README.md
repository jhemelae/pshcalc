# pshcalc

A Rust library for computations with finite sets, with planned support for category theory and presheaf calculations. Currently a work in progress.

## Overview

`pshcalc` implements finite sets, Cartesian products, and function spaces for computational mathematics. Currently focused on enumerating algebraic structures like semigroups, with category theory features planned for future development.

## Features

- Finite set operations and iteration
- Cartesian product construction
- Function space enumeration
- Memory-efficient design with contiguous data layout
- Stack-allocated types for performance-critical operations

## Architecture

The library uses an Entity-Component-System inspired design centered around a `World` struct that manages all data in contiguous arrays. Mathematical objects are represented by lightweight handles that store only offsets and lengths.

## Examples

### Counting Semigroups

```rust
use pshcalc::set::{AtomSet, HomSet, ProductSet, Variable, World};

let mut world = World::new();
let n = 4;

let a = AtomSet::new(n);
let a_x_a = ProductSet::new(&[a.clone(), a.clone()]);
let multiplications = HomSet::new(&a_x_a, &a);

let mut function = multiplications.create_variable(&mut world);
function.initialize(&mut world);

let mut semigroup_count = 0;
while let Some(f) = function.get() {
    if is_associative(&world, f, &a) {
        semigroup_count += 1;
    }
    function.advance(&mut world);
}
```

## Performance

The library uses several optimizations:

- Contiguous memory layout for better cache performance
- Specialized binary tuple operations
- Stack allocation for small, fixed-size operations

Example: Enumerating all 3,492 semigroups on a 4-element set takes ~20 seconds.
