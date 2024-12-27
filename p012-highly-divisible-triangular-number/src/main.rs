#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use divisors::get_divisors;

fn get_divisors1(n: u128) -> Vec<u128> {
    match n {
        1 => vec![1],
        3 => vec![1, 3],
        _ => {
            let mut divisors = get_divisors(n);
            divisors.extend(vec![1, n]);
            divisors
        }
    }
}

fn solve(n_divisors: usize) -> u128 {
    let mut triangular_num = 1;

    for i in 2.. {
        let current_divisors = get_divisors1(triangular_num);
        if current_divisors.len() > n_divisors {
            break
        }
        triangular_num += i;
    }

    triangular_num
}

fn main() {
    println!("{}", solve(500));
}

#[cfg(test)]
mod tests {
    #[test]
    fn triangular_num_with_five_divisors() {
        assert_eq!(28, super::solve(5));
    }
}
