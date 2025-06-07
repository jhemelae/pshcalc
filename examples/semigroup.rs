use pshcalc::set::{AtomSet, HomSet, ProductSet, Variable, World};
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let mut world = World::new();
    let n = 4;

    let a = AtomSet::new(n);
    let a_x_a = ProductSet::new(&[a.clone(), a.clone()]);
    let multiplications = HomSet::new(&a_x_a, &a);

    let mut function = multiplications.create_variable(&mut world);
    function.initialize(&mut world);

    let mut count = 0;
    while let Some(f) = function.get() {
        if is_associative(&world, f, &a) {
            count += 1;
        }
        function.advance(&mut world);
    }

    println!("Count = {:?}", count);
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}

/// Check if a function is associative using direct StackAtom arrays.
/// A function f: A×A → A is associative if f(f(i,j),k) = f(i,f(j,k)) for all i,j,k ∈ A
fn is_associative(
    world: &World,
    f: &pshcalc::set::FunctionHandle,
    a: &AtomSet,
) -> bool {
    for i in a {
        for j in a {
            for k in a {
                // Calculate f(f(i,j), k)
                let f_ij = f.apply(world, &[i.clone(), j.clone()]);
                let left = f.apply(world, &[f_ij, k.clone()]);

                // Calculate f(i, f(j,k))
                let f_jk = f.apply(world, &[j.clone(), k.clone()]);
                let right = f.apply(world, &[i.clone(), f_jk]);

                if left != right {
                    return false;
                }
            }
        }
    }
    true
}
