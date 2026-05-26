use crate::cat::{Category, AtomCategory};
use crate::cursor;
use crate::set::{AtomSet, Set, Variable};

#[derive(Debug, PartialEq)]
pub enum PresheafError {
    NotWellDefined { s: usize, f: usize },
    NonAssociative { triple: (usize, usize, usize) },
}

impl std::fmt::Display for PresheafError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PresheafError::NotWellDefined { s, f } => {
                write!(formatter, "Not well-defined: s={} and f={}", s, f)
            }
            PresheafError::NonAssociative { triple } => {
                write!(formatter, "Non-associative composition: {:?}", triple)
            }
        }
    }
}

pub trait Presheaf {
    fn sections(&self) -> impl Set<usize>;
    fn pi(&self, index: usize) -> usize;
    fn action<T: Category>(&self, category: &T, section: usize, morphism: usize) -> usize;
}

#[derive(Clone, Debug)]
pub struct AtomPresheaf {
    pi: Vec<usize>,
    action: Vec<usize>,
}

impl Presheaf for AtomPresheaf {
    #[inline(always)]
    fn sections(&self) -> impl Set<usize> {
        AtomSet::new(self.pi.len())
    }

    #[inline(always)]
    fn pi(&self, index: usize) -> usize {
        self.pi[index]
    }

    #[inline(always)]
    fn action<T: Category>(&self, category: &T, section: usize, morphism: usize) -> usize {
        // identity?
        if morphism < category.objects().size() {
            return section;
        }
        let morphism = morphism - category.objects().size();
        let number_of_sections = self.pi.len();
        self.action[section + morphism * number_of_sections]
    }
}

impl AtomPresheaf {
    #[inline(always)]
    pub fn validate(&self, category: &AtomCategory) -> Result<(), PresheafError> {
        self.validate_associativity(category)?;
        self.validate_well_definedness(category)?;
        Ok(())
    }

    #[inline(always)]
    fn validate_associativity(&self, category: &AtomCategory) -> Result<(), PresheafError> {
        let sections = self.sections();
        let morphisms = category.morphisms();

        cursor!(s in &sections => {
            cursor!(f in &morphisms => {
                cursor!(g in &morphisms => {
                    let left = self.action(category, self.action(category, *s, *f), *g);
                    let right = self.action(category, *s, category.composition(*g, *f));

                    if left != right {
                        return Err(PresheafError::NonAssociative {
                            triple: (*s, *f, *g),
                        });
                    }
                });
            });
        });
        Ok(())
    }

    #[inline(always)]
    pub fn validate_well_definedness(&self, category: &AtomCategory) -> Result<(), PresheafError> {
        let sections = self.sections();
        let morphisms = category.morphisms();

        cursor!(s in &sections => {
            cursor!(f in &morphisms => {
                let s_f = self.action(category, *s, *f);
                let v = self.pi(s_f);
                let source_f = category.source(*f);
                let u = self.pi(*s);
                let target_f = category.target(*f);

                if  v != source_f {
                    return Err(PresheafError::NotWellDefined { s: *s, f: *f });
                }

                if u != target_f && s_f != 0 {
                    return Err(PresheafError::NotWellDefined { s: *s, f: *f });
                }
            });
        });
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct VaryAction<'a> {
    category: &'a AtomCategory,
    pi: &'a Vec<usize>,
}

impl<'a> VaryAction<'a> {
    #[inline(always)]
    pub fn new(category: &'a AtomCategory, pi: &'a Vec<usize>) -> Self {
        VaryAction { category, pi }
    }
}

impl Set<AtomPresheaf> for VaryAction<'_> {
    #[inline(always)]
    fn allocate(&self) -> Variable<AtomPresheaf> {
        let number_of_nonidentity_morphisms =
            self.category.morphisms().size() - self.category.objects().size();
        let number_of_sections = self.pi.len();
        let presheaf = AtomPresheaf {
            pi: self.pi.clone(),
            action: vec![0; number_of_sections * number_of_nonidentity_morphisms],
        };
        Variable {
            value: presheaf,
            ongoing: false
        }
    }

    #[inline(always)]
    fn next(&self, current: &mut Variable<AtomPresheaf>) {
        let number_of_sections = self.pi.len();
        for i in 0..current.value.action.len() {
            current.value.action[i] += 1;
            if current.value.action[i] < number_of_sections {
                if current.value.validate(self.category).is_ok() {
                    current.ongoing = true;
                    return;
                }
                return self.next(current);
            } else {
                current.value.action[i] = 0;
            }
        }
        current.ongoing = false;
    }

    #[inline(always)]
    fn reset(&self, current: &mut Variable<AtomPresheaf>) {
        for i in 0..current.value.pi.len() {
            current.value.pi[i] = 0;
        }
        for i in 0..current.value.action.len() {
            current.value.action[i] = 0;
        }
        if current.value.validate(self.category).is_ok() {
            current.ongoing = true;
            return; 
        }
        self.next(current);
    }
}

struct Yoneda<'a, T: Category> {
    object: usize,
    category: &'a T,
}

struct YonedaSet<'a, T: Category> {
    category: &'a T,
    object: usize,
}

impl<'a, T> Set<usize> for YonedaSet<'a, T> 
    where T: Category {
    fn allocate(&self) -> Variable<usize> {
        Variable {
            value: 0,
            ongoing: false,
        }
    }
    
    fn reset(&self, current: &mut Variable<usize>) {
        let morphisms = self.category.morphisms();

        morphisms.reset(current);
        
        while self.category.target(current.value) != self.object {
            morphisms.next(current);
        }
    }

    fn next(&self, current: &mut Variable<usize>) {
        let morphisms = self.category.morphisms();

        morphisms.next(current);

        while self.category.target(current.value) != self.object {
            morphisms.next(current);
        }
    }
}

impl<'a, T> Presheaf for Yoneda<'a, T>
    where T: Category {
    fn sections(&self) -> impl Set<usize> {
        YonedaSet {
            object: self.object,
            category: self.category,
        }
    }

    fn pi(&self, index: usize) -> usize {
        self.category.source(index)
    }

    fn action<S: Category>(&self, category: &S, section: usize, morphism: usize) -> usize {
        category.composition(section, morphism)
    }
}

