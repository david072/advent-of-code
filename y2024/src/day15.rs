use std::collections::HashMap;

const INPUT: &str = include_str!("../day15.txt");

fn part1(map_str: &str, movement_sequence: &[(i32, i32)]) {
    #[derive(Clone, Copy, PartialEq, Eq)]
    enum Tile {
        Empty,
        Wall,
        Box,
    }

    fn try_move(
        map: &mut HashMap<(i32, i32), Tile>,
        position: (i32, i32),
        direction: (i32, i32),
    ) -> bool {
        match map[&position] {
            Tile::Empty => true,
            Tile::Wall => false,
            Tile::Box => {
                let new_pos = (position.0 + direction.0, position.1 + direction.1);
                if !try_move(map, new_pos, direction) {
                    false
                } else {
                    *map.get_mut(&new_pos).unwrap() = Tile::Box;
                    *map.get_mut(&position).unwrap() = Tile::Empty;
                    true
                }
            }
        }
    }

    let mut robot = (0i32, 0i32);
    let mut map = HashMap::<(i32, i32), Tile>::new();
    for (y, l) in map_str.trim().lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            let pos = (x as i32, y as i32);

            if c == '@' {
                robot = pos;
            }

            map.insert(
                pos,
                match c {
                    '.' | '@' => Tile::Empty,
                    '#' => Tile::Wall,
                    'O' => Tile::Box,
                    _ => unreachable!(),
                },
            );
        }
    }

    for dir in movement_sequence {
        let new_robot_pos = (robot.0 + dir.0, robot.1 + dir.1);
        if try_move(&mut map, new_robot_pos, *dir) {
            robot = new_robot_pos;
        }
    }

    let gps_coords_sum = map
        .iter()
        .filter(|(_, tile)| **tile == Tile::Box)
        .map(|((x, y), _)| y * 100 + x)
        .sum::<i32>();
    println!("gps coords sum: {gps_coords_sum}");
}

fn part2(map_str: &str, movement_sequence: &[(i32, i32)]) {
    #[derive(Clone, Copy, PartialEq, Eq)]
    enum Tile {
        Empty,
        Wall,
        LeftBoxHalf,
        RightBoxHalf,
    }

    fn try_move(
        map: &mut HashMap<(i32, i32), Tile>,
        position: (i32, i32),
        direction: (i32, i32),
        perform_move: bool,
    ) -> bool {
        fn try_move_box(
            map: &mut HashMap<(i32, i32), Tile>,
            left_box_pos: (i32, i32),
            right_box_pos: (i32, i32),
            direction: (i32, i32),
            perform_move: bool,
        ) -> bool {
            let new_left_box_pos = (left_box_pos.0 + direction.0, left_box_pos.1 + direction.1);
            let new_right_box_pos = (right_box_pos.0 + direction.0, right_box_pos.1 + direction.1);

            let can_move = match direction {
                (0, _) => {
                    try_move(map, new_left_box_pos, direction, perform_move)
                        && try_move(map, new_right_box_pos, direction, perform_move)
                }
                (-1, 0) => try_move(map, new_left_box_pos, direction, perform_move),
                (1, 0) => try_move(map, new_right_box_pos, direction, perform_move),
                _ => unreachable!(),
            };

            if can_move {
                if perform_move {
                    *map.get_mut(&left_box_pos).unwrap() = Tile::Empty;
                    *map.get_mut(&right_box_pos).unwrap() = Tile::Empty;
                    *map.get_mut(&new_left_box_pos).unwrap() = Tile::LeftBoxHalf;
                    *map.get_mut(&new_right_box_pos).unwrap() = Tile::RightBoxHalf;
                }
                true
            } else {
                false
            }
        }

        match map[&position] {
            Tile::Empty => true,
            Tile::Wall => false,
            Tile::LeftBoxHalf => try_move_box(
                map,
                position,
                (position.0 + 1, position.1),
                direction,
                perform_move,
            ),
            Tile::RightBoxHalf => try_move_box(
                map,
                (position.0 - 1, position.1),
                position,
                direction,
                perform_move,
            ),
        }
    }

    let mut robot = (0i32, 0i32);
    let mut map = HashMap::<(i32, i32), Tile>::new();
    for (y, l) in map_str.trim().lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            let pos = (x as i32 * 2, y as i32);

            if c == '@' {
                robot = pos;
            }

            let (left_tile, right_tile) = match c {
                '.' | '@' => (Tile::Empty, Tile::Empty),
                '#' => (Tile::Wall, Tile::Wall),
                'O' => (Tile::LeftBoxHalf, Tile::RightBoxHalf),
                _ => unreachable!(),
            };

            map.insert(pos, left_tile);
            map.insert((pos.0 + 1, pos.1), right_tile);
        }
    }

    for dir in movement_sequence {
        let new_robot_pos = (robot.0 + dir.0, robot.1 + dir.1);
        if try_move(&mut map, new_robot_pos, *dir, false) {
            try_move(&mut map, new_robot_pos, *dir, true);
            robot = new_robot_pos;
        }
    }

    let gps_coords_sum = map
        .iter()
        .filter(|(_, tile)| **tile == Tile::LeftBoxHalf)
        .map(|((x, y), _)| y * 100 + x)
        .sum::<i32>();
    println!("gps coords sum: {gps_coords_sum}");
}

fn main() {
    let (map_str, movement_sequence) = INPUT.split_once("\n\n").unwrap();

    let movement_sequence = movement_sequence
        .trim()
        .chars()
        .filter(|c| *c != '\n')
        .map(|c| match c {
            '^' => (0, -1),
            '<' => (-1, 0),
            '>' => (1, 0),
            'v' => (0, 1),
            _ => unreachable!(),
        })
        .collect::<Vec<(i32, i32)>>();

    part1(map_str, &movement_sequence);
    part2(map_str, &movement_sequence);
}
