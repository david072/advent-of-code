use std::collections::HashMap;

const DIRECTIONS: [Pos; 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

type Pos = (isize, isize);

struct Tile {
    height: u8,
    visited: bool,
    dist: u16,
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("No input file found");
    let now = std::time::SystemTime::now();
    let s1 = solution1(&input);
    let time = now.elapsed().expect("Error timing");
    println!("Solution 1: {s1} (took {time:?})");
}

fn solution1(input: &str) -> usize {
    let mut map = HashMap::new();
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (y, line) in input.split('\n').enumerate() {
        for (x, char) in line.chars().enumerate() {
            let coord = (x as isize, y as isize);
            if char == 'S' {
                start = coord;
            } else if char == 'E' {
                end = coord;
            }

            map.insert(
                coord,
                Tile {
                    height: char_to_height(char),
                    visited: false,
                    dist: if char == 'S' { 0 } else { u16::MAX / 2 },
                },
            );
        }
    }

    assert!(start != end);

    let mut current_pos = start;

    loop {
        if current_pos == end {
            break;
        }

        let current_height = map[&current_pos].height;
        let current_dist = map[&current_pos].dist;

        map.get_mut(&current_pos).unwrap().visited = true;

        let neighbors = DIRECTIONS
            .iter()
            .map(|dir| (current_pos.0 + dir.0, current_pos.1 + dir.1))
            .filter(|pos| map.get(&pos).map(|t| !t.visited).unwrap_or(false))
            .collect::<Vec<_>>();
        for neighbor in &neighbors {
            if let Some(tile) = map.get_mut(&neighbor) {
                if tile.height > current_height + 1 {
                    continue;
                }

                tile.dist = (current_dist + 1).min(tile.dist);
            }
        }

        current_pos = *neighbors
            .iter()
            .min_by(|pos1, pos2| map[pos1].height.cmp(&map[pos2].height))
            .unwrap();

        println!("pos: {current_pos:?}");
    }

    map[&end].dist.into()
}

fn char_to_height(c: char) -> u8 {
    let mut c = c;
    if c == 'S' {
        c = 'a';
    }
    if c == 'E' {
        c = 'z';
    }

    c as u8 - 'a' as u8
}
