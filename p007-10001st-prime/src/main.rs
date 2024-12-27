#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use primes::{Sieve, PrimeSet};

fn solve(n: usize) -> u64 {
    let mut pset = Sieve::new();
    pset.iter().nth(n - 1).unwrap()
}
 
fn main() {
    println!("{}", solve(10001));
}

#[cfg(test)]
mod tests {
    #[test]
    fn sixth_prime() {
        assert_eq!(13, super::solve(6));
    }
}