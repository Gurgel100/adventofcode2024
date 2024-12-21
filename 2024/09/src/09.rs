aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    let blocks = input
        .raw()
        .char_indices()
        .flat_map(|(i, l)| {
            let len = l.to_digit(10).unwrap();
            let file = if i % 2 == 0 { Some(i / 2) } else { None };
            vec![file; len as usize]
        })
        .collect::<Vec<_>>();
    let mut back_block = blocks.iter().enumerate().filter(|(_, b)| b.is_some()).rev();
    blocks
        .iter()
        .enumerate()
        .map_while(|(i, b)| {
            if let Some(file) = b {
                Some(*file * i)
            } else if let Some((j, block)) = back_block.next() {
                if j <= i {
                    None
                } else if let Some(file) = block {
                    Some(*file * i)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .sum::<usize>()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut blocks = input
        .raw()
        .char_indices()
        .map(|(i, l)| {
            let len = l.to_digit(10).unwrap();
            let file = if i % 2 == 0 { Some(i / 2) } else { None };
            (file, len as usize)
        })
        .collect::<Vec<_>>();
    let mut end_index = blocks.len() - 1;
    while end_index > 0 {
        let (block, len) = blocks[end_index];
        if let Some(block) = block {
            // Search for free space
            if let Some((target_index, target_len)) =
                blocks.iter().enumerate().find_map(|(i, (b, l))| {
                    (i < end_index && b.is_none() && *l >= len).then(|| (i, *l))
                })
            {
                blocks[end_index].0 = None;
                blocks[target_index] = (Some(block), len);
                if len < target_len {
                    blocks.insert(target_index + 1, (None, target_len - len));
                    end_index += 1;
                }
            }
        }
        end_index -= 1;
    }
    let mut current_index = 0;
    blocks
        .iter()
        .filter_map(|(b, l)| {
            let res = if let Some(id) = b {
                Some(id * (current_index..current_index + l).sum::<usize>())
            } else {
                None
            };
            current_index += l;
            res
        })
        .sum::<usize>()
}
