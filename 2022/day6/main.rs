pub fn main() -> utils::Result<()> {
    let input = utils::get_input(2022, 6)?;

    println!("Part One: {}", solve1(&input));
    println!("Part Two: {}", solve2(&input));

    Ok(())
}

fn solve1(input: &str) -> usize {
    input.chars().collect::<Vec<_>>()
        .windows(4)
        .position(|chars| {
            for char in chars {
                if chars.iter().filter(|c| *c == char).count() > 1 {
                    return false;
                }
            }
            true
        })
        .map(|i| i + 4)
        .unwrap()
}

fn solve2(input: &str) -> usize {
    input.chars().collect::<Vec<_>>()
        .windows(14)
        .position(|chars| {
            for char in chars {
                if chars.iter().filter(|c| *c == char).count() > 1 {
                    return false;
                }
            }
            true
        })
        .map(|i| i + 14)
        .unwrap()
}
