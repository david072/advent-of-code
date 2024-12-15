use regex::Regex;

const INPUT: &str = include_str!("../day14.txt");

struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
}

fn main() {
    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    let mut robots = re
        .captures_iter(INPUT)
        .map(|c| c.extract())
        .map(|(_, [px, py, vx, vy])| {
            let num = |s: &str| s.parse::<i32>().unwrap();

            Robot {
                position: (num(px), num(py)),
                velocity: (num(vx), num(vy)),
            }
        })
        .collect::<Vec<_>>();

    const WIDTH: i32 = 101;
    const HEIGHT: i32 = 103;

    for _ in 0..100 {
        for robot in &mut robots {
            robot.position.0 += robot.velocity.0;
            robot.position.1 += robot.velocity.1;
            if robot.position.0 < 0 {
                robot.position.0 = WIDTH + robot.position.0;
            } else if robot.position.0 >= WIDTH {
                robot.position.0 -= WIDTH;
            }
            if robot.position.1 < 0 {
                robot.position.1 = HEIGHT + robot.position.1;
            } else if robot.position.1 >= HEIGHT {
                robot.position.1 -= HEIGHT;
            }
        }
    }

    println!(
        "robots: {:?}",
        robots.iter().map(|r| r.position).collect::<Vec<_>>()
    );

    let quadrant_width = WIDTH / 2;
    let quadrant_height = HEIGHT / 2;

    let mut quadrants: [i32; 4] = [0i32; 4];

    for robot in &robots {
        let mut add_to_quadrant_if_possible = |q: usize, x: i32, y: i32| {
            if robot.position.0 >= x
                && robot.position.0 < x + quadrant_width
                && robot.position.1 >= y
                && robot.position.1 < y + quadrant_height
            {
                quadrants[q] += 1;
            }
        };

        add_to_quadrant_if_possible(0, 0, 0);
        add_to_quadrant_if_possible(1, WIDTH - quadrant_width, 0);
        add_to_quadrant_if_possible(2, 0, HEIGHT - quadrant_height);
        add_to_quadrant_if_possible(3, WIDTH - quadrant_width, HEIGHT - quadrant_height);
    }

    let safety_factor = quadrants.iter().product::<i32>();
    println!("safety factor: {safety_factor}");
}
