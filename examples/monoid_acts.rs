use pshcalc::set::Set;
use pshcalc::cat::{monoids};
use pshcalc::psh::{VaryAction};
use std::time::Instant;

fn avg(n: usize, m: usize) -> f64 {
    println!(
        "Counting average number of monoid acts of size {} over monoids with {} elements...",
        m, n
    );
    let start = Instant::now();
    let mut total_acts = 0;
    let mut monoid_count = 0;
    let pi = vec![0; m];
    let monoid_set = monoids(n);
    let mut monoid = monoid_set.allocate();

    monoid_set.reset(&mut monoid);
    let presheaf_set = VaryAction::new(&monoid.value, &pi);
    let mut presheaf = presheaf_set.allocate();
    while monoid.ongoing {
        let mut act_count = 0;
        let presheaf_set = VaryAction::new(&monoid.value, &pi);
        presheaf_set.reset(&mut presheaf);
        while presheaf.ongoing {
            act_count += 1;
            presheaf_set.next(&mut presheaf);
        };
        println!(
            "Monoid {} has {} acts of size {}",
            monoid_count, act_count, m
        );
        total_acts += act_count;
        monoid_count += 1;
        monoid_set.next(&mut monoid);
    }
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
