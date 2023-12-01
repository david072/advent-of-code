use std::ops::RangeInclusive;

pub fn main() -> utils::Result<()> {
    let input = utils::get_input(2022, 4)?;

    println!("Part One: {}", solve1(&input));
    println!("Part Two: {}", solve2(&input));

    Ok(())
}

fn solve1(input: &str) -> usize {
    input.lines()
        .map(|l| l.split_once(',').unwrap())
        .map(|(r1, r2)| (make_range(r1), make_range(r2)))
        .filter(|(r1, r2)| {
            (r1.start() >= r2.start() && r1.end() <= r2.end()) ||
                (r2.start() >= r1.start() && r2.end() <= r1.end())
        })
        .count()
}

fn solve2(input: &str) -> usize {
    input.lines()
        .map(|l| l.split_once(',').unwrap())
        .map(|(r1, r2)| (make_range(r1), make_range(r2)))
        .filter(|(r1, r2)| {
            r1.contains(r2.start()) || r1.contains(r2.end()) ||
               r2.contains(r1.start()) || r2.contains(r1.end())
        })
        .count()
}

fn make_range(str: &str) -> RangeInclusive<u64> {
    let (start, end) = str.split_once('-').unwrap();
    start.parse::<u64>().unwrap()..=end.parse::<u64>().unwrap()
}
