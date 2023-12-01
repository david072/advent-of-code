fn main() -> utils::Result<()> {
    let input = utils::get_input(2022, 1)?;
    println!("Part One: {}", solve1(&input));
    println!("Part Two: {}", solve2(&input));
    Ok(())
}

fn solve1(input: &str) -> u64 {
    input.split("\n\n")
        .map(|str| str.lines()
            .map(|l| l.parse::<u64>().unwrap())
            .sum()
        )
        .max().unwrap()
}

fn solve2(input: &str) -> u64 {
    input.split("\n\n")
        .map(|str| str.lines()
            .map(|l| l.parse::<u64>().unwrap())
            .sum()
        )
        .fold([0, 0, 0], |mut acc, n| {
            if let Some(v) = acc.iter_mut().find(|num| n > **num) {
                *v = n;
            }
            acc
        })
        .iter().sum()
}