use std::collections::HashMap;

const INPUT: &str = include_str!("../day20.txt");
const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn part1(map: &HashMap<(i32, i32), i32>) {
    let mut shortcuts = 0usize;
    for ((x, y), distance_to_end) in map.iter() {
        for (dx, dy) in DIRECTIONS {
            if let Some(dist2) = map.get(&(x + 2 * dx, y + 2 * dy)) {
                if distance_to_end - dist2 - 2 >= 100 {
                    shortcuts += 1;
                }
            }
        }
    }

    println!("shortcuts that save more than 100ps: {shortcuts}");
}

fn part2(map: &HashMap<(i32, i32), i32>) {
    let mut shortcuts = 0usize;
    let mut values = map.into_iter().collect::<Vec<_>>();
    values.sort_by_key(|(_, d)| *d);
    values.reverse();
    for (i, ((x, y), distance_to_end)) in values.iter().enumerate() {
        for ((x2, y2), distance_to_end2) in values[i + 1..].iter() {
            let dist = (x2 - x).abs() + (y2 - y).abs();
            if dist <= 20 {
                let time_saved = **distance_to_end - **distance_to_end2 - dist;
                if time_saved >= 100 {
                    shortcuts += 1;
                }
            }
        }
    }

    println!("shortcuts that save more than 100ps: {shortcuts}");
}

fn main() {
    let mut map = HashMap::<(i32, i32), i32>::new();
    let mut start = (0i32, 0i32);
    let mut end = (0i32, 0i32);

    for (y, line) in INPUT.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let p = (x as i32, y as i32);
            match c {
                'S' => start = p,
                'E' => end = p,
                '#' => continue,
                _ => {}
            }

            map.insert(p, 0);
        }
    }

    let mut pos = end;
    let mut last_pos = start;
    let mut distance_to_end = 0i32;
    while pos != start {
        *map.get_mut(&pos).unwrap() = distance_to_end;
        distance_to_end += 1;

        // There are only ever two road tiles next to each road tile (except for the end tile,
        // which has only one). If we therefore filter out the tile we were on before, we'll be
        // left with the only tile that we haven't seen yet (for the end tile, we just use a
        // completely different tile for the last position, since there is only one road tile
        // adjacent to the end tile).
        for (dx, dy) in DIRECTIONS {
            let p = (pos.0 + dx, pos.1 + dy);
            if p != last_pos && map.contains_key(&p) {
                // found the next tile
                last_pos = pos;
                pos = p;
                break;
            }
        }
    }

    *map.get_mut(&start).unwrap() = distance_to_end;

    part1(&map);
    part2(&map);
}
