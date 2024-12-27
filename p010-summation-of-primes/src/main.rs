#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use primes::{PrimeSet, Sieve};

fn solve(bound: u64) -> u64 {
    let mut pset = Sieve::new();
    pset.iter().take_while(|&p| p < bound).sum::<u64>()
}

fn main() {
    println!("{}", solve(2_000_000))
}

#[cfg(test)]
mod tests {
    #[test]
    fn sum_below_10() {
        assert_eq!(17, super::solve(10));
    }
}
