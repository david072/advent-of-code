pub fn main() -> utils::Result<()> {
    let input = utils::get_input(2022, 2)?;

    println!("Part One: {}", solve1(&input));
    println!("Part Two: {}", solve2(&input));

    Ok(())
}

fn solve1(input: &str) -> u64 {
    input.lines()
        .map(|line| {
            let mut parts = line.split(' ');
            (parts.next().unwrap(), parts.next().unwrap())
        })
        .map(|shapes @ (_, me)| {
            let mut points = match me {
                "X" => 1,
                "Y" => 2,
                "Z" => 3,
                _ => unreachable!(),
            };
            points += match shapes {
                ("A", "Y") | ("B", "Z") | ("C", "X") => 6,
                ("A", "X") | ("B", "Y") | ("C", "Z") => 3,
                _ => 0
            };
            points
        })
        .sum()
}

fn solve2(input: &str) -> u64 {
    input.lines()
        .map(|line| {
            let mut parts = line.split(' ');
            (parts.next().unwrap(), parts.next().unwrap())
        })
        .map(|(opponent, target)| {
            match target {
                "X" => match opponent {
                    "A" => 3,
                    "B" => 1,
                    "C" => 2,
                    _ => unreachable!(),
                }
                "Y" => (opponent.chars().next().unwrap() as u64 - 64) + 3,
                "Z" => match opponent {
                    "A" => 2 + 6,
                    "B" => 3 + 6,
                    "C" => 1 + 6,
                    _ => unreachable!(),
                }
                _ => unreachable!(),
            }
        })
        .sum()
}
