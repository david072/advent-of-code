use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../day10.txt");

fn collect_reachable_nines(
    map: &HashMap<(isize, isize), u32>,
    reachable_nines: &mut HashSet<(isize, isize)>,
    p: (isize, isize),
) {
    if map[&p] == 9 {
        reachable_nines.insert(p);
        return;
    }

    for v in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
        let p2 = (p.0 + v.0, p.1 + v.1);
        if !map.contains_key(&p2) {
            continue;
        }
        if map[&p2] != map[&p] + 1 {
            continue;
        }
        collect_reachable_nines(map, reachable_nines, p2);
    }
}

fn count_distinct_hiking_trails(map: &HashMap<(isize, isize), u32>, p: (isize, isize)) -> u32 {
    if map[&p] == 9 {
        return 1;
    }

    let mut score = 0;
    for v in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
        let p2 = (p.0 + v.0, p.1 + v.1);
        if !map.contains_key(&p2) {
            continue;
        }
        if map[&p2] != map[&p] + 1 {
            continue;
        }
        score += count_distinct_hiking_trails(map, p2);
    }

    score
}

fn part1(map: &HashMap<(isize, isize), u32>, starting_positions: &[(isize, isize)]) {
    let score_sum = starting_positions
        .iter()
        .map(|p| {
            let mut reachable_nines = HashSet::new();
            collect_reachable_nines(&map, &mut reachable_nines, *p);
            reachable_nines.len()
        })
        .sum::<usize>();
    println!("score sum: {score_sum}");
}

fn part2(map: &HashMap<(isize, isize), u32>, starting_positions: &[(isize, isize)]) {
    let score_sum = starting_positions
        .iter()
        .map(|p| count_distinct_hiking_trails(&map, *p))
        .sum::<u32>();
    println!("score sum: {score_sum}");
}

fn main() {
    let mut map = HashMap::<(isize, isize), u32>::new();
    let mut starting_positions = vec![];
    for (y, l) in INPUT.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            let height = c.to_digit(10).unwrap();
            let pos = (x as isize, y as isize);
            map.insert(pos, height);
            if height == 0 {
                starting_positions.push(pos);
            }
        }
    }

    part1(&map, &starting_positions);
    part2(&map, &starting_positions);
}
