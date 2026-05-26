use pshcalc::cat::monoids;
use pshcalc::set::Set;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let n = 4;

    println!("Counting monoids with {} elements...", n);

    let start = Instant::now();

    let monoid_set = monoids(n); 

    let mut count = 0;
    let mut monoid = monoid_set.allocate();
    monoid_set.reset(&mut monoid);
    while monoid.ongoing {
        count += 1;

        if count % 100 == 0 {
            println!("  Found {} so far...", count);
        }
        monoid_set.next(&mut monoid);
    }
    let duration = start.elapsed();

    println!("Found {} monoids on {} elements", count, n);
    println!("Time elapsed: {:.2?}", duration);

    Ok(())
}
