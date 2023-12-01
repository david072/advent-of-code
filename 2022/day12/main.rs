pub fn main() -> utils::Result<()> {
    let input = utils::get_input(2022, 12)?;

    println!("Part One: {}", solve1(&input));
    println!("Part Two: {}", solve2(&input));

    Ok(())
}

fn solve1(input: &str) -> u64 {
    let mut start = (0usize, 0usize);
    let mut end = (0usize, 0usize);

    let map = input.lines()
        .enumerate()
        .map(|(y, line)| line.chars()
            .enumerate()
            .map(|(x, char)| {
                match char {
                    'S' => {
                        start = (y, x);
                        0
                    }
                    'E' => {
                        end = (y, x);
                        0
                    }
                    _ => char as u8 - 97,
                }
            })
            .collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut distances = vec![0u32; map.len()];
    let mut spt_set = vec![false; map.len()];

    distances[] = 0;

    0
}

fn solve2(input: &str) -> u64 {
    0
}
