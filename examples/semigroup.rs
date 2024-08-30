use std::time::Instant;
use pshcalc::prelude::*;
use pshcalc::set;

#[inline(always)]
fn get(s: &[usize], n: usize, i: usize, j: usize) -> usize {
        s[n * i + j]
}

#[inline(always)]
fn is_associative(a: &set::Basic, s: &[usize]) -> bool {
    a.iter().all(|i| {
        a.iter().all(|j| {
            a.iter().all(|k| {
                let n = a.size;
                let left = get(s, n, get(s, n, *i, *j), *k);
                let right = get(s, n, *i, get(s, n, *j, *k));
                left==right
            })
        })
    })
}

fn main() {
    let start = Instant::now();
    let n = 4;
    let a = set::Basic::new(n);
    let a_x_a = set::Product::new(&[&a, &a]);

    let multiplications = set::Hom::new(
        &a_x_a,
        &a
    );
    let count = multiplications.iter().filter(
        |f| is_associative(&a,&f)
    ).count();
    println!("Count = {:?}", count);

    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}
