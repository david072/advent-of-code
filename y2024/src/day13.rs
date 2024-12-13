use regex::Regex;

const INPUT: &str = include_str!("../day13.txt");

struct Machine {
    btn_a: (i64, i64),
    btn_b: (i64, i64),
    prize: (i64, i64),
}

impl Machine {
    pub fn tokens_needed(&self) -> Option<i64> {
        // Let (Ax, Ay) be the delta that button A moves the claw and `a` the amount of times it is
        // pressed.
        let (ax, ay) = self.btn_a;
        // Let (Bx, By) be the delta that button B moves the claw and `b` the amount of times it is
        // pressed.
        let (bx, by) = self.btn_b;
        // Let (Px, Py) be the position of the prize.
        let (px, py) = self.prize;

        // Therefore, we can describe the problem using the following equation system:
        // (1) Ax * a + Bx * b = Px
        // (2) Ay * a + By * b = Py

        // Solving this system for a and b we get the following equations:
        // a = (Py * Bx - By * Px) / (Ay * Bx - By * Ax)
        // b = (Px - Ax * a) / Bx
        // Both equations have to yield natural numbers for it to be possible to reach the prize.

        let a = {
            let numerator = py * bx - by * px;
            let denominator = ay * bx - by * ax;
            if numerator % denominator != 0 {
                return None;
            }

            numerator / denominator
        };

        let b = {
            let numerator = px - ax * a;
            if numerator % bx != 0 {
                return None;
            }
            numerator / bx
        };

        Some(a * 3 + b)
    }
}

fn part1(machines: impl Iterator<Item = Machine>) {
    let tokens = machines.filter_map(|m| m.tokens_needed()).sum::<i64>();
    println!("tokens: {tokens}");
}

fn part2(machines: impl Iterator<Item = Machine>) {
    let tokens = machines
        .map(|m| Machine {
            prize: (m.prize.0 + 10000000000000, m.prize.1 + 10000000000000),
            ..m
        })
        .filter_map(|m| m.tokens_needed())
        .sum::<i64>();
    println!("tokens: {tokens}");
}

fn main() {
    let re = Regex::new(r".*X.(\d+), Y.(\d+)").unwrap();
    let machines = INPUT.split("\n\n").map(|s| {
        let mut coords = re
            .captures_iter(s)
            .map(|c| c.extract())
            .map(|(_, [n1, n2])| (n1.parse::<i64>().unwrap(), n2.parse::<i64>().unwrap()));

        Machine {
            btn_a: coords.next().unwrap(),
            btn_b: coords.next().unwrap(),
            prize: coords.next().unwrap(),
        }
    });

    part1(machines.clone());
    part2(machines);
}
