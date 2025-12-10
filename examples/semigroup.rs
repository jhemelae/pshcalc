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
    let multiplications = HomSet::new(&a_x_a.clone().into(), &a);

    let mut count = 0;
    cursor!(f in multiplications {
        if is_associative(f, &a, &a_x_a) {
            count += 1;
        }
    });
    let duration = start.elapsed();
    println!("Count = {:?}", count);
    println!("Time elapsed is: {:?}", duration);
}

/// Check if a function is associative.
/// A function f: A×A → A is associative if f(f(i,j),k) = f(i,f(j,k)) for all i,j,k ∈ A
fn is_associative(f: &[usize], a: &AtomSet, a_x_a: &ProductSet) -> bool {
    cursor!(i in a {
        cursor!(j in a {
            cursor!(k in a {
                // Calculate f(f(i,j), k)
                let f_ij = f[a_x_a.get(&[*i, *j])];
                let left = f[a_x_a.get(&[f_ij, *k])];

                // Calculate f(i, f(j,k))
                let f_jk = f[a_x_a.get(&[*j, *k])];
                let right = f[a_x_a.get(&[*i, f_jk])];

                if left != right {
                    return false;
                }
            });
        });
    });
    true
}
