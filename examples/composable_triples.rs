use pshcalc::set::{AtomSet, FunctionHandle, HomSet, Variable, World};
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let mut world = World::new();

    // Set sizes
    let m_size = 8; // Size of set M
    let o_size = 3; // Size of set O

    let m = AtomSet::new(m_size);
    let o = AtomSet::new(o_size);
    let morphisms = HomSet::new(&m, &o); // Morphisms M → O

    println!("Computing average number of composable triples...");
    println!("M has {} elements, O has {} elements", m_size, o_size);
    println!(
        "Total number of morphism pairs (s,t): {}",
        morphisms.size().pow(2)
    );

    let mut total_composable_triples = 0u64;
    let mut total_morphism_pairs = 0u64;

    // Allocate all variables once outside the loops for better performance
    let mut s_var = morphisms.create_variable(&mut world);
    let mut t_var = morphisms.create_variable(&mut world);

    // Progress indicator for long computations
    let progress_interval = (morphisms.size() / 10).max(1) as u64;

    s_var.initialize(&mut world);

    while let Some(s) = s_var.get() {
        // Reset and iterate over all morphisms t: M → O
        t_var.initialize(&mut world);

        while let Some(t) = t_var.get() {
            let composable_count = count_composable_triples(&world, s, t, &m);
            total_composable_triples += composable_count as u64;
            total_morphism_pairs += 1;

            t_var.advance(&mut world);
        }

        s_var.advance(&mut world);

        if total_morphism_pairs % progress_interval == 0 {
            let progress = (total_morphism_pairs as f64)
                / (morphisms.size().pow(2) as f64)
                * 100.0;
            println!("Progress: {:.1}%", progress);
        }
    }

    let average = total_composable_triples as f64 / total_morphism_pairs as f64;

    println!("\nResults:");
    println!("Total morphism pairs analyzed: {}", total_morphism_pairs);
    println!(
        "Total composable triples found: {}",
        total_composable_triples
    );
    println!("Average number of composable triples: {:.6}", average);

    // Theoretical expectation for comparison
    let theoretical_average =
        theoretical_expected_composable_triples(m_size, o_size);
    println!("Theoretical expectation: {:.6}", theoretical_average);
    println!("Difference: {:.6}", (average - theoretical_average).abs());

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}

/// Count composable triples (a,b,c) for fixed morphisms s and t
/// A triple (a,b,c) is composable if s(a) = t(b) and s(b) = t(c)
fn count_composable_triples(
    world: &World,
    s: &FunctionHandle,
    t: &FunctionHandle,
    m: &AtomSet,
) -> usize {
    let mut count = 0;

    // Clean iterator syntax with StackAtom for zero-allocation performance
    for a in m {
        let s_a = s.apply(world, &a); // s(a) returns StackAtom

        for b in m {
            let t_b = t.apply(world, &b); // t(b) returns StackAtom

            // Early exit: if s(a) != t(b), no need to check any 'c'
            if s_a != t_b {
                continue;
            }

            let s_b = s.apply(world, &b); // s(b) returns StackAtom

            for c in m {
                let t_c = t.apply(world, &c); // t(c) returns StackAtom

                // Check second composability condition: s(b) = t(c)
                if s_b == t_c {
                    count += 1;
                }
            }
        }
    }

    count
}

/// Calculate theoretical approximate number of composable triples
/// For random morphisms s,t: M → O, the probability that s(a) = t(b) is approx. 1/|O|
/// Similarly, the probability that s(b) = t(c) is approx. 1/|O|
/// Since these events are independent for different pairs, the approximate number
/// of composable triples is |M|³ × (1/|O|)²
fn theoretical_expected_composable_triples(
    m_size: usize,
    o_size: usize,
) -> f64 {
    let m_cubed = (m_size as f64).powi(3);
    let o_squared = (o_size as f64).powi(2);
    m_cubed / o_squared
}
