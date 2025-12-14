use crate::cat::Category;
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

#[derive(Clone, Debug)]
pub struct Presheaf {
    pub number_of_sections: usize,
    pub number_of_objects: usize,
    pub number_of_morphisms: usize,
    pub pi: Vec<usize>,
    pub action: Vec<usize>,
}

impl Presheaf {
    #[inline(always)]
    pub fn new(
        category: &Category,
        pi: Vec<usize>,
        action: Vec<usize>,
    ) -> Self {
        let number_of_sections = pi.len();
        let number_of_morphisms = category.number_of_morphisms();
        let number_of_objects = category.number_of_objects();
        Presheaf {
            number_of_sections,
            number_of_objects,
            number_of_morphisms,
            pi,
            action,
        }
    }

    #[inline(always)]
    pub fn allocate(
        number_of_objects: usize,
        number_of_morphisms: usize,
        number_of_sections: usize,
    ) -> Variable<Self> {
        Variable::uninitialized(Presheaf {
            number_of_sections,
            number_of_objects,
            number_of_morphisms,
            pi: vec![0; number_of_sections],
            action: vec![
                0;
                number_of_sections
                    * (number_of_morphisms - number_of_objects)
            ],
        })
    }

    #[inline(always)]
    pub fn number_of_sections(&self) -> usize {
        self.number_of_sections
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
    pub fn sections(&self) -> AtomSet {
        AtomSet::new(self.number_of_sections())
    }

    #[inline(always)]
    pub fn pi(&self, index: usize) -> usize {
        self.pi[index]
    }

    #[inline(always)]
    pub fn action(&self, section: usize, morphism: usize) -> usize {
        // identity?
        if morphism < self.number_of_objects() {
            return section;
        }
        let morphism = morphism - self.number_of_objects();
        self.action[section + morphism * self.number_of_sections()]
    }

    #[inline(always)]
    pub fn validate(&self, category: &Category) -> Result<(), PresheafError> {
        self.validate_associativity(category)?;
        self.validate_well_definedness(category)?;
        Ok(())
    }

    #[inline(always)]
    fn validate_associativity(
        &self,
        category: &Category,
    ) -> Result<(), PresheafError> {
        let sections = self.sections();
        let morphisms = category.morphisms();

        cursor!(s in &sections => {
            cursor!(f in &morphisms => {
                cursor!(g in &morphisms => {
                    let left = self.action(self.action(*s, *f), *g);
                    let right = self.action(*s, category.composition(*g, *f));

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
    pub fn validate_well_definedness(
        &self,
        category: &Category,
    ) -> Result<(), PresheafError> {
        let sections = self.sections();
        let morphisms = category.morphisms();

        cursor!(s in &sections => {
            cursor!(f in &morphisms => {
                let s_f = self.action(*s, *f);
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
pub struct PresheafSet<'a> {
    category: &'a Category,
    pi: &'a Vec<usize>,
}

impl<'a> PresheafSet<'a> {
    #[inline(always)]
    pub fn new(category: &'a Category, pi: &'a Vec<usize>) -> Self {
        PresheafSet { category, pi }
    }
}

impl Set<Presheaf> for PresheafSet<'_> {
    #[inline(always)]
    fn allocate(&self) -> Variable<Presheaf> {
        let number_of_nonidentity_morphisms =
            self.category.number_of_morphisms()
                - self.category.number_of_objects();
        let number_of_sections = self.pi.len();
        let presheaf = Presheaf::new(
            self.category,
            self.pi.clone(),
            vec![0; number_of_sections * number_of_nonidentity_morphisms],
        );
        Variable::uninitialized(presheaf)
    }

    #[inline(always)]
    fn next<'a>(&self, current: &'a mut Presheaf) -> bool {
        let number_of_sections = self.pi.len();
        for i in 0..current.action.len() {
            current.action[i] += 1;
            if current.action[i] < number_of_sections {
                if current.validate(&self.category).is_ok() {
                    return true;
                }
                return self.next(current);
            } else {
                current.action[i] = 0;
            }
        }
        false
    }

    #[inline(always)]
    fn reset<'a>(&self, current: &'a mut Presheaf) -> bool {
        for i in 0..current.pi.len() {
            current.pi[i] = 0;
        }
        for i in 0..current.action.len() {
            current.action[i] = 0;
        }
        if current.validate(&self.category).is_ok() {
            return true;
        }
        self.next(current)
    }
}
