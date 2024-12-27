#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

fn solve(divisors: Vec<u32>) -> u32 {
    for n in 1.. {
        if divisors.iter().all(|&d| n % d == 0) {
            return n
        }
    }

    unreachable!();
}

fn main() {
    let divisors = (1..21).collect::<Vec<u32>>();
    println!("{}", solve(divisors));
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn first_ten() {
        let divisors = (1..11).collect::<Vec<u32>>();
        assert_eq!(2520, solve(divisors));
    }
}