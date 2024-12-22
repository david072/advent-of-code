use std::collections::HashMap;

const INPUT: &str = include_str!("../day22.txt");

fn mix_and_prune_with(secret: u64, rhs: u64) -> u64 {
    (secret ^ rhs) % 16777216
}

fn next_secret_number(mut secret: u64) -> u64 {
    // Step 1: Calculate the result of multiplying the secret number by 64. Then, mix this result into the secret number. Finally, prune the secret number.
    secret = mix_and_prune_with(secret, secret * 64);
    // Step 2: Calculate the result of dividing the secret number by 32. Round the result down to the nearest integer. Then, mix this result into the secret number. Finally, prune the secret number.
    secret = mix_and_prune_with(secret, secret / 32);
    // Step 3: Calculate the result of multiplying the secret number by 2048. Then, mix this result into the secret number. Finally, prune the secret number.
    mix_and_prune_with(secret, secret * 2048)
}

fn part1() {
    let result = INPUT
        .lines()
        .map(|l| l.parse::<u64>().unwrap())
        .map(|mut secret| {
            for _ in 0..2000 {
                secret = next_secret_number(secret);
            }
            secret
        })
        .sum::<u64>();
    println!("result: {result}");
}

/// Takes 10s in debug mode. Compiling in release mode takes runtime down to 800ms, so I'll take
/// that.
fn part2() {
    let returns = INPUT
        .lines()
        .map(|l| l.parse::<u64>().unwrap())
        .map(|mut secret| {
            let mut banana_returns = HashMap::<[i64; 4], i64>::new();
            let mut changes = [0i64; 4];
            for i in 0..2000 {
                let prev_secret_ones = (secret % 10) as i64;
                secret = next_secret_number(secret);
                let ones = (secret % 10) as i64;

                if i < 4 {
                    changes[i] = ones - prev_secret_ones;
                } else {
                    for i in 1..4 {
                        changes[i - 1] = changes[i];
                    }
                    changes[3] = ones - prev_secret_ones;
                }

                if i >= 3 {
                    if !banana_returns.contains_key(&changes) {
                        banana_returns.insert(changes.clone(), ones);
                    }
                }
            }
            banana_returns
        })
        .reduce(|mut acc, e| {
            for key in e.keys() {
                acc.entry(*key)
                    .and_modify(|d| *d += e[key])
                    .or_insert(e[key]);
            }
            acc
        })
        .unwrap();
    let (max_bananas_key, max_bananas) = returns.iter().max_by_key(|(_, d)| *d).unwrap();
    println!("max bananas: {max_bananas}, sequence: {max_bananas_key:?}");
}

fn main() {
    part1();
    part2();
}
