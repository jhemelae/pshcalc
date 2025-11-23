use crate::set::{AtomSet, DataRange, Function, World};

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

pub struct Category<'a> {
    values: &'a [usize],
    sizes: &'a [usize],
    morphism_count: usize,
    object_count: usize,
}

impl Category<'_> {
    #[inline(always)]
    pub fn source(&self) -> Function {
        let range: std::ops::Range<usize> =
            Self::source_range(self.morphism_count, self.object_count).into();
        Function::new(&self.values[range.clone()], &self.sizes[range])
    }

    #[inline(always)]
    pub fn target(&self) -> Function {
        let range: std::ops::Range<usize> =
            Self::target_range(self.morphism_count, self.object_count).into();
        Function::new(&self.values[range.clone()], &self.sizes[range])
    }

    #[inline(always)]
    pub fn identity(&self) -> Function {
        let range: std::ops::Range<usize> =
            Self::identity_range(self.morphism_count, self.object_count).into();
        Function::new(&self.values[range.clone()], &self.sizes[range])
    }

    #[inline(always)]
    pub fn composition(&self) -> Function {
        let range: std::ops::Range<usize> =
            Self::composition_range(self.morphism_count, self.object_count)
                .into();
        Function::new(&self.values[range.clone()], &self.sizes[range])
    }

    #[inline(always)]
    fn source_range(morphism_count: usize, _object_count: usize) -> DataRange {
        DataRange::new(0, morphism_count)
    }

    #[inline(always)]
    fn target_range(morphism_count: usize, _object_count: usize) -> DataRange {
        DataRange::new(morphism_count, morphism_count)
    }

    #[inline(always)]
    fn identity_range(morphism_count: usize, object_count: usize) -> DataRange {
        DataRange::new(2 * morphism_count, object_count)
    }

    #[inline(always)]
    fn composition_range(
        morphism_count: usize,
        object_count: usize,
    ) -> DataRange {
        DataRange::new(
            2 * morphism_count + object_count,
            morphism_count * morphism_count,
        )
    }

    #[inline(always)]
    pub fn validate(&self) -> Result<(), CategoryError> {
        let objects = self.identity().domain();
        let morphisms = self.source().domain();

        let composition = self.composition();
        let source = self.source();
        let target = self.target();
        let identity = self.identity();

        self.validate_identity_laws(
            &objects,
            &morphisms,
            &composition,
            &source,
            &target,
            &identity,
        )?;
        self.validate_associativity(&morphisms, &composition)?;
        self.validate_well_definedness(
            &morphisms,
            &composition,
            &source,
            &target,
        )?;
        Ok(())
    }

    #[inline(always)]
    fn validate_well_definedness(
        &self,
        morphisms: &AtomSet,
        composition: &Function,
        source: &Function,
        target: &Function,
    ) -> Result<(), CategoryError> {
        for f in morphisms {
            for g in morphisms {
                let target_f = target.apply(&[f.clone()]);
                let source_g = source.apply(&[g.clone()]);
                let composition = composition.apply(&[g.clone(), f.clone()]);

                if target_f != source_g && composition.index != 0 {
                    return Err(CategoryError::IncompatibleComposition {
                        g: g.index,
                        f: f.index,
                    });
                }
            }
        }
        Ok(())
    }

    #[inline(always)]
    fn validate_identity_laws(
        &self,
        objects: &AtomSet,
        morphisms: &AtomSet,
        composition: &Function,
        source: &Function,
        target: &Function,
        identity: &Function,
    ) -> Result<(), CategoryError> {
        for x in objects {
            let id_x = identity.apply(&[x.clone()]);

            for f in morphisms {
                let source_f = source.apply(&[f.clone()]);
                let target_f = target.apply(&[f.clone()]);
                if source_f == x {
                    let composed =
                        composition.apply(&[f.clone(), id_x.clone()]);
                    if composed != f {
                        return Err(CategoryError::NotAnIdentity {
                            morphism: id_x.index,
                        });
                    }
                }

                if target_f == x {
                    let composed =
                        composition.apply(&[id_x.clone(), f.clone()]);
                    if composed != f {
                        return Err(CategoryError::NotAnIdentity {
                            morphism: id_x.index,
                        });
                    }
                }
            }
        }
        Ok(())
    }

    #[inline(always)]
    fn validate_associativity(
        &self,
        morphisms: &AtomSet,
        composition: &Function,
    ) -> Result<(), CategoryError> {
        for f in morphisms {
            for g in morphisms {
                for h in morphisms {
                    let left = composition.apply(&[
                        composition.apply(&[h.clone(), g.clone()]),
                        f.clone(),
                    ]);
                    let right = composition.apply(&[
                        h.clone(),
                        composition.apply(&[g.clone(), f.clone()]),
                    ]);

                    if left != right {
                        return Err(CategoryError::NonAssociative {
                            morphisms: (h.index, g.index, f.index),
                        });
                    }
                }
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct CategoryVariable {
    data_range: DataRange,
    morphism_count: usize,
    object_count: usize,
    done: bool,
}

impl CategoryVariable {
    #[inline(always)]
    pub fn new(
        world: &mut World,
        object_count: usize,
        morphism_count: usize,
    ) -> Self {
        let total_len =
            2 * morphism_count + object_count + morphism_count * morphism_count;
        let data_range = world.alloc(total_len);

        // Use Category's range helper functions to avoid code duplication
        let sizes = world.get_sizes_mut(&data_range);

        let source_range = Category::source_range(morphism_count, object_count);
        let target_range = Category::target_range(morphism_count, object_count);
        let identity_range =
            Category::identity_range(morphism_count, object_count);
        let composition_range =
            Category::composition_range(morphism_count, object_count);

        sizes[std::ops::Range::<usize>::from(source_range)].fill(object_count);
        sizes[std::ops::Range::<usize>::from(target_range)].fill(object_count);
        sizes[std::ops::Range::<usize>::from(identity_range)]
            .fill(morphism_count);
        sizes[std::ops::Range::<usize>::from(composition_range)]
            .fill(morphism_count);

        world.get_values_mut(&data_range).fill(0);

        Self {
            data_range,
            morphism_count,
            object_count,
            done: false,
        }
    }

    #[inline(always)]
    pub fn initialize(&mut self, world: &mut World) {
        self.done = false;
        world.get_values_mut(&self.data_range).fill(0);
    }

    #[inline(always)]
    pub fn advance(&mut self, world: &mut World) {
        if self.done {
            return;
        }

        let composition_range =
            Category::composition_range(self.morphism_count, self.object_count);
        self.done = world.advance_counter(&composition_range);
    }

    #[inline(always)]
    pub fn get<'a>(&self, world: &'a World) -> Option<Category<'a>> {
        if self.done {
            return None;
        }

        Some(Category {
            values: world.get_values(&self.data_range),
            sizes: world.get_sizes(&self.data_range),
            morphism_count: self.morphism_count,
            object_count: self.object_count,
        })
    }
}
