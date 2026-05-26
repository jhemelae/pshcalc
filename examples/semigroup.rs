use pshcalc::set::{AtomSet, HomSet, ProductSet, Set};
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let n = 4;

    let a = AtomSet::new(n);
    let a_x_a = ProductSet::new(&[a.clone(), a.clone()]);
    let multiplications = HomSet::new(&a_x_a.clone().into(), &a);

    let mut count = 0;
    let mut f = multiplications.allocate();
    multiplications.reset(&mut f);
    while f.ongoing {
        if is_associative(&f.value, &a, &a_x_a) {
            count += 1;
        }
        multiplications.next(&mut f);
    }
    let duration = start.elapsed();
    println!("Count = {:?}", count);
    println!("Time elapsed is: {:?}", duration);
}

/// Check if a function is associative.
/// A function f: A×A → A is associative if f(f(i,j),k) = f(i,f(j,k)) for all i,j,k ∈ A
fn is_associative(f: &[usize], a: &AtomSet, a_x_a: &ProductSet) -> bool {
    let mut i_var = a.allocate();
    let mut j_var = a.allocate();
    let mut k_var = a.allocate();

    a.reset(&mut i_var);
    while i_var.ongoing {
        let i = i_var.value;

        a.reset(&mut j_var);
        while j_var.ongoing {
            let j = j_var.value;

            a.reset(&mut k_var);
            while k_var.ongoing {
                let k = k_var.value;

                // Calculate f(f(i,j), k)
                let f_ij = f[a_x_a.get(&[i, j])];
                let left = f[a_x_a.get(&[f_ij, k])];

                // Calculate f(i, f(j,k))
                let f_jk = f[a_x_a.get(&[j, k])];
                let right = f[a_x_a.get(&[i, f_jk])];

                if left != right {
                    return false;
                }
                a.next(&mut k_var);
            }
            a.next(&mut j_var);
        }
        a.next(&mut i_var);
    }
    true
}
