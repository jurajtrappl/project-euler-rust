#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

type Calendar = Vec<(u32, u32, u32)>;

fn is_leap_year(y: u32) -> bool {
    (y % 4 == 0 && y % 100 != 0) || y % 400 == 0
}

fn generate_days() -> Calendar {
    let mut result = Vec::new();

    for y in 1900..=2000 {
        for m in 1..=12 {
            for d in 1..=31 {
                result.push((y, m, d));
            }
        }
    }

    result
}

fn adjust_thirty_days(dates: Calendar) -> Calendar {
    dates
        .into_iter()
        .filter(|&(_, m, d)| !(d == 31 && (m == 4 || m == 6 || m == 9 || m == 11)))
        .collect()
}

fn adjust_february(dates: Calendar) -> Calendar {
    dates
        .into_iter()
        .filter(|&(y, m, d)| {
            if m == 2 {
                if is_leap_year(y) {
                    d <= 29
                } else {
                    d <= 28
                }
            } else {
                true
            }
        })
        .collect()
}

fn is_sunday(n: usize) -> bool {
    n % 7 == 6
}

fn main() {
    let dates = generate_days();
    let thirdy_days_months_adjusted = adjust_thirty_days(dates);
    let feb_adjusted = adjust_february(thirdy_days_months_adjusted);

    let sundays_on_first_of_month: Calendar = feb_adjusted
        .into_iter()
        .enumerate()
        .filter(|&(i, (_, _, d))| d == 1 && is_sunday(i))
        .map(|(_, date)| date)
        .collect();

    println!("{}", sundays_on_first_of_month.len());
}
