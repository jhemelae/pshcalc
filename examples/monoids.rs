use pshcalc::cat::CategoryVariable;
use pshcalc::set::World;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let element_count = 4;
    println!("Counting monoids with {} elements...", element_count);

    let start = Instant::now();
    let monoid_count = count_monoids(element_count);
    let duration = start.elapsed();

    println!(
        "Found {} monoids on {} elements",
        monoid_count, element_count
    );
    println!("Time elapsed: {:.2?}", duration);

    Ok(())
}

#[inline(always)]
fn count_monoids(n: usize) -> usize {
    let mut world = World::new();

    let mut category_var = CategoryVariable::new(&mut world, 1, n);

    let mut count = 0;
    while let Some(category) = category_var.get(&world) {
        if category.validate().is_ok() {
            count += 1;
            if count % 50 == 0 {
                println!("  Found {} monoids so far...", count);
            }
        }
        category_var.advance(&mut world);
    }

    count
}
