#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use core::ops::Range;

fn solve(count: u32) -> u32 {
    let nums: Range<u32> = 1..count + 1;
    let sum_of_squares: u32 = nums.clone().map(|x| x * x).sum();
    let whole_squared: u32 = nums.sum::<u32>().pow(2);
    whole_squared - sum_of_squares
}

fn main() {
    println!("{}", solve(100));
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn difference_for_first_ten() {
        assert_eq!(2640, solve(10));
    }
}