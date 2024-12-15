use std::collections::{HashMap, HashSet};

aoc::parts!(1, 2);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
struct Guard {
    x: usize,
    y: usize,
    direction: Direction,
}
impl Guard {
    fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            direction: Direction::Up,
        }
    }

    fn do_move(&mut self) {
        (self.x, self.y) = self.get_next_position();
    }

    fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        };
    }

    fn get_next_position(&self) -> (usize, usize) {
        // The origin is the top left point which means:
        // X increases to the left
        // Y increases downwards
        match self.direction {
            Direction::Up => (self.x, self.y - 1),
            Direction::Down => (self.x, self.y + 1),
            Direction::Left => (self.x - 1, self.y),
            Direction::Right => (self.x + 1, self.y),
        }
    }
}

#[allow(unused)]
fn print_map_path(map: &[&str], visited: &[Vec<bool>]) {
    for (map_row, visited_row) in map.into_iter().zip(visited) {
        for (map_col, visited_col) in map_row.chars().zip(visited_row) {
            if *visited_col {
                print!("X");
            } else {
                print!("{map_col}");
            }
        }
        println!("");
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    let lines = input.as_lines();
    let mut visited = lines
        .iter()
        .map(|line| vec![false; line.len()])
        .collect::<Vec<_>>();
    let mut guard = lines
        .iter()
        .enumerate()
        .find_map(|(y, line)| line.find('^').map(|x| Guard::new(x, y)))
        .unwrap();

    let map = lines;
    let map_height = map.len();
    let map_width = map[0].len();

    loop {
        // Mark current position
        visited[guard.y][guard.x] = true;
        let (next_x, next_y) = guard.get_next_position();
        if next_x >= map_width || next_y >= map_height {
            break;
        }
        if &map[next_y][next_x..=next_x] == "#" {
            guard.turn_right();
        }
        guard.do_move();
    }

    visited
        .into_iter()
        .map(|v| v.into_iter().map(|b| b as u32).sum::<u32>())
        .sum::<u32>()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let lines = input.as_lines();
    let mut visited = lines
        .iter()
        .map(|line| vec![HashSet::default(); line.len()])
        .collect::<Vec<_>>();
    let mut guard = lines
        .iter()
        .enumerate()
        .find_map(|(y, line)| line.find('^').map(|x| Guard::new(x, y)))
        .unwrap();

    let starting_position = (guard.x, guard.y);

    let map = lines;
    let map_height = map.len();
    let map_width = map[0].len();

    let mut possible_obstacles = HashSet::new();
    loop {
        // Mark current position
        visited[guard.y][guard.x].insert(guard.direction);
        let (next_x, next_y) = guard.get_next_position();
        if next_x >= map_width || next_y >= map_height {
            break;
        }
        if &map[next_y][next_x..=next_x] == "#" {
            guard.turn_right();
        } else {
            // Simulate that there is a obstruction
            let mut sim_guard = guard.clone();
            sim_guard.turn_right();
            if find_path(
                map,
                map_width,
                map_height,
                (next_x, next_y),
                &visited,
                sim_guard,
            ) {
                possible_obstacles.insert((next_x, next_y));
            }
        }
        guard.do_move();
    }

    possible_obstacles.remove(&starting_position);

    possible_obstacles.len()
}

fn find_path(
    map: &[&str],
    map_width: usize,
    map_height: usize,
    sim_obs: (usize, usize),
    visited: &[Vec<HashSet<Direction>>],
    mut guard: Guard,
) -> bool {
    let mut sim_visited: HashMap<(usize, usize), HashSet<Direction>> = HashMap::new();
    loop {
        if visited[guard.y][guard.x].contains(&guard.direction)
            || sim_visited
                .get(&(guard.x, guard.y))
                .map_or(false, |directions| directions.contains(&guard.direction))
        {
            return true;
        }
        let (next_x, next_y) = guard.get_next_position();
        if next_x >= map_width || next_y >= map_height {
            break;
        }
        if &map[next_y][next_x..=next_x] == "#" || sim_obs == (next_x, next_y) {
            guard.turn_right();
            if visited[guard.y][guard.x].contains(&guard.direction) {
                return true;
            }
        }

        sim_visited
            .entry((guard.x, guard.y))
            .or_default()
            .insert(guard.direction);
        guard.do_move();
    }
    false
}
