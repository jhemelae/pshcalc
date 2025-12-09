use pshcalc::cat::CategorySet;
use pshcalc::cursor;
use pshcalc::set::{Cursor, Set};
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let n = 4;

    println!("Counting monoids with {} elements...", n);

    let start = Instant::now();

    let category_set = CategorySet::new(1, vec![0; n - 1], vec![0; n - 1]);

    let mut count = 0;
    cursor!(_ in category_set {
        count += 1;

        if count % 100 == 0 {
            println!("  Found {} so far...", count);
        }
    });
    let duration = start.elapsed();

    println!("Found {} monoids on {} elements", count, n);
    println!("Time elapsed: {:.2?}", duration);

    Ok(())
}
