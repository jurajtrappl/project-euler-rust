#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

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

fn main() {
    let mut amicable_sum = 0;
    for i in 1..=10_000 {
        let sum_i = sum_of_proper_divisors(i);
        if sum_i > i && sum_i <= 10_000 {
            if sum_of_proper_divisors(sum_i) == i {
                amicable_sum += i + sum_i;
            }
        }
    }

    println!("{}", amicable_sum);
}
