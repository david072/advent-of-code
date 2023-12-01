use std::collections::HashMap;

pub fn main() -> utils::Result<()> {
    let input = utils::get_input(2022, 8)?;

    println!("Part One: {}", solve1(&input));
    println!("Part Two: {}", solve2(&input));

    Ok(())
}

fn solve1(input: &str) -> u64 {
    let map = parse_map(input);

    let is_visible_in_direction = |start: (isize, isize), direction: (isize, isize), tree_height: u32| {
        let mut coord = (start.0 + direction.0, start.1 + direction.1);
        while let Some(height) = map.get(&coord) {
            if *height >= tree_height { return false; }
            coord.0 += direction.0;
            coord.1 += direction.1;
        }
        true
    };

    map.iter()
        .map(|(coord, tree_height)| {
            u64::from(is_visible_in_direction(*coord, (-1, 0), *tree_height) ||
                is_visible_in_direction(*coord, (1, 0), *tree_height) ||
                is_visible_in_direction(*coord, (0, 1), *tree_height) ||
                is_visible_in_direction(*coord, (0, -1), *tree_height))
        })
        .sum()
}

fn solve2(input: &str) -> usize {
    let map = parse_map(input);

    let distance_in_direction = |start_coord: (isize, isize), direction: (isize, isize), tree_height: u32| {
        let mut coord = (start_coord.0 + direction.0, start_coord.1 + direction.1);
        let mut distance = 0usize;
        while let Some(height) = map.get(&coord) {
            if *height >= tree_height { return distance + 1; }
            distance += 1;
            coord.0 += direction.0;
            coord.1 += direction.1;
        }

        distance
    };

    map.iter()
        .map(|(coord, tree_height)| {
            let left = distance_in_direction(*coord, (-1, 0), *tree_height);
            let right = distance_in_direction(*coord, (1, 0), *tree_height);
            let up = distance_in_direction(*coord, (0, -1), *tree_height);
            let down = distance_in_direction(*coord, (0, 1), *tree_height);
            left * right * up * down
        })
        .max()
        .unwrap()
}

fn parse_map(input: &str) -> HashMap<(isize, isize), u32> {
    let mut map = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, height) in line.chars().enumerate() {
            let Some(height) = height.to_digit(10) else { continue; };
            map.insert((x as isize, y as isize), height);
        }
    }

    map
}
