#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

fn solve(upper_bound: u32) -> u32 {
    (1..upper_bound).filter(|&n| n % 3 == 0 || n % 5 == 0).sum()
}

fn main() {
    println!("{}", solve(1000));
}

#[cfg(test)]
mod tests {
    #[test]
    fn sum_below_ten() {
        assert_eq!(23, crate::solve(10));
    }
}