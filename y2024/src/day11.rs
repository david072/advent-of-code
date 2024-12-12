use std::collections::HashMap;

const INPUT: &str = include_str!("../day11.txt");

fn digit_count(n: u64) -> u64 {
    ((n as f32).log10() + 1.).floor() as u64
}

fn get_digit(n: u64, d: u32) -> u64 {
    n / 10u64.pow(d) % 10
}

/// Simulates a stone for `iterations` iterations and returns the amount of stones it turns into
/// after said iterations.
/// `cache` uses the stone and the iterations remaining as the key and remembers the function's
/// result for that pair. This way, we avoid having to recalculate the stone count and can
/// short-circuit using the cached value.
fn stone_count_after_iterations(
    cache: &mut HashMap<(u64, usize), usize>,
    stone: u64,
    iterations: usize,
) -> usize {
    if iterations == 0 {
        return 1;
    }

    if let Some(count) = cache.get(&(stone, iterations)) {
        return *count;
    }

    let digits = digit_count(stone);

    let result = if stone == 0 {
        stone_count_after_iterations(cache, 1, iterations - 1)
    } else if digits % 2 == 0 {
        let mut right_half = 0u64;
        for j in 0u32..(digits / 2) as u32 {
            right_half += get_digit(stone, j) * 10u64.pow(j);
        }

        let left_half = (stone - right_half) / 10u64.pow((digits / 2) as u32);
        stone_count_after_iterations(cache, left_half, iterations - 1)
            + stone_count_after_iterations(cache, right_half, iterations - 1)
    } else {
        stone_count_after_iterations(cache, stone * 2024, iterations - 1)
    };

    cache.insert((stone, iterations), result);
    result
}

fn simulate_iterations(stones: &[u64], iterations: usize) {
    let mut cache = HashMap::new();
    let sum = stones
        .iter()
        .map(|s| stone_count_after_iterations(&mut cache, *s, iterations))
        .sum::<usize>();
    println!("sum: {sum}");
}

fn main() {
    let stones = INPUT
        .trim()
        .split(' ')
        .map(|p| p.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    // part 1
    simulate_iterations(&stones, 25);
    // part 2
    simulate_iterations(&stones, 75);
}
