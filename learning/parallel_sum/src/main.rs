use rayon::prelude::*;
use std::time::Instant;

fn main() {
    // Example large range of numbers
    let numbers: Vec<i64> = (1..=1_000_000).collect();

    // Measure sequential execution time
    let start = Instant::now();
    let sum_sequential: i64 = numbers.iter().sum();
    let duration = start.elapsed();
    println!("Sequential sum: {}, Time taken: {:?}", sum_sequential, duration);

    // Measure parallel execution time
    let start = Instant::now();
    let sum_parallel: i64 = numbers.par_iter().sum();
    let duration = start.elapsed();
    println!("Parallel sum: {}, Time taken: {:?}", sum_parallel, duration);
}
