#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use primes;

fn main() {
    println!("{}", primes::factors(600_851_475_143).last().unwrap());
}