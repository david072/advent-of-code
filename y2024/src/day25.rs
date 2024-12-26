const INPUT: &str = include_str!("../day25.txt");

fn fits(lock: &[usize], key: &[usize]) -> bool {
    lock.iter()
        .zip(key.iter())
        .map(|(l, k)| l + k)
        .all(|v| v <= 5)
}

fn main() {
    let mut locks = vec![];
    let mut keys = vec![];
    for part in INPUT.split("\n\n") {
        let schematic = part
            .trim()
            .lines()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        // locks have the top row filled, while keys have the top row empty
        let is_lock = part.trim().lines().next().unwrap().starts_with('#');
        let data = (0..schematic[0].len())
            .map(|col| {
                let mut count = 0usize;
                for i in 0..schematic.len() {
                    if schematic[i][col] == schematic[0][col] {
                        count += 1;
                    }
                }

                count -= 1;
                if is_lock {
                    count
                } else {
                    5 - count
                }
            })
            .collect::<Vec<_>>();
        if is_lock {
            locks.push(data);
        } else {
            keys.push(data)
        }
    }

    let mut count = 0usize;
    for lock in &locks {
        for key in &keys {
            if fits(lock, key) {
                count += 1;
            }
        }
    }

    println!("count: {count}");
}
