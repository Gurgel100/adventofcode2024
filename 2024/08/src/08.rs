use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Mul, Sub},
};

aoc::parts!(1);

fn part_1(input: aoc::Input) -> impl ToString {
    let map = {
        let lines = input.as_lines();
        Map {
            width: lines[0].len(),
            height: lines.len(),
        }
    };
    let antenna_groups = parse_antennas(input);
    // Calculate antinodes
    antenna_groups
        .values()
        .flat_map(|antenna_group| {
            antenna_group.iter().enumerate().flat_map(|(i, &antenna)| {
                antenna_group[i + 1..]
                    .iter()
                    .flat_map(move |&other_antenna| {
                        let vec = antenna - other_antenna;
                        [antenna + vec, other_antenna - vec]
                    })
            })
        })
        .filter(|p| map.in_bounds(p))
        .unique()
        .count()
}

// fn part_2(input: aoc::Input) -> impl ToString {
//     0
// }

fn parse_antennas(input: aoc::Input) -> HashMap<char, Vec<Position>> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.char_indices()
                .filter_map(|(x, c)| (c != '.').then_some((c, (x, y))))
                .collect::<Vec<_>>()
        })
        .fold(HashMap::default(), |mut collection, (c, coord)| {
            collection.entry(c).or_default().push(coord.into());
            collection
        })
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position {
    x: isize,
    y: isize,
}
impl Add<Vector> for Position {
    type Output = Self;

    fn add(self, rhs: Vector) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl Sub<Vector> for Position {
    type Output = Self;

    fn sub(self, rhs: Vector) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl Sub for Position {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl From<(usize, usize)> for Position {
    fn from((x, y): (usize, usize)) -> Self {
        Self {
            x: x as isize,
            y: y as isize,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Vector {
    x: isize,
    y: isize,
}
impl Mul<i32> for Vector {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x * rhs as isize,
            y: self.y * rhs as isize,
        }
    }
}
impl Vector {
    fn len(&self) -> f64 {
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
    }
}

#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
}
impl Map {
    fn in_bounds(&self, p: &Position) -> bool {
        p.x >= 0 && (p.x as usize) < self.width && p.y >= 0 && (p.y as usize) < self.height
    }
}

#[derive(Default)]
struct Unique<I: Iterator<Item: Clone + PartialEq + Eq + std::hash::Hash>> {
    iter: I,
    seen: HashSet<I::Item>,
}
impl<I: Iterator<Item: Clone + PartialEq + Eq + std::hash::Hash>> Unique<I> {
    fn new(iter: I) -> Self {
        Self {
            iter,
            seen: Default::default(),
        }
    }
}
impl<I: Iterator<Item: Clone + PartialEq + Eq + std::hash::Hash>> Iterator for Unique<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let i = self.iter.next()?;
            if self.seen.insert(i.clone()) {
                return Some(i);
            }
        }
    }
}

trait UniqueIterator {
    fn unique(self) -> Unique<Self>
    where
        Self: Iterator<Item: Clone + PartialEq + Eq + std::hash::Hash> + Sized;
}

impl<I: Iterator<Item: Clone + PartialEq + Eq + std::hash::Hash>> UniqueIterator for I {
    fn unique(self) -> Unique<Self>
    where
        Self: Iterator + Sized,
    {
        Unique::new(self)
    }
}
