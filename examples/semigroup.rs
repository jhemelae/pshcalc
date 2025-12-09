use pshcalc::{
    cursor,
    set::{AtomSet, Cursor, HomSet, ProductSet, Set},
};
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let n = 4;

    let a = AtomSet::new(n);
    let a_x_a = ProductSet::new(&[a.clone(), a.clone()]);
    let multiplications = HomSet::new(&a_x_a.into(), &a);

    let mut count = 0;
    cursor!(f in multiplications {
        if is_associative(f, &a) {
            count += 1;
        }
    });
    let duration = start.elapsed();
    println!("Count = {:?}", count);
    println!("Time elapsed is: {:?}", duration);
}

/// Check if a function is associative.
/// A function f: A×A → A is associative if f(f(i,j),k) = f(i,f(j,k)) for all i,j,k ∈ A
fn is_associative(f: &Vec<usize>, a: &AtomSet) -> bool {
    cursor!(i in a {
        cursor!(j in a {
            cursor!(k in a {
                // Calculate f(f(i,j), k)
                let f_ij = f[i * a.size() + j];
                let left = f[f_ij * a.size() + k];

                // Calculate f(i, f(j,k))
                let f_jk = f[j * a.size() + k];
                let right = f[i * a.size() + f_jk];

                if left != right {
                    return false;
                }
            });
        });
    });
    true
}
