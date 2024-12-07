const INPUT: &str = include_str!("../day7.txt");

fn is_result_possible(
    result: u64,
    mut current_result: u64,
    numbers: &[u64],
    allow_concat: bool,
) -> bool {
    if numbers.len() == 0 {
        return result == current_result;
    }

    let n = numbers[0];
    let mut possible = false;
    possible |= is_result_possible(result, current_result * n, &numbers[1..], allow_concat);
    if !possible {
        possible |= is_result_possible(result, current_result + n, &numbers[1..], allow_concat);
    }

    if !possible && allow_concat {
        let len = (numbers[0] as f32).log10().floor() as u32 + 1;
        current_result = n + current_result * 10u64.pow(len);
        possible |= is_result_possible(result, current_result, &numbers[1..], allow_concat);
    }

    possible
}

fn part1<It>(equations: It)
where
    It: Iterator<Item = (u64, Vec<u64>)>,
{
    let possible_equations: u64 = equations
        .filter(|(result, numbers)| is_result_possible(*result, 0, numbers, false))
        .map(|(result, _)| result)
        .sum();
    println!("possible equations sum: {possible_equations}");
}

fn part2<It>(equations: It)
where
    It: Iterator<Item = (u64, Vec<u64>)>,
{
    let possible_equations: u64 = equations
        .filter(|(result, numbers)| is_result_possible(*result, 0, numbers, true))
        .map(|(result, _)| result)
        .sum();
    println!("possible equations sum: {possible_equations}");
}

fn main() {
    let equations = INPUT.lines().map(|l| {
        let (result, numbers) = l.split_once(": ").unwrap();
        let numbers = numbers
            .split(' ')
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        let result = result.parse::<u64>().unwrap();
        (result, numbers)
    });

    part1(equations.clone());
    part2(equations);
}
