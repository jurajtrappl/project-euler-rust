#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

fn main() {
    for a in 1..=1000 {
        for b in 1..=1000 {
            for c in 1..=1000 {
                if a + b + c == 1000 && (a * a + b * b == c * c) {
                    println!("Found triplet: {}, {}, {}. Product: {}", a, b, c, a * b * c);
                }
            }
        }
    }
}
