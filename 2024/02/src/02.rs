use aoc::Parse;
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

aoc::parts!(1, 2);

enum SortDirection {
    None,
    Ascending,
    Descending,
}

fn part_1(input: aoc::Input) -> impl ToString {
    input
        .into_iter()
        .map(|line| {
            is_safe_report(
                &line
                    .split_whitespace()
                    .map(|v| v.parse_uw::<i32>())
                    .collect::<Vec<_>>(),
                0,
            )
        })
        .filter(|v| *v)
        .count()
}

fn part_2(input: aoc::Input) -> impl ToString {
    input
        .into_iter()
        .map(|line| {
            is_safe_report(
                &line
                    .split_whitespace()
                    .map(|v| v.parse_uw::<i32>())
                    .collect::<Vec<_>>(),
                1,
            )
        })
        .filter(|v| *v)
        .count()
}

fn is_safe_report(report: &[i32], tolerate: usize) -> bool {
    let res = !report
        .into_iter()
        .tuple_windows()
        .fold_while(SortDirection::None, |sort_direction, (a, b)| {
            if (a - b).abs() <= 3 {
                match sort_direction {
                    SortDirection::None => {
                        if a < b {
                            Continue(SortDirection::Ascending)
                        } else if a > b {
                            Continue(SortDirection::Descending)
                        } else {
                            Done(sort_direction)
                        }
                    }
                    SortDirection::Ascending => {
                        if a < b {
                            Continue(sort_direction)
                        } else {
                            Done(sort_direction)
                        }
                    }
                    SortDirection::Descending => {
                        if a > b {
                            Continue(sort_direction)
                        } else {
                            Done(sort_direction)
                        }
                    }
                }
            } else {
                Done(sort_direction)
            }
        })
        .is_done();
    if !res && tolerate > 0 {
        // Remove an element and try again
        (0..report.len()).any(|i| {
            is_safe_report(
                &report[..i]
                    .into_iter()
                    .chain(report[(i + 1)..].into_iter())
                    .copied()
                    .collect::<Vec<_>>(),
                tolerate - 1,
            )
        })
    } else {
        res
    }
}
