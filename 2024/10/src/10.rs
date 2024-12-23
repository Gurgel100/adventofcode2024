use std::{cell::RefCell, cmp::Ordering, collections::HashSet, hash::Hash, rc::Rc};

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    let nodes = parse_input(input);
    nodes
        .iter()
        .flatten()
        .filter(|n| n.get_value() == 0)
        .map(|n| n.count_reachable_nodes(9))
        .sum::<usize>()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let nodes = parse_input(input);
    nodes
        .iter()
        .flatten()
        .filter(|n| n.get_value() == 0)
        .map(|n| n.count_ways_to_target(9))
        .sum::<usize>()
}

fn parse_input(input: aoc::Input) -> Vec<Vec<Rc<Node>>> {
    let mut nodes = vec![];
    for (y, line) in input.lines().enumerate() {
        nodes.push(vec![]);
        for (x, c) in line.char_indices() {
            let height = c.to_digit(10).unwrap();
            let node = Rc::new(Node::new((y << 32) + x, height));
            nodes[y].push(node.clone());
            if y > 0 {
                node.link(&nodes[y - 1][x]);
            }
            if x > 0 {
                node.link(&nodes[y][x - 1]);
            }
        }
    }
    nodes
}

trait LinkNode {
    fn link(&self, other: &Self);
    fn unique_reachable_nodes(&self, target_value: u32, set: &mut HashSet<Self>)
    where
        Self: Sized;

    fn count_reachable_nodes(&self, target_value: u32) -> usize
    where
        Self: Sized,
    {
        let mut set = HashSet::default();
        self.unique_reachable_nodes(target_value, &mut set);
        set.len()
    }
}

#[derive(Debug, Eq)]
struct Node {
    id: usize,
    value: u32,
    next: RefCell<Vec<Rc<Self>>>,
}
impl Node {
    fn new(id: usize, value: u32) -> Self {
        Self {
            id,
            value,
            next: vec![].into(),
        }
    }

    fn set_next(&self, next: Rc<Self>) {
        self.next.borrow_mut().push(next);
    }

    fn get_value(&self) -> u32 {
        self.value
    }

    fn count_ways_to_target(&self, target_value: u32) -> usize {
        if self.value == target_value {
            1
        } else {
            self.next
                .borrow()
                .iter()
                .map(|n| n.count_ways_to_target(target_value))
                .sum()
        }
    }
}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
impl LinkNode for Rc<Node> {
    fn link(&self, other: &Self) {
        if self.value.abs_diff(other.get_value()) == 1 {
            match self.value.cmp(&other.get_value()) {
                Ordering::Less => self.set_next(other.clone()),
                Ordering::Equal => unreachable!(),
                Ordering::Greater => other.set_next(self.clone()),
            }
        }
    }

    fn unique_reachable_nodes(&self, target_value: u32, set: &mut HashSet<Self>) {
        if self.value == target_value {
            set.insert(self.clone());
        } else {
            for next in self.next.borrow().iter() {
                next.unique_reachable_nodes(target_value, set);
            }
        }
    }
}
