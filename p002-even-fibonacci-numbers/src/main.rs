#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

fn fib_recursive(n: u32) -> u64 {
    match n {
        0 => 1,
        1 => 2,
        _ => fib_recursive(n - 1) + fib_recursive(n - 2),
    }
}

fn solve(upper_bound: Option<u32>) -> u64 {
    let mut i = 0;
    let mut fib_num: u64 = fib_recursive(i);
    let mut res: Vec<u64> = Vec::new();

    while fib_num < 4000000
        && match upper_bound {
            Some(bound) => i < bound,
            None => true,
        }
    {
        if fib_num % 2 == 0 {
            res.push(fib_num);
        }
        i = i + 1;
        fib_num = fib_recursive(i);
    }
    res.iter().sum()
}

fn main() {
    println!("{}", solve(None));
}

#[cfg(test)]
mod tests {
    #[test]
    fn first_fib() {
        assert_eq!(1, crate::fib_recursive(0));
    }

    #[test]
    fn second_fib() {
        assert_eq!(2, crate::fib_recursive(1));
    }

    #[test]
    fn fifth_fib() {
        assert_eq!(8, crate::fib_recursive(4));
    }

    #[test]
    fn sum_first_ten_terms() {
        assert_eq!(44, crate::solve(Some(10)));
    }
}
