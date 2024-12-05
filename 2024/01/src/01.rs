use aoc::IterUnwrap;
use fxhash::{FxHashMap, FxHashSet};

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    let (mut l, mut r): (Vec<_>, Vec<_>) = input.lines().map(|l| {
        let mut it = l.split_whitespace().map(|p| p.parse::<i32>().unwrap());
        (it.next_uw(), it.next_uw())
    }).unzip();
    l.sort();
    r.sort();
    l.into_iter().zip(r).map(|(l, r)| (l - r).abs()).sum::<i32>()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let (l_sum, r_sum) = input.lines().fold((FxHashSet::default(), FxHashMap::default()), |(mut l_sum, mut r_sum), line| {
        let mut it = line.split_whitespace().map(|p| p.parse::<u32>().unwrap());
        l_sum.insert(it.next_uw());
        r_sum.entry(it.next_uw()).and_modify(|c| *c += 1).or_insert(1);
        (l_sum, r_sum)
    });
    l_sum.into_iter().filter_map(|k| r_sum.get(&k).map(|rc| k * rc)).sum::<u32>()
}