use std::cmp::{max, min};
use std::collections::HashMap;

pub fn main() -> utils::Result<()> {
    let input = utils::get_input(2022, 14)?;

    println!("Part One: {}", solve1(&input));
    println!("Part Two: {}", solve2(&input));

    Ok(())
}

/// Returns the map parsed from `input` as well as the maximum y coordinate of any rock formation
fn parse_map(input: &str) -> (HashMap<(usize, usize), bool>, usize) {
    let mut map: HashMap<(usize, usize), bool> = HashMap::new();
    let mut max_y = 0usize;

    for line in input.lines() {
        let parts = line.split(" -> ")
            .map(|part| {
                let mut components = part.split(',');
                (components.next()
                     .and_then(|n| n.parse::<usize>().ok())
                     .unwrap(),
                 components.next()
                     .and_then(|n| n.parse::<usize>().ok())
                     .unwrap())
            }).collect::<Vec<_>>();

        for coords in parts.windows(2) {
            let [first, second] = coords else { unreachable!(); };
            // vertical line
            if first.0 == second.0 {
                let x = first.0;
                let min = min(first.1, second.1);
                let max = max(first.1, second.1);
                for y in min..=max {
                    map.insert((x, y), true);
                    if y > max_y { max_y = y; }
                }
            }
            // horizontal line
            else {
                let y = first.1;
                if y > max_y { max_y = y; }
                let min = min(first.0, second.0);
                let max = max(first.0, second.0);
                for x in min..=max {
                    map.insert((x, y), true);
                }
            }
        }
    }

    (map, max_y)
}

fn solve1(input: &str) -> usize {
    print_map(input);
    let (mut map, max_y) = parse_map(input);

    let mut current_sand = (500usize, 0usize);
    let mut resting_sand_count = 0usize;
    loop {
        if current_sand.1 + 1 > max_y { break; }

        if !map.get(&(current_sand.0, current_sand.1 + 1)).unwrap_or(&false) {
            current_sand.1 += 1;
        } else if !map.get(&(current_sand.0 - 1, current_sand.1 + 1)).unwrap_or(&false) {
            current_sand.0 -= 1;
            current_sand.1 += 1;
        } else if !map.get(&(current_sand.0 + 1, current_sand.1 + 1)).unwrap_or(&false) {
            current_sand.0 += 1;
            current_sand.1 += 1;
        } else {
            resting_sand_count += 1;
            map.insert(current_sand, true);
            current_sand = (500, 0);
        }
    }

    resting_sand_count
}

fn solve2(input: &str) -> usize {
    let (mut map, max_y) = parse_map(input);

    let start = std::time::Instant::now();

    let mut current_sand = (500usize, 0usize);
    let mut resting_sand_count = 0usize;
    let mut iterations = 0usize;
    loop {
        iterations += 1;
        if current_sand.1 + 1 == max_y + 2 {
            resting_sand_count += 1;
            map.insert(current_sand, true);
            current_sand = (500, 0);
            continue;
        }

        if !map.get(&(current_sand.0, current_sand.1 + 1)).unwrap_or(&false) {
            current_sand.1 += 1;
        } else if !map.get(&(current_sand.0 - 1, current_sand.1 + 1)).unwrap_or(&false) {
            current_sand.0 -= 1;
            current_sand.1 += 1;
        } else if !map.get(&(current_sand.0 + 1, current_sand.1 + 1)).unwrap_or(&false) {
            current_sand.0 += 1;
            current_sand.1 += 1;
        } else {
            resting_sand_count += 1;
            if current_sand == (500, 0) { break; }
            map.insert(current_sand, true);
            current_sand = (500, 0);
        }
    }

    let elapsed = start.elapsed();
    println!("Finished in {elapsed:?} with {iterations} iterations.");

    resting_sand_count
}

/// Parses the input and displays the map (for fun :))
fn print_map(input: &str) {
    let mut map: HashMap<(usize, usize), bool> = HashMap::new();

    let mut min_x = usize::MAX;
    let mut min_y = usize::MAX;
    let mut max_x = 0usize;
    let mut max_y = 0usize;

    for line in input.lines() {
        let parts = line.split(" -> ")
            .map(|part| {
                let mut components = part.split(',');
                (components.next()
                     .and_then(|n| n.parse::<usize>().ok())
                     .unwrap(),
                 components.next()
                     .and_then(|n| n.parse::<usize>().ok())
                     .unwrap())
            }).collect::<Vec<_>>();

        for coords in parts.windows(2) {
            let [first, second] = coords else { unreachable!(); };
            // vertical line
            if first.0 == second.0 {
                let x = first.0;
                if x > max_x { max_x = x } else if x < min_x { min_x = x; }

                let min = min(first.1, second.1);
                let max = max(first.1, second.1);
                for y in min..=max {
                    map.insert((x, y), true);
                    if y > max_y { max_y = y; } else if y < min_y { min_y = y; }
                }
            }
            // horizontal line
            else {
                let y = first.1;
                if y > max_y { max_y = y; } else if y < min_y { min_y = y; }

                let min = min(first.0, second.0);
                let max = max(first.0, second.0);
                for x in min..=max {
                    map.insert((x, y), true);
                    if x > max_x { max_x = x } else if x < min_x { min_x = x; }
                }
            }
        }
    }

    println!("x range: {min_x}..{max_x}");
    for y in min_y..=max_y {
        print!("{y}. ");
        for x in min_x..=max_x {
            if *map.get(&(x, y)).unwrap_or(&false) {
                print!("#");
            }
            else {
                print!(".");
            }
        }
        println!();
    }
}
