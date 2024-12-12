use std::collections::HashMap;

const INPUT: &str = include_str!("../day12.txt");

#[derive(Clone, Copy)]
struct Tile {
    c: char,
    visited: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EdgeDirection {
    Left,
    Right,
    Top,
    Bottom,
}

fn flood_fill(
    map: &mut HashMap<(isize, isize), Tile>,
    p: (isize, isize),
    edges: &mut HashMap<(isize, isize), Vec<EdgeDirection>>,
) -> (usize, usize) {
    let mut area = 1usize;
    let mut perimeter = 0usize;

    let current_tile = map[&p].c;
    map.get_mut(&p).unwrap().visited = true;
    for (v, edge_dir) in [
        ((0, 1), EdgeDirection::Bottom),
        ((1, 0), EdgeDirection::Right),
        ((0, -1), EdgeDirection::Top),
        ((-1, 0), EdgeDirection::Left),
    ] {
        let new_pos = (p.0 + v.0, p.1 + v.1);
        match map.get_mut(&new_pos) {
            Some(Tile { c, visited }) if *c == current_tile => {
                if !*visited {
                    let (a, peri) = flood_fill(map, new_pos, edges);
                    area += a;
                    perimeter += peri;
                }
            }
            _ => {
                perimeter += 1;
                edges
                    .entry(p)
                    .and_modify(|v| v.push(edge_dir))
                    .or_insert_with(|| vec![edge_dir]);
            }
        }
    }

    (area, perimeter)
}

fn part1(mut map: HashMap<(isize, isize), Tile>) {
    let mut result = 0usize;
    let coords = map.keys().map(|p| *p).collect::<Vec<_>>();
    let mut edges = HashMap::new();
    for p in coords {
        if map[&p].visited {
            continue;
        }
        let (area, perimeter) = flood_fill(&mut map, p, &mut edges);
        result += area * perimeter;
    }

    println!("price: {result}");
}

fn part2(mut map: HashMap<(isize, isize), Tile>) {
    fn remove_edges_in_direction(
        edges: &mut HashMap<(isize, isize), Vec<EdgeDirection>>,
        edge_dir: EdgeDirection,
        start: (isize, isize),
        perpendicular_dir: (isize, isize),
    ) {
        let mut p = start;
        while let Some(dirs) = edges.get_mut(&p) {
            if let Some(i) = dirs.iter().position(|edge| *edge == edge_dir) {
                dirs.remove(i);
                p = (p.0 + perpendicular_dir.0, p.1 + perpendicular_dir.1);
            } else {
                break;
            }
        }
    }

    let mut result = 0usize;
    let coords = map.keys().map(|p| *p).collect::<Vec<_>>();
    let mut edges = HashMap::new();
    for p in coords {
        if map[&p].visited {
            continue;
        }
        let (area, _) = flood_fill(&mut map, p, &mut edges);

        let mut sides = 0usize;
        let edge_positions = edges.keys().map(|p| *p).collect::<Vec<_>>();
        for p in edge_positions {
            if !edges.contains_key(&p) {
                continue;
            }

            for edge_dir in edges[&p].clone() {
                let perpendicular_dir = match edge_dir {
                    EdgeDirection::Top | EdgeDirection::Bottom => (1, 0),
                    EdgeDirection::Left | EdgeDirection::Right => (0, 1),
                };

                remove_edges_in_direction(&mut edges, edge_dir, p, perpendicular_dir);
                remove_edges_in_direction(
                    &mut edges,
                    edge_dir,
                    (p.0 - perpendicular_dir.0, p.1 - perpendicular_dir.1),
                    (-perpendicular_dir.0, -perpendicular_dir.1),
                );

                sides += 1;
            }
        }

        result += area * sides;
    }

    println!("price: {result}");
}

fn main() {
    let map =
        HashMap::<(isize, isize), Tile>::from_iter(INPUT.lines().enumerate().flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(move |(x, c)| ((x as isize, y as isize), Tile { c, visited: false }))
        }));

    part1(map.clone());
    part2(map);
}
