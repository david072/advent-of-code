use std::ops::{Add, AddAssign};

pub fn main() -> utils::Result<()> {
    let input = utils::get_input(2022, 9)?;

    println!("Part One: {}", solve1(&input));
    println!("Part Two: {}", solve2(&input));

    Ok(())
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Coord(isize, isize);

impl Coord {
    pub fn is_adjacent(&self, other: Coord, allow_diagonal: bool) -> bool {
        for dx in -1isize..=1isize {
            for dy in -1isize..=1isize {
                if !allow_diagonal && (dx == dy || dx == -dy || dy == -dx) { continue; }
                if *self + Coord(dx, dy) == other { return true; }
            }
        }

        false
    }
}

impl Add<Coord> for Coord {
    type Output = Coord;

    fn add(self, rhs: Coord) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign<Coord> for Coord {
    fn add_assign(&mut self, rhs: Coord) {
        *self = self.add(rhs);
    }
}

fn solve1(input: &str) -> usize {
    let mut head: Coord = Coord(0, 0);
    let mut tail: Coord = Coord(0, 0);

    let mut visited_coords: Vec<Coord> = Vec::new();
    visited_coords.push(tail);

    for line in input.lines() {
        let mut parts = line.split(' ');
        let direction = parts.next().unwrap();
        let steps = parts.next().unwrap().parse::<isize>().unwrap();

        for _ in 0..steps {
            match direction {
                "R" => head += Coord(1, 0),
                "L" => head += Coord(-1, 0),
                "U" => head += Coord(0, -1),
                "D" => head += Coord(0, 1),
                _ => unreachable!(),
            }

            if !tail.is_adjacent(head, true) {
                'outer: for dx in -1isize..=1isize {
                    for dy in -1isize..=1isize {
                        if (tail + Coord(dx, dy)).is_adjacent(head, false) {
                            tail += Coord(dx, dy);
                            break 'outer;
                        }
                    }
                }
            }

            if !visited_coords.contains(&tail) {
                visited_coords.push(tail);
            }
        }
    }

    visited_coords.len()
}

fn solve2(input: &str) -> usize {
    let mut knots: [Coord; 10] = [Coord(0, 0); 10];

    let mut visited_coords: Vec<Coord> = Vec::new();
    visited_coords.push(*knots.last().unwrap());

    for line in input.lines() {
        let mut parts = line.split(' ');
        let direction = parts.next().unwrap();
        let steps = parts.next().unwrap().parse::<isize>().unwrap();

        for _ in 0..steps {
            {
                let head = &mut knots[0];
                match direction {
                    "R" => *head += Coord(1, 0),
                    "L" => *head += Coord(-1, 0),
                    "U" => *head += Coord(0, -1),
                    "D" => *head += Coord(0, 1),
                    _ => unreachable!(),
                }
            }

            for tail_idx in 1..knots.len() {
                let head = knots[tail_idx - 1];
                let tail = &mut knots[tail_idx];

                if !tail.is_adjacent(head, true) {
                    let mut diagonal_move: Option<Coord> = None;
                    let mut found_move = false;

                    'outer: for dx in -1isize..=1isize {
                        for dy in -1isize..=1isize {
                            let new_tail = *tail + Coord(dx, dy);
                            if new_tail.is_adjacent(head, false) {
                                *tail = new_tail;
                                found_move = true;
                                break 'outer;
                            }
                            else if new_tail.is_adjacent(head, true) && diagonal_move.is_none() {
                                diagonal_move = Some(Coord(dx, dy));
                            }
                        }
                    }

                    if !found_move {
                        *tail += diagonal_move.unwrap();
                    }
                }
            }

            let last_knot = knots.last().unwrap();
            if !visited_coords.contains(last_knot) {
                visited_coords.push(*last_knot);
            }
        }
    }

    visited_coords.len()
}
