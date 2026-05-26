use crate::cursor;
use crate::set::{AtomSet, Set, Variable};

#[derive(Debug, PartialEq)]
pub enum CategoryError {
    IncompatibleComposition { g: usize, f: usize },
    NonAssociative { morphisms: (usize, usize, usize) },
}

impl std::fmt::Display for CategoryError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CategoryError::IncompatibleComposition { g, f } => {
                write!(formatter, "Incompatible composition: g={} and f={}", g, f)
            }
            CategoryError::NonAssociative { morphisms } => {
                write!(formatter, "Non-associative composition: {:?}", morphisms)
            }
        }
    }
}

impl std::error::Error for CategoryError {}

pub trait Category {
    fn objects(&self) -> impl Set<usize>;
    fn morphisms(&self) -> impl Set<usize>; 
    fn source(&self, input: usize) -> usize;
    fn target(&self, input: usize) -> usize;
    fn composition(&self, g: usize, f: usize) -> usize;
} 
 
// We assume list of objects and list of morphisms.
// The first morphisms are the identities for each object, in the exact same order.
// So the identity map is [0, 1, 2, ..., number_of_objects - 1]
// Source, target and composition store only the values on non-identity morphisms.
// This means for source and target that the first n values are omitted, where n is the number of objects.
// For composition, the values for compositions involving identity morphisms are omitted.
// These are the linearindexable tuples (i, j) where i < number_of_objects or j < number_of_objects.
#[derive(Clone, Debug)]
pub struct AtomCategory {
    number_of_objects: usize,
    number_of_morphisms: usize,
    source: Vec<usize>,
    target: Vec<usize>,
    composition: Vec<usize>,
}


impl Category for AtomCategory {
    #[inline(always)]
    fn objects(&self) -> impl Set<usize> {
        AtomSet::new(self.number_of_objects)
    }

    #[inline(always)]
    fn morphisms(&self) -> impl Set<usize> {
        AtomSet::new(self.number_of_morphisms)
    }

    #[inline(always)]
    fn source(&self, input: usize) -> usize {
        // identity morphism?
        if input < self.number_of_objects {
            return input;
        }
            self.source[input - self.number_of_objects]
    }

    #[inline(always)]
    fn target(&self, input: usize) -> usize {
        // identity morphism?
            if input < self.number_of_objects {
            return input;
        }
        self.target[input - self.number_of_objects]
    }

    #[inline(always)]
    fn composition(&self, g: usize, f: usize) -> usize {
        if g < self.number_of_objects {
            if self.target(f) == self.source(g) {
                return f;
            } else {
                return 0;
            }
        }

        if f < self.number_of_objects {
            if self.target(f) == self.source(g) {
                return g;
            } else {
                return 0;
            }
        }

        let j = g - self.number_of_objects;
        let i = f - self.number_of_objects;
        let n = self.number_of_morphisms - self.number_of_objects;
        let index = j * n + i;
        self.composition[index]
    }
}

impl AtomCategory {
    #[inline(always)]
    pub fn validate(&self) -> Result<(), CategoryError> {
        self.validate_associativity()?;
        self.validate_well_definedness()?;
        Ok(())
    }

    #[inline(always)]
    fn validate_associativity(&self) -> Result<(), CategoryError> {
        let morphisms = self.morphisms();
        cursor!(f in &morphisms => {
            cursor!(g in &morphisms => {
                cursor!(h in &morphisms => {
                    let left = self.composition(self.composition(*h, *g), *f);
                    let right = self.composition(*h, self.composition(*g, *f));

                    if left != right {
                        return Err(CategoryError::NonAssociative {
                            morphisms: (*h, *g, *f),
                        });
                    }
                });
            });
        });
        Ok(())
    }

    #[inline(always)]
    fn validate_well_definedness(&self) -> Result<(), CategoryError> {
        // we consider non-identity morphisms only
        for f in self.number_of_objects..self.number_of_morphisms {
            for g in self.number_of_objects..self.number_of_morphisms {
                let target_f = self.target(f);
                let source_g = self.source(g);
                let composition = self.composition(g, f);

                if target_f != source_g && composition != 0 {
                    return Err(CategoryError::IncompatibleComposition { g, f });
                }
            }
        }
        Ok(())
    }
}

#[derive(Clone)]
pub struct VaryCompositionSet {
    number_of_objects: usize,
    number_of_morphisms: usize,
    source: Vec<usize>,
    target: Vec<usize>,
}


impl Set<AtomCategory> for VaryCompositionSet {
    #[inline(always)]
    fn allocate(&self) -> Variable<AtomCategory> {
        let category = AtomCategory {
            number_of_objects: self.number_of_objects,
            number_of_morphisms: self.number_of_morphisms,
            source: self.source.clone(),
            target: self.target.clone(),
            composition: vec![
                0;
                (self.number_of_morphisms - self.number_of_objects)
                    * (self.number_of_morphisms - self.number_of_objects)
            ],
        };
        Variable {
            value: category,
            ongoing: false
        }
    }

    #[inline(always)]
    fn next(&self, current: &mut Variable<AtomCategory>) {
        for i in 0..current.value.composition.len() {
            current.value.composition[i] += 1;
            if current.value.composition[i] < self.number_of_morphisms {
                if current.value.validate().is_ok() {
                    current.ongoing = true;
                    return;
                }
                return self.next(current);
            }
            current.value.composition[i] = 0;
        }
        current.ongoing = false;
    }

    #[inline(always)]
    fn reset(&self, current: &mut Variable<AtomCategory>) {
        for i in 0..current.value.composition.len() {
            current.value.composition[i] = 0;
        }
        if current.value.validate().is_ok() {
            current.ongoing = true;
        }
        self.next(current)
    }
}

pub fn monoids(size: usize) -> VaryCompositionSet {
    VaryCompositionSet {
        number_of_objects: 1,
        number_of_morphisms: size,
        source: vec![0;size-1],
        target: vec![0;size-1],
    }
}

        

