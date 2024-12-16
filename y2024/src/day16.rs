use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

const INPUT: &str = include_str!("../day16.txt");

#[derive(Clone)]
struct Tile {
    distance: isize,
    visited: bool,
    previous_tiles: Vec<(isize, isize, isize)>,
}

impl Tile {
    fn new() -> Tile {
        Self {
            distance: isize::MAX,
            visited: false,
            previous_tiles: vec![],
        }
    }
}

/// Populates Tile::previous_tiles and returns the final position of the path
/// => by following the values in previous_tiles, all shortest paths can be reconstructed
fn find_shortest_paths(
    map: &mut HashMap<(isize, isize, isize), Tile>,
    start_position: (isize, isize),
    end_position: (isize, isize),
) -> (isize, isize, isize) {
    map.get_mut(&(start_position.0, start_position.1, 0))
        .unwrap()
        .distance = 0;

    /// Inserts `p` into `vec`, keeping `vec`'s sorting by distance, by searching for the correct
    /// spot using binary search.
    ///
    /// This is about **10 TIMES faster** than pushing `p` to the end of the vector and re-sorting the
    /// vector!!!
    fn insert_sorted(
        map: &HashMap<(isize, isize, isize), Tile>,
        p: (isize, isize, isize),
        vec: &mut Vec<(isize, isize, isize)>,
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

    let mut queue: Vec<(isize, isize, isize)> = vec![(start_position.0, start_position.1, 0)];
    let mut last_position = (0, 0, 0);
    while !queue.is_empty() {
        let p = queue.remove(0);

        if p.0 == end_position.0 && p.1 == end_position.1 {
            last_position = p;
            break;
        }

        let d = map[&p].distance;
        map.get_mut(&p).unwrap().visited = true;

        let mut update_neighbor = |new_pos, new_dist: isize| {
            if !map.contains_key(&new_pos) {
                return;
            }

            if new_dist == map[&new_pos].distance {
                map.get_mut(&new_pos).unwrap().previous_tiles.push(p);
            }

            if map[&new_pos].visited {
                return;
            }

            let tile = map.get_mut(&new_pos).unwrap();
            if new_dist < tile.distance {
                tile.distance = new_dist;
                tile.previous_tiles.clear();
                tile.previous_tiles.push(p);
            }
            if !queue.contains(&new_pos) {
                insert_sorted(map, new_pos, &mut queue);
            }
        };

        // check movement
        {
            let (dx, dy) = match p.2 {
                0 => (1, 0),
                1 => (0, 1),
                2 => (-1, 0),
                3 => (0, -1),
                _ => unreachable!(),
            };
            let p = (p.0 + dx, p.1 + dy, p.2);
            update_neighbor(p, d + 1);
        }

        // check rotation
        update_neighbor((p.0, p.1, (p.2 + 1) % 4), d + 1000);
        update_neighbor((p.0, p.1, if p.2 - 1 < 0 { 3 } else { p.2 - 1 }), d + 1000);
    }

    last_position
}

fn part1(
    map: &HashMap<(isize, isize, isize), Tile>,
    start_position: (isize, isize),
    last_position: (isize, isize, isize),
) {
    let mut p = last_position;
    let mut points = 0isize;
    while p != (start_position.0, start_position.1, 0) {
        let next_pos = &map[&p].previous_tiles[0];
        if next_pos.2 != p.2 {
            points += 1000;
        } else {
            points += 1;
        }

        p = *next_pos;
    }

    println!("points: {points}");
}

fn part2(
    map: &HashMap<(isize, isize, isize), Tile>,
    start_position: (isize, isize),
    last_position: (isize, isize, isize),
) {
    fn count_tiles(
        map: &HashMap<(isize, isize, isize), Tile>,
        mut p: (isize, isize, isize),
        start_position: (isize, isize),
        counted_positions: &mut HashSet<(isize, isize)>,
    ) -> usize {
        let mut tiles = 0usize;
        while p != (start_position.0, start_position.1, 0) {
            if !counted_positions.contains(&(p.0, p.1)) {
                let prev_tiles = map[&p]
                    .previous_tiles
                    .iter()
                    .skip(1)
                    .map(|p| *p)
                    .collect::<Vec<_>>();
                for next_pos in prev_tiles {
                    tiles += count_tiles(map, next_pos, start_position, counted_positions);
                }

                tiles += 1;
            }
            counted_positions.insert((p.0, p.1));
            p = map[&p].previous_tiles[0];
        }
        tiles
    }

    let mut set = HashSet::new();
    let tiles = count_tiles(&map, last_position, start_position, &mut set);
    println!("tiles: {tiles}");
}

fn main() {
    let mut map = HashMap::<(isize, isize, isize), Tile>::new();
    let mut start_position = (0isize, 0isize);
    let mut end_position = (0isize, 0isize);
    for (y, l) in INPUT.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            let p = (x as isize, y as isize);
            match c {
                'S' => start_position = p,
                'E' => end_position = p,
                '.' => {}
                _ => continue,
            }
            for orientation in 0..4 {
                map.insert((p.0, p.1, orientation), Tile::new());
            }
        }
    }

    let last_position = find_shortest_paths(&mut map, start_position, end_position);

    part1(&map, start_position, last_position);
    part2(&map, start_position, last_position);
}
