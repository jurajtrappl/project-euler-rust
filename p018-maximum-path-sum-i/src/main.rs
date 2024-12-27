#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use std::cmp::max;

const INPUT: &str = "75
                     95 64
                     17 47 82
                     18 35 87 10
                     20 04 82 47 65
                     19 01 23 75 03 34
                     88 02 77 73 07 63 67
                     99 65 04 28 06 16 70 92
                     41 41 26 56 83 40 80 70 33
                     41 48 72 33 47 32 37 16 94 29
                     53 71 44 65 25 43 91 52 97 51 14
                     70 11 33 28 77 73 17 78 39 68 17 57
                     91 71 52 38 17 14 91 43 58 50 27 29 48
                     63 66 04 68 89 53 67 30 73 16 69 87 40 31
                     04 62 98 27 23 09 70 98 73 93 38 53 60 04 23";

fn input_to_grid() -> Vec<Vec<u32>> {
    let input: String = INPUT.chars().collect();
    let mut lines: Vec<Vec<u32>> = input
        .split("\n")
        .map(|l| {
            l.trim()
                .split(" ")
                .map(|n| n.parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    let max_length = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    for line in lines.iter_mut() {
        line.resize(max_length, 0);
        line.insert(0, 0);
    }

    lines
}

fn main() {
    let mut grid = input_to_grid();
    let cols = grid[0].len();
    let rows = grid.len();

    for r in 1..rows {
        for c in 1..cols {
            grid[r][c] += max(grid[r - 1][c], grid[r - 1][c - 1]);
        }
    }

    let max_sum_number = grid.last().unwrap().iter().max().unwrap();
    println!("{:?}", max_sum_number);
}
