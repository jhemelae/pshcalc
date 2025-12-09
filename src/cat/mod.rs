use crate::cursor;
use crate::set::{AtomSet, BasicCursor, Cursor, Set};

#[derive(Debug, PartialEq)]
pub enum CategoryError {
    IncompatibleComposition { g: usize, f: usize },
    NonAssociative { morphisms: (usize, usize, usize) },
    NotAnIdentity { morphism: usize },
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
            CategoryError::NotAnIdentity { morphism } => {
                write!(formatter, "Morphism {} is not an identity", morphism)
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
        // we consider non-identity morphisms only
        cursor!(f in morphisms {
            cursor!(g in morphisms  {
                cursor!(h in morphisms {
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

pub trait Advanceable {
    fn advance(&mut self) -> bool;
}

impl Advanceable for Category {
    #[inline(always)]
    fn advance(&mut self) -> bool {
        for i in 0..self.composition.len() {
            self.composition[i] += 1;
            if self.composition[i] < self.number_of_morphisms() {
                return false;
            }
            self.composition[i] = 0;
        }
        true
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
    fn cursor(&self) -> impl Cursor<Category> {
        BasicCursor::new(self.clone())
    }

    #[inline(always)]
    fn get_next<'a>(
        &self,
        current: &'a mut Option<Category>,
    ) -> &'a Option<Category> {
        if let Some(category) = current {
            for i in 0..category.composition.len() {
                category.composition[i] += 1;
                if category.composition[i] < self.number_of_morphisms {
                    if category.validate().is_ok() {
                        return current;
                    }
                    return self.get_next(current);
                }
                category.composition[i] = 0;
            }
            *current = None;
            return current;
        } else {
            *current = Some(Category::new(
                self.number_of_objects,
                self.source.clone(),
                self.target.clone(),
                vec![
                    0;
                    (self.number_of_morphisms - self.number_of_objects)
                        * (self.number_of_morphisms - self.number_of_objects)
                ],
            ));
            let cat = current.as_mut().unwrap();
            if cat.validate().is_ok() {
                return current;
            }
            return self.get_next(current);
        }
    }

    fn get_index(&self, value: &Category) -> usize {
        let mut index = 0;
        let mut multiplier = 1;
        for &img in &value.composition {
            index += img * multiplier;
            multiplier *= self.number_of_morphisms;
        }
        index
    }
}
