use pshcalc::set::{
    AssociativityChecker, AtomSet, HomSet, ProductSet, Variable, World,
};
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let mut world = World::new();
    let n = 4;

    let a = AtomSet::new(n);
    let a_x_a = ProductSet::new(&[a.clone(), a.clone()]);
    let multiplications = HomSet::new(&a_x_a, &a);

    // Zero-allocation checker
    let mut checker = AssociativityChecker::new(&mut world, n);

    let mut function = multiplications.create_variable(&mut world);
    function.initialize(&mut world);

    let mut count = 0;
    while let Some(f) = function.get() {
        if checker.is_associative(&world, f) {
            // if ultra_checker.is_associative(&world, f) {
            count += 1;
        }
        function.advance(&mut world);
    }

    println!("Count = {:?}", count);
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}
