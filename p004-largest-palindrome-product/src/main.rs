#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

fn is_palindrome(n: u32) -> bool {
    let m: String = n.to_string();
    m == m.chars().rev().collect::<String>()
}

fn solve(lower_bound: u32, upper_bound: u32) -> u32 {
    let nums1: Vec<u32> = (lower_bound..upper_bound).rev().collect::<Vec<u32>>();
    let nums2 = nums1.clone();
    let mut biggest_palindrome = 0;

    for n in nums1 {
        for m in &nums2 {
            let mult = n * m;
            if is_palindrome(mult) {
                if mult > biggest_palindrome {
                    biggest_palindrome = mult;
                }
            }
        }
    }

    biggest_palindrome
}

fn main() {
    println!("{}", solve(100, 1000));
}

#[cfg(test)]
mod tests {
    use crate::{is_palindrome, solve};

    #[test]
    fn test_is_palindrome() {
        assert_eq!(true, is_palindrome(90109));
        assert_eq!(true, is_palindrome(1234554321));
        assert_eq!(false, is_palindrome(123));
        assert_eq!(false, is_palindrome(123421));
    }

    #[test]
    fn largest_two_digit() {
        assert_eq!(9009, solve(10, 100));
    }
}