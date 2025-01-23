use pshcalc::prelude::*;
use pshcalc::set;
use std::time::Instant;

#[inline(always)]
fn get(s: &[usize], n: usize, i: usize, j: usize) -> usize {
    s[n * i + j]
}

#[inline(always)]
fn is_associative(s: &[usize], n: usize) -> bool {
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                let left = get(s, n, get(s, n, i, j), k);
                let right = get(s, n, i, get(s, n, j, k));
                if left != right {
                    return false;
                }
            }
        }
    }
    true
}

fn main() {
    let start = Instant::now();
    let n = 4;
    let a = set::Basic::new(n);
    let a_x_a = set::Product::new(&[&a, &a]);

    let multiplications = set::Hom::new(&a_x_a, &a);
    let count = multiplications
        .iter()
        .filter(|f| is_associative(&f, n))
        .count();
    println!("Count = {:?}", count);

    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}
