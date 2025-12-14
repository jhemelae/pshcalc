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
                write!(
                    formatter,
                    "Incompatible composition: g={} and f={}",
                    g, f
                )
            }
            CategoryError::NonAssociative { morphisms } => {
                write!(
                    formatter,
                    "Non-associative composition: {:?}",
                    morphisms
                )
            }
        }
    }
}

impl std::error::Error for CategoryError {}

// We assume list of objects and list of morphisms.
// The first morphisms are the identities for each object, in the exact same order.
// So the identity map is [0, 1, 2, ..., number_of_objects - 1]
// Source, target and composition store only the values on non-identity morphisms.
// This means for source and target that the first n values are omitted, where n is the number of objects.
// For composition, the values for compositions involving identity morphisms are omitted.
// These are the linearindexable tuples (i, j) where i < number_of_objects or j < number_of_objects.
#[derive(Clone, Debug)]
pub struct Category {
    number_of_objects: usize,
    number_of_morphisms: usize,
    source: Vec<usize>,
    target: Vec<usize>,
    composition: Vec<usize>,
}

impl Category {
    #[inline(always)]
    pub fn new(
        number_of_objects: usize,
        source: Vec<usize>,
        target: Vec<usize>,
        composition: Vec<usize>,
    ) -> Self {
        let number_of_morphisms = source.len() + number_of_objects;

        Category {
            number_of_objects,
            number_of_morphisms,
            source,
            target,
            composition,
        }
    }

    #[inline(always)]
    pub fn allocate(
        number_of_objects: usize,
        number_of_morphisms: usize,
    ) -> Variable<Self> {
        let non_identity_morphisms = number_of_morphisms - number_of_objects;
        let category = Category {
            number_of_objects,
            number_of_morphisms,
            source: vec![0; non_identity_morphisms],
            target: vec![0; non_identity_morphisms],
            composition: vec![
                0;
                non_identity_morphisms * non_identity_morphisms
            ],
        };
        Variable::uninitialized(category)
    }

    #[inline(always)]
    pub fn number_of_objects(&self) -> usize {
        self.number_of_objects
    }

    #[inline(always)]
    pub fn number_of_morphisms(&self) -> usize {
        self.number_of_morphisms
    }

    #[inline(always)]
    pub fn objects(&self) -> AtomSet {
        AtomSet::new(self.number_of_objects)
    }

    #[inline(always)]
    pub fn morphisms(&self) -> AtomSet {
        AtomSet::new(self.number_of_morphisms)
    }

    #[inline(always)]
    pub fn source(&self, input: usize) -> usize {
        // identity morphism?
        if input < self.number_of_objects() {
            return input;
        }
        self.source[input - self.number_of_objects()]
    }

    #[inline(always)]
    pub fn target(&self, input: usize) -> usize {
        // identity morphism?
        if input < self.number_of_objects() {
            return input;
        }
        self.target[input - self.number_of_objects()]
    }

    #[inline(always)]
    pub fn composition(&self, g: usize, f: usize) -> usize {
        if g < self.number_of_objects() {
            if self.target(f) == self.source(g) {
                return f;
            } else {
                return 0;
            }
        }

        if f < self.number_of_objects() {
            if self.target(f) == self.source(g) {
                return g;
            } else {
                return 0;
            }
        }

        let j = g - self.number_of_objects();
        let i = f - self.number_of_objects();
        let n = self.number_of_morphisms() - self.number_of_objects();
        let index = j * n + i;
        self.composition[index]
    }

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
        for f in self.number_of_objects()..self.number_of_morphisms() {
            for g in self.number_of_objects()..self.number_of_morphisms() {
                let target_f = self.target(f);
                let source_g = self.source(g);
                let composition = self.composition(g, f);

                if target_f != source_g && composition != 0 {
                    return Err(CategoryError::IncompatibleComposition {
                        g,
                        f,
                    });
                }
            }
        }
        Ok(())
    }
}

#[derive(Clone)]
pub struct CategorySet {
    number_of_objects: usize,
    number_of_morphisms: usize,
    source: Vec<usize>,
    target: Vec<usize>,
}

impl CategorySet {
    #[inline(always)]
    pub fn new(
        number_of_objects: usize,
        source: Vec<usize>,
        target: Vec<usize>,
    ) -> Self {
        let number_of_morphisms = source.len() + number_of_objects;
        Self {
            number_of_objects,
            number_of_morphisms,
            source,
            target,
        }
    }
}

impl Set<Category> for CategorySet {
    #[inline(always)]
    fn allocate(&self) -> Variable<Category> {
        let category = Category::new(
            self.number_of_objects,
            self.source.clone(),
            self.target.clone(),
            vec![
                0;
                (self.number_of_morphisms - self.number_of_objects)
                    * (self.number_of_morphisms - self.number_of_objects)
            ],
        );
        Variable::uninitialized(category)
    }

    #[inline(always)]
    fn next<'a>(&self, current: &'a mut Category) -> bool {
        for i in 0..current.composition.len() {
            current.composition[i] += 1;
            if current.composition[i] < self.number_of_morphisms {
                if current.validate().is_ok() {
                    return true;
                }
                return self.next(current);
            }
            current.composition[i] = 0;
        }
        false
    }

    #[inline(always)]
    fn reset<'a>(&self, current: &'a mut Category) -> bool {
        for i in 0..current.composition.len() {
            current.composition[i] = 0;
        }
        if current.validate().is_ok() {
            return true;
        }
        self.next(current)
    }
}
