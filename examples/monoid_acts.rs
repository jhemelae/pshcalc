use pshcalc::cat::Category;
use pshcalc::psh::{Presheaf, PresheafSet};
use pshcalc::traverse;
use std::time::Instant;

fn avg(n: usize, m: usize) -> f64 {
    println!("Counting average number of monoid acts of size {} over monoids with {} elements...", m, n);
    let start = Instant::now();
    let mut total_acts = 0;
    let mut monoid_count = 0;
    let mut monoid = Category::allocate(1, n);
    let mut presheaf = Presheaf::allocate(1, n, m);
    let pi = vec![0; m];
    let monoid_set =
        pshcalc::cat::CategorySet::new(1, vec![0; n - 1], vec![0; n - 1]);
    traverse!(monoid in &monoid_set => {
        let presheaf_set = PresheafSet::new(&monoid, &pi);
        let mut act_count = 0;
        traverse!(presheaf in &presheaf_set => {
            act_count += 1;
        });
        println!(
            "Monoid {} has {} acts of size {}",
            monoid_count, act_count, m
        );
        total_acts += act_count;
        monoid_count += 1;
    });
    let average_acts = total_acts as f64 / monoid_count as f64;
    let duration = start.elapsed();
    println!(
        "Average number of monoid acts of size {} over monoids with {} elements: {:.2}",
        m, n, average_acts
    );
    println!("Total monoids: {}", monoid_count);
    println!("Time elapsed: {:.2?}", duration);
    average_acts
}

fn main() {
    let mut results = vec![];
    for m in 1..=6 {
        let result = avg(3, m);
        results.push(result);
    }
}
