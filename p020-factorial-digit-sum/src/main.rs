#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use std::collections::HashMap;
use num::BigUint;

fn factorial(n: &BigUint, memo: &mut HashMap<BigUint, BigUint>) -> BigUint {
    let two = BigUint::from(2u32);
    if n <= &two {
        return n.clone();
    }

    if let Some(result) = memo.get(n) {
        return result.clone();
    }

    let n_minus_one = n - 1u32;
    let result = n * factorial(&n_minus_one, memo);
    _ = memo.insert(n.clone(), result.clone());

    result
}

fn main() {
    let mut memo = HashMap::new();

    let one_hundred = BigUint::from(100u32);
    let one_hundred_factorial = factorial(&one_hundred, &mut memo);

    let digits_sum: u32 = one_hundred_factorial
        .to_string()
        .chars()
        .map(|digit| digit.to_digit(10).unwrap())
        .sum();
    println!("{}", digits_sum);
}
