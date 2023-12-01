use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign};
use std::time::Instant;
use nohash_hasher::BuildNoHashHasher;

pub fn main() -> utils::Result<()> {
    let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>".to_string();
    // let input = utils::get_input(2022, 17)?;

    println!("Part One: {}", solve1(&input));
    println!("Part Two: {}", solve2(&input));

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coord(isize, isize);

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

macro_rules! formation {
    ($($x:expr,$y:expr);+) => {
        &[$(Coord($x, $y)),+]
    }
}

/// Stores the offsets of every child rock to the top-left and the height of the formation
const ROCK_FORMATIONS: [(&[Coord], usize); 5] = [
    // ####
    (formation!(0,0 ; 1,0 ; 2,0 ; 3,0), 1),
    // .#.
    // ###
    // .#.
    (formation!(1,0 ; 0,1 ; 1,1 ; 2,1 ; 1,2), 3),
    // ..#
    // ..#
    // ###
    (formation!(2,0 ; 2,1 ; 0,2 ; 1,2 ; 2,2), 3),
    // #
    // #
    // #
    // #
    (formation!(0,0 ; 0,1 ; 0,2 ; 0,3), 4),
    // ##
    // ##
    (formation!(0,0 ; 1,0 ; 0,1 ; 1,1), 2),
];

struct Rock {
    position: Coord,
    formation_index: usize,
}

impl Display for Rock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "({}, {}), formation: {}", self.position.0, self.position.1, self.formation_index)
    }
}

impl Rock {
    pub fn new(position: Coord, formation_index: usize) -> Self {
        Self { position, formation_index }
    }

    #[inline(never)]
    pub fn move_if_possible<CollisionFn: Fn(Coord) -> bool>(&mut self, offset: Coord, is_collision: CollisionFn) -> bool {
        let child_coords = ROCK_FORMATIONS[self.formation_index].0;

        let new_pos = self.position + offset;
        for child_offset in child_coords {
            let coord = new_pos + *child_offset;
            if is_collision(coord) { return false; }
        }

        self.position = new_pos;
        true
    }
}

fn parse_jet_streams(input: &str) -> Vec<isize> {
    input.trim()
        .chars()
        .map(|char| {
            match char {
                '>' => 1isize,
                '<' => -1isize,
                _ => unreachable!(),
            }
        })
        .collect::<Vec<_>>()
}

#[inline(never)]
fn solve1(input: &str) -> isize {
    println!("input: {input:?}");

    let jet_streams = parse_jet_streams(input);

    let mut jet_stream_index = 0usize;
    // left wall is at x=0
    // floor is at y=0
    let mut rocks: HashMap<isize, Vec<isize>, BuildNoHashHasher<isize>> = HashMap::with_hasher(BuildNoHashHasher::default());
    let mut current_rock = Rock::new(Coord(3, -4), 0);
    // have to keep track of the lowest y coordinate, since higher up means y is smaller
    let mut lowest_y = 0isize;
    let mut resting_rocks = 0usize;

    let start = Instant::now();

    loop {
        let jet_stream_offset = jet_streams[jet_stream_index];
        jet_stream_index = (jet_stream_index + 1) % jet_streams.len();

        let is_collision = |Coord(x, y)| {
            x < 1 || x > 7 || y > -1 || rocks.get(&y).map(|x_values| x_values.contains(&x)).unwrap_or_default()
            // !(1..=7).contains(&x) || y > -1 || rocks.get(&y).map(|x_values| x_values.contains(&x)).unwrap_or_default()
        };
        current_rock.move_if_possible(Coord(jet_stream_offset, 0), is_collision);

        let did_move_down = current_rock.move_if_possible(Coord(0, 1), is_collision);
        if !did_move_down {
            resting_rocks += 1;
            // println!("{}/2022", resting_rocks);
            if current_rock.position.1 < lowest_y {
                lowest_y = current_rock.position.1;
            }
            if resting_rocks == 2022 { break; }

            let formation_index = (current_rock.formation_index + 1) % ROCK_FORMATIONS.len();

            for coord in ROCK_FORMATIONS[current_rock.formation_index].0 {
                let coord = current_rock.position + *coord;
                rocks.entry(coord.1)
                    .and_modify(|x_values| x_values.push(coord.0))
                    .or_insert_with(|| vec![coord.0]);
            }

            current_rock = Rock::new(
                Coord(3, lowest_y - 3 - ROCK_FORMATIONS[formation_index].1 as isize),
                formation_index,
            );
        }
    }

    println!("Finished in {:?}", start.elapsed());

    -lowest_y
}

fn solve2(input: &str) -> u64 {
    0
}
