#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use std::collections::HashMap;

fn create_word_desc() -> HashMap<u32, &'static str> {
    HashMap::from([
        (0, "zero"),
        (1, "one"),
        (2, "two"),
        (3, "three"),
        (4, "four"),
        (5, "five"),
        (6, "six"),
        (7, "seven"),
        (8, "eight"),
        (9, "nine"),
        (10, "ten"),
        (11, "eleven"),
        (12, "twelve"),
        (13, "thirteen"),
        (14, "fourteen"),
        (15, "fifteen"),
        (16, "sixteen"),
        (17, "seventeen"),
        (18, "eighteen"),
        (19, "nineteen"),
        (20, "twenty"),
        (30, "thirty"),
        (40, "forty"),
        (50, "fifty"),
        (60, "sixty"),
        (70, "seventy"),
        (80, "eighty"),
        (90, "ninety"),
        (100, "hundred"),
        (1000, "thousand"),
    ])
}

fn convert_to_words(word_letters: &HashMap<u32, &'static str>, n: u32) -> Vec<String> {
    match n {
        0 => vec![word_letters[&0].to_string()],
        1000 => vec![
            word_letters[&1].to_string(),
            word_letters[&1000].to_string(),
        ],
        n if n % 100 == 0 => vec![
            word_letters[&(n / 100)].to_string(),
            word_letters[&100].to_string(),
        ],
        n if n > 100 => {
            let mut words = vec![
                word_letters[&(n / 100)].to_string(),
                word_letters[&100].to_string(),
                "and".to_string(),
            ];
            words.extend(convert_to_words(word_letters, n - (n / 100) * 100));
            words
        },
        n if n >= 21 => {
            let mut words = vec![word_letters[&((n / 10) * 10)].to_string()];
            if n % 10 != 0 {
                words.push(word_letters[&(n % 10)].to_string());
            }
            words
        },
        _ => vec![word_letters[&n].to_string()],
    }
}

fn solve(upper_bound: u32) -> usize {
    let word_letters: HashMap<u32, &'static str> = create_word_desc();
    (1..=upper_bound)
        .map(|n| convert_to_words(&word_letters, n)
            .join("")
            .len())
        .sum::<usize>()
}

fn main() {
    println!("{}", solve(1000));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_convert_to_words() {
        let word_letters: HashMap<u32, &'static str> = create_word_desc();
        let test_cases = [
            (0, vec!["zero"]),
            (37, vec!["thirty", "seven"]),
            (500, vec!["five", "hundred"]),
        ];

        for (input, expected) in test_cases {
            assert_eq!(expected, convert_to_words(&word_letters, input));
        }
    }

    #[test]
    fn test_solve() {
        assert_eq!(19, solve(5));
    }
}
