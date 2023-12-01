pub fn main() -> utils::Result<()> {
    let input = utils::get_input(2022, 3)?;

    println!("Part One: {}", solve1(&input));
    println!("Part Two: {}", solve2(&input));

    Ok(())
}

fn solve1(input: &str) -> u64 {
    input.lines()
        .map(|l| l.split_at(l.len() / 2))
        .map(|(first, second)| {
            first.chars()
                .find(|c| second.contains(*c))
                .map(|char| {
                    if char.is_lowercase() {
                        char as u64 - 96
                    } else {
                        (char as u64 - 64) + 26
                    }
                }).unwrap()
        })
        .sum()
}

fn solve2(input: &str) -> u64 {
    input.lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|lines| {
            let [a, b, c] = lines else { unreachable!(); };
            a.chars()
                .find(|char| b.contains(*char) && c.contains(*char))
                .map(|char| {
                    if char.is_lowercase() {
                        char as u64 - 96
                    } else {
                        (char as u64 - 64) + 26
                    }
                })
                .unwrap()
        })
        .sum()
}
