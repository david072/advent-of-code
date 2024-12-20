use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

const INPUT: &str = include_str!("../day18.txt");

const WIDTH: isize = 70;
const HEIGHT: isize = 70;
const INITIALLY_FALLEN_BYTES: usize = 1024;

#[derive(Clone)]
struct Tile {
    distance: isize,
    visited: bool,
    previous_tile: Option<(isize, isize)>,
}

impl Tile {
    fn new() -> Tile {
        Self {
            distance: isize::MAX,
            visited: false,
            previous_tile: None,
        }
    }
}

/// Populates Tile::previous_tiles and returns the final position of the path
/// => by following the values in previous_tiles, all shortest paths can be reconstructed
fn find_shortest_path(
    map: &mut HashMap<(isize, isize), Tile>,
    start_position: (isize, isize),
    end_position: (isize, isize),
) -> (isize, isize) {
    map.get_mut(&(start_position.0, start_position.1))
        .unwrap()
        .distance = 0;

    /// Inserts `p` into `vec`, keeping `vec`'s sorting by distance, by searching for the correct
    /// spot using binary search.
    ///
    /// This is about **10 TIMES faster** than pushing `p` to the end of the vector and re-sorting the
    /// vector!!!
    fn insert_sorted(
        map: &HashMap<(isize, isize), Tile>,
        p: (isize, isize),
        vec: &mut Vec<(isize, isize)>,
    ) {
        let d = map[&p].distance;
        let mut left = 0usize;
        let mut right = vec.len();

        while left < right {
            let mid = (left + right) / 2;
            match map[&vec[mid]].distance.cmp(&d) {
                Ordering::Less => left = mid + 1,
                Ordering::Greater => right = mid,
                Ordering::Equal => {
                    vec.insert(mid, p);
                    return;
                }
            }
        }

        vec.insert(left, p);
    }

    let mut queue: Vec<(isize, isize)> = vec![(start_position.0, start_position.1)];
    let mut last_position = (0, 0);
    while !queue.is_empty() {
        let p = queue.remove(0);

        if p == end_position {
            last_position = p;
            break;
        }

        let d = map[&p].distance;
        map.get_mut(&p).unwrap().visited = true;

        let mut update_neighbor = |new_pos, new_dist: isize| {
            if !map.contains_key(&new_pos) || map[&new_pos].visited {
                return;
            }

            if new_dist < map[&new_pos].distance {
                map.get_mut(&new_pos).unwrap().distance = new_dist;
                map.get_mut(&new_pos).unwrap().previous_tile = Some(p);
            }

            if !queue.contains(&new_pos) {
                insert_sorted(map, new_pos, &mut queue);
            }
        };

        for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let p = (p.0 + dx, p.1 + dy);
            update_neighbor(p, d + 1);
        }
    }

    last_position
}

fn part1(mut map: HashMap<(isize, isize), Tile>) {
    let mut last_position = find_shortest_path(&mut map, (0, 0), (WIDTH, HEIGHT));
    let mut length = 0;
    while last_position != (0, 0) {
        length += 1;
        last_position = map[&last_position].previous_tile.unwrap();
    }

    println!("shortest path: {length}");
}

fn part2(mut map: HashMap<(isize, isize), Tile>, remaining_bytes: &[(isize, isize)]) {
    let mut remaining_bytes_idx = 0;

    // Collect all tiles along the path to the exit and simulate bytes falling until one of them
    // lands on the path. Then, recompute the path. Repeat this, until there is no path to the exit
    // anymore, at which point the last byte that fell obstructed all paths to the exit.
    loop {
        let path = {
            let mut map = map.clone();
            let mut last_position = find_shortest_path(&mut map, (0, 0), (WIDTH, HEIGHT));
            if last_position != (WIDTH, HEIGHT) {
                break;
            }

            let mut path = HashSet::<(isize, isize)>::new();
            while last_position != (0, 0) {
                path.insert(last_position);
                last_position = map[&last_position].previous_tile.unwrap();
            }
            path
        };

        while remaining_bytes_idx < remaining_bytes.len() {
            let b = &remaining_bytes[remaining_bytes_idx];
            remaining_bytes_idx += 1;
            map.remove(b);
            if path.contains(b) {
                break;
            }
        }
    }

    let (x, y) = remaining_bytes[remaining_bytes_idx - 1];
    println!("first byte that makes exit unreachable: {x},{y}");
}

fn main() {
    let bytes = INPUT
        .lines()
        .map(|l| {
            let mut nums = l.split(',').map(|n| n.parse::<isize>().unwrap());
            (nums.next().unwrap(), nums.next().unwrap())
        })
        .collect::<Vec<_>>();

    let mut map = HashMap::<(isize, isize), Tile>::from_iter(
        bytes
            .iter()
            .take(INITIALLY_FALLEN_BYTES)
            .map(|p| (*p, Tile::new())),
    );

    // "invert" HashMap => every position, except the given position is in the map
    for x in 0..=WIDTH {
        for y in 0..=HEIGHT {
            if !map.contains_key(&(x, y)) {
                map.insert((x, y), Tile::new());
            } else {
                map.remove(&(x, y));
            }
        }
    }

    part1(map.clone());
    part2(map.clone(), &bytes[INITIALLY_FALLEN_BYTES..]);
}
