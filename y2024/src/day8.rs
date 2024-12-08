use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../day8.txt");

fn part1(antennas: &HashMap<char, Vec<(isize, isize)>>, width: isize, height: isize) {
    let mut antinodes = HashSet::<(isize, isize)>::new();
    for (_, positions) in antennas.into_iter() {
        for i in 0..positions.len() {
            for j in (i + 1)..positions.len() {
                let mut insert = |p @ (x, y)| {
                    if x < 0 || x >= width || y < 0 || y >= height {
                        return;
                    }
                    antinodes.insert(p);
                };

                let (x1, y1) = positions[i];
                let (x2, y2) = positions[j];
                let dx = x2 - x1;
                let dy = y2 - y1;
                insert((x1 - dx, y1 - dy));
                insert((x2 + dx, y2 + dy));
            }
        }
    }

    println!("antinodes: {}", antinodes.len());
}

fn part2(antennas: &HashMap<char, Vec<(isize, isize)>>, width: isize, height: isize) {
    let is_inbounds = |(x, y): (isize, isize)| x >= 0 && x < width && y >= 0 && y < height;

    let mut antinodes = HashSet::<(isize, isize)>::new();
    for (_, positions) in antennas.into_iter() {
        for i in 0..positions.len() {
            for j in (i + 1)..positions.len() {
                let (x1, y1) = positions[i];
                let (x2, y2) = positions[j];
                let dx = x2 - x1;
                let dy = y2 - y1;

                let mut p = (x1, y1);
                while is_inbounds(p) {
                    antinodes.insert(p);
                    p = (p.0 + dx, p.1 + dy);
                }

                let mut p = (x1, y1);
                while is_inbounds(p) {
                    antinodes.insert(p);
                    p = (p.0 - dx, p.1 - dy);
                }
            }
        }
    }

    println!("antinodes: {}", antinodes.len());
}

fn main() {
    let mut antennas = HashMap::<char, Vec<(isize, isize)>>::new();
    for (y, l) in INPUT.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            if c == '.' {
                continue;
            }

            let p = (x as isize, y as isize);
            antennas
                .entry(c)
                .and_modify(|v| v.push(p))
                .or_insert(vec![p]);
        }
    }

    let width = INPUT.lines().next().unwrap().len() as isize;
    let height = INPUT.lines().count() as isize;

    part1(&antennas, width, height);
    part2(&antennas, width, height);
}
