use aoc::{IterUnwrap, Parse};
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    str::FromStr,
};

aoc::parts!(1, 2);

struct PageOrderRule(u32, u32);
impl FromStr for PageOrderRule {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split('|').map(str::parse_uw);
        Ok(Self(s.next_uw(), s.next_uw()))
    }
}

#[derive(Default, Debug)]
struct PageOrder {
    // We always store a map of which pages need to be printed before the key
    rules: HashMap<u32, HashSet<u32>>,
}
impl PageOrder {
    fn parse_rules<'a>(lines: &mut impl Iterator<Item = &'a str>) -> Self {
        Self {
            rules: lines
                .map_while(|line| {
                    if line.is_empty() {
                        None
                    } else {
                        Some(line.parse_uw::<PageOrderRule>())
                    }
                })
                .fold(HashMap::default(), |mut rules, rule| {
                    rules.entry(rule.0).or_default().insert(rule.1);
                    rules
                }),
        }
    }

    fn correct_order(&self, first_page: u32, second_page: u32) -> bool {
        self.rules
            .get(&first_page)
            .map_or(false, |p| p.contains(&second_page))
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    let mut lines = input.into_iter();
    let page_order = PageOrder::parse_rules(&mut lines);

    lines
        .filter_map(|line| {
            let pages: Vec<u32> = line.split(',').map(str::parse_uw).collect();
            pages
                .is_sorted_by(|l, r| page_order.correct_order(*l, *r))
                .then(|| pages[pages.len() / 2])
        })
        .sum::<u32>()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut lines = input.into_iter();
    let page_order = PageOrder::parse_rules(&mut lines);

    lines
        .filter_map(|line| {
            let mut pages: Vec<u32> = line.split(',').map(str::parse_uw).collect();
            (!pages.is_sorted_by(|l, r| page_order.correct_order(*l, *r))).then(|| {
                pages.sort_unstable_by(|l, r| {
                    if page_order.correct_order(*l, *r) {
                        Ordering::Less
                    } else if page_order.correct_order(*r, *l) {
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    }
                });
                pages[pages.len() / 2]
            })
        })
        .sum::<u32>()
}
