#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use std::collections::HashSet;

fn sum_of_proper_divisors(n: u32) -> u32 {
    let sqrt = (n as f64).sqrt() as u32;
    let mut sum = 1;
    
    for i in 2..=sqrt {
        if n % i == 0 {
            sum += i;
            
            if i != n/i {
                sum += n/i;
            }
        }
    }
    
    sum
}

fn is_abundant(n: u32) -> bool {
    sum_of_proper_divisors(n) > n
}

fn main() {
    let mut abundant_numbers = Vec::new();
    for n in 12..28_123 {
        if is_abundant(n) {
            abundant_numbers.push(n);
        }
    }

    let mut summed = HashSet::new();
    for (i, &a) in abundant_numbers.iter().enumerate() {
        for &b in abundant_numbers[i..].iter() {
            let sum = a + b;
            if sum <= 28123 {
                _ = summed.insert(sum);
            }
        }
    }
    
    let total: u32 = (1..=28123)
        .filter(|&n| !summed.contains(&n))
        .sum();

    println!("{}", total);
}
