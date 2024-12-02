const INPUT: &str = include_str!("../day2.txt");

fn is_safe(elements: &[i32]) -> bool {
    let sign = (elements[0] - elements[1]).signum();
    for i in 1..elements.len() {
        let diff = elements[i - 1] - elements[i];
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }
        if diff.signum() != sign.signum() {
            return false;
        }
    }

    true
}

fn part1() {
    let safe_count = INPUT
        .lines()
        .filter(|l| {
            let elements = l
                .split(' ')
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            is_safe(&elements)
        })
        .count();
    println!("Safe reports: {safe_count}");
}

/// NOTE: This is horrible, since for a report of length n, it is O(n^2) runtime, but I can't think
/// of a clean way to do this more efficiently.
fn part2() {
    let safe_count = INPUT
        .lines()
        .filter(|l| {
            let elements = l
                .split(' ')
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            if is_safe(&elements) {
                return true;
            }

            for i in 0..elements.len() {
                let mut els = elements.clone();
                els.remove(i);
                if is_safe(&els) {
                    return true;
                }
            }

            false
        })
        .count();
    println!("Safe reports with problem dampener: {safe_count}");
}

fn main() {
    part1();
    part2();
}
