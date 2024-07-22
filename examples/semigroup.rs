use std::time::Instant;
use pshcalc::prelude::*;
use pshcalc::set::hom_set::HomSet;

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
    let n: usize = 4;

    let multiplications = HomSet::new(
        n*n,
        n
    );
    let count = multiplications.iter().filter(
        |s| is_associative(s, n)
    ).count();
    println!("Count = {:?}", count);

    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}
