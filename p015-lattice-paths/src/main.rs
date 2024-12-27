#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

fn solve(r_dim: usize, c_dim: usize) -> u64 {
    let mut lattice = vec![vec![0; c_dim + 1]; r_dim + 1];
    for r in 0..r_dim + 1 {
        for c in 0..c_dim + 1 {
            if r == 0 || c == 0 {
                lattice[r][c] = 1;
            }
        }
    }

    for r in 1..r_dim + 1{
        for c in 1..c_dim + 1 {
            lattice[r][c] += lattice[r - 1][c] + lattice[r][c - 1]
        }
    }
    
    lattice[r_dim][c_dim]
}

fn main() {
    println!("{}", solve(20, 20));
}

#[cfg(test)]
mod tests {
    #[test]
    fn lattice_two_by_two() {
        assert_eq!(6, super::solve(2, 2));
    }
}
