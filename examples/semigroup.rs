use std::time::Instant;
use pshcalc::prelude::*;
use pshcalc::set::basic_set::BasicSet;
use pshcalc::set::hom_set::HomSet;
use pshcalc::set::product_set::ProductSet;

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
    let a = BasicSet::new(n);
    let a_x_a = ProductSet::new(&[&a, &a]);

    let multiplications = HomSet::new(
        &a_x_a,
        &a
    );
    let count = multiplications.iter().filter(
        |f| is_associative(&f, n)
    ).count();
    println!("Count = {:?}", count);

    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}
