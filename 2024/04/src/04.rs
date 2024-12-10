aoc::parts!(1, 2);

const PAT: &str = "XMAS";

fn part_1(input: aoc::Input) -> impl ToString {
    let lines = input.as_lines();
    // let horinzontal_count: usize = lines
    //     .iter()
    //     .map(|l| l.matches(PAT).count() + l.chars().rev().collect::<String>().matches(PAT).count())
    //     .sum();
    // let vertical_count: usize = lines
    //     .iter()
    //     .enumerate()
    //     .map(|(i, _)| {
    //         let vertical_line = lines
    //             .iter()
    //             .filter_map(move |l| l.chars().nth(i))
    //             .collect::<String>();
    //         vertical_line.matches(PAT).count()
    //             + vertical_line
    //                 .chars()
    //                 .rev()
    //                 .collect::<String>()
    //                 .matches(PAT)
    //                 .count()
    //     })
    //     .sum();
    // horinzontal_count + vertical_count

    let directions = [
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];

    lines
        .iter()
        .enumerate()
        .map(|(i, line)| {
            line.char_indices()
                .map(|(j, _)| {
                    directions
                        .map(|(dx, dy)| {
                            PAT.char_indices().all(|(k, p)| {
                                lines.get((i as isize + k as isize * dy) as usize).map_or(
                                    false,
                                    |l| {
                                        l.chars()
                                            .nth((j as isize + k as isize * dx) as usize)
                                            .map_or(false, |c| c == p)
                                    },
                                )
                            }) as u32
                        })
                        .into_iter()
                        .sum::<u32>()
                })
                .sum::<u32>()
        })
        .sum::<u32>()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let lines = input.as_lines();

    let directions = [(1, 1), (1, -1)];

    lines
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.char_indices()
                .filter_map(|(x, ch)| {
                    (ch == 'A').then(|| {
                        (directions
                            .iter()
                            .chain(&directions.map(|(x, y)| (x * -1, y * -1)))
                            .filter(|(dx, dy)| {
                                lines.get((y as isize + dy) as usize).map_or(false, |l| {
                                    l.chars()
                                        .nth((x as isize + dx) as usize)
                                        .map_or(false, |c| c == 'M')
                                }) && lines.get((y as isize - dy) as usize).map_or(false, |l| {
                                    l.chars()
                                        .nth((x as isize - dx) as usize)
                                        .map_or(false, |c| c == 'S')
                                })
                            })
                            .count()
                            == 2) as u32
                    })
                })
                .sum::<u32>()
        })
        .sum::<u32>()
}
