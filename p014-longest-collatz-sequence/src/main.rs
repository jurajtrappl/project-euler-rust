#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

fn collatz_sequence_len(n: u64) -> u32 {
    let mut res = 0;
    let mut m = n;
    while m != 1 {
        if m % 2 == 0 {
            m /= 2;
        } else {
            m = 3*m + 1
        }
        res += 1;
    }
    res
}

fn solve() -> u64 {
    let mut result = 1;
    let mut longest_len = 0;
    for i in 2..1_000_000 {
        let i_len = collatz_sequence_len(i);
        if i_len > longest_len {
            longest_len = i_len;
            result = i;
        }
    }

    result
}

fn main() {
    println!("{}", solve());
}

#[cfg(test)]
mod tests {
    #[test]
    fn collatz_for_thirteen() {
        assert_eq!(10, super::collatz_sequence_len(13));
    }
}