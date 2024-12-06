use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../day6.txt");

#[derive(Clone)]
struct Tile {
    is_obstruction: bool,
    visited: bool,
    visited_directions: usize,
}

impl Tile {
    fn new(is_obstruction: bool) -> Self {
        Self {
            is_obstruction,
            visited: false,
            visited_directions: 0,
        }
    }
}

#[derive(Clone)]
struct Map {
    map: HashMap<(isize, isize), Tile>,
    guard: (isize, isize),
}

fn get_movement_vector(dir: usize) -> (isize, isize) {
    match dir {
        0 => (0, -1),
        1 => (1, 0),
        2 => (0, 1),
        3 => (-1, 0),
        _ => unreachable!(),
    }
}

fn part1(mut map: Map) {
    map.map.get_mut(&map.guard).unwrap().visited = true;
    let mut guard_dir = 0;
    let mut visited_tiles = 1;
    loop {
        let dir = get_movement_vector(guard_dir);
        let new_pos = (map.guard.0 + dir.0, map.guard.1 + dir.1);
        if let Some(tile) = map.map.get_mut(&new_pos) {
            if tile.is_obstruction {
                guard_dir = (guard_dir + 1) % 4;
            } else {
                map.guard = new_pos;
                if !tile.visited {
                    visited_tiles += 1;
                    tile.visited = true;
                }
            }
        } else {
            break;
        }
    }

    println!("visited tiles: {visited_tiles}");
}

// NOTE: This is very unoptimal (takes ~5s on my machine in debug mode).
//
// Optimization idea: Instead of storing the entire map with every tile, only store the
// obstructions in a vector. Then, create two more vectors, that map the x/y coordinate (the index)
// to a list of indices into the obstructions vector. This way, we can query the obstructions in a
// row/column without having to traverse the map to find it. If every obstruction stores the
// directions that we encountered it with, we can detect a loop by checking whether we got to the
// same obstruction with the same direction like below. This would save us having to iterate the
// entire map as we could basically jump around from obstacle to obstacle without iterating over
// the space in between.
fn part2(mut map: Map) {
    fn is_circular_path(mut map: Map, mut guard_dir: usize) -> bool {
        loop {
            let dir = get_movement_vector(guard_dir);
            let new_pos = (map.guard.0 + dir.0, map.guard.1 + dir.1);
            if let Some(tile) = map.map.get_mut(&new_pos) {
                if tile.is_obstruction {
                    guard_dir = (guard_dir + 1) % 4;
                } else {
                    // if we passed over the tile with the same direction before, we are in a loop
                    if tile.visited && tile.visited_directions & (1 << guard_dir) > 0 {
                        return true;
                    }

                    tile.visited = true;
                    tile.visited_directions |= 1 << guard_dir;
                    map.guard = new_pos;
                }
            } else {
                break;
            }
        }

        false
    }

    let mut obstructions = 0;
    let mut guard_dir = 0;
    loop {
        let dir = get_movement_vector(guard_dir);
        let new_pos = (map.guard.0 + dir.0, map.guard.1 + dir.1);
        if let Some(tile) = map.map.get_mut(&new_pos) {
            if tile.is_obstruction {
                guard_dir = (guard_dir + 1) % 4;
            } else {
                if tile.visited {
                    map.guard = new_pos;
                    continue;
                }
                tile.visited = true;

                let mut m = map.clone();
                m.map.get_mut(&new_pos).unwrap().is_obstruction = true;
                if is_circular_path(m, guard_dir) {
                    obstructions += 1;
                }

                map.guard = new_pos;
            }
        } else {
            break;
        }
    }

    println!("obstructions: {obstructions}");
}

fn main() {
    let mut map = HashMap::<(isize, isize), Tile>::new();

    let mut guard = (0isize, 0isize);
    for (y, line) in INPUT.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let p = (x as isize, y as isize);
            if c == '^' {
                guard = p;
            }
            map.insert(p, Tile::new(c == '#'));
        }
    }

    let map = Map { map, guard };
    part1(map.clone());
    part2(map);
}
