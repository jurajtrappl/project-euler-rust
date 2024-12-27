#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use malachite::num::arithmetic::traits::Pow;
use malachite::Natural;

fn solve(n: u64, power: u64) -> u64 {
    let number = Natural::from(n);
    let power = number.pow(power);
    power
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .sum::<u32>() as u64
}

fn main() {
    println!("{}", solve(2, 1000));
}

#[cfg(test)]
mod tests {
    #[test]
    fn digit_sum_two_to_fifteen() {
        assert_eq!(26, super::solve(2, 15));
    }
}
