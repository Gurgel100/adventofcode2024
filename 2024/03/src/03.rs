use aoc::Parse;
use regex::Regex;
use std::collections::BTreeSet;

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    input
        .lines()
        .map(|line| {
            re.captures_iter(line)
                .map(|c| {
                    (1..=2)
                        .into_iter()
                        .map(|i| c.get(i).unwrap().as_str().parse_uw::<u32>())
                        .product::<u32>()
                })
                .sum::<u32>()
        })
        .sum::<u32>()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let re_mul = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let re_do = Regex::new(r"do\(\)").unwrap();
    let re_dont = Regex::new(r"don't\(\)").unwrap();
    let line = input.to_string();
    let activate = re_do
        .find_iter(&line)
        .map(|m| m.start())
        .collect::<BTreeSet<_>>();
    let deactivate = re_dont
        .find_iter(&line)
        .map(|m| m.start())
        .collect::<BTreeSet<_>>();
    re_mul
        .captures_iter(&line)
        .filter_map(|c| {
            let m = c.get(0).unwrap();
            let activated_at = activate.range(..m.start()).next_back();
            let deactivated_at = deactivate.range(..m.start()).next_back();
            let do_mul = match (activated_at, deactivated_at) {
                (None, None) => true,
                (None, Some(_)) => false,
                (Some(_), None) => true,
                (Some(a), Some(d)) => a > d,
            };

            do_mul.then(|| {
                (1..=2)
                    .into_iter()
                    .map(|i| c.get(i).unwrap().as_str().parse_uw::<u32>())
                    .product::<u32>()
            })
        })
        .sum::<u32>()
}
