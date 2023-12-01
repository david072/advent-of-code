pub fn main() -> utils::Result<()> {
    let input = utils::get_input(2022, 5)?;

    println!("Part One: {}", solve1(&input));
    println!("Part Two: {}", solve2(&input));

    Ok(())
}

fn solve1(input: &str) -> String {
    let mut parts = input.split("\n\n");
    let mut stacks = parse_stacks(parts.next().unwrap());

    for instruction in parts.next().unwrap()
        .lines()
        .map(|line| line.split(' ')
            .filter_map(|p| p.parse::<usize>().ok())
            .collect::<Vec<_>>()) {
        let [count, origin, destination] = instruction.as_slice() else { continue; };

        for _ in 0..*count {
            let Some(value) = stacks[*origin - 1].pop() else { break; };
            stacks[*destination - 1].push(value);
        }
    }

    stacks.iter()
        .map(|stack| stack.last().map(|s| s.to_owned()).unwrap_or_default())
        .fold(String::new(), |str, v| str + &v.to_string())
}

fn parse_stacks(stacks: &str) -> Vec<Vec<char>> {
    let mut result: Vec<Vec<char>> = Vec::new();

    let mut lines = stacks.lines().peekable();
    while let Some(line) = lines.next() {
        if lines.peek().is_none() { break; }

        let mut i = 0usize;
        let mut start = 0usize;
        while start + 3 <= line.len() {
            let slice = &line[start..start + 3];
            if slice.starts_with('[') {
                while result.len() <= i { result.push(Vec::new()); }
                result[i].insert(0, slice.chars().nth(1).unwrap());
            }

            i += 1;
            start += 4;
        }
    }

    result
}

fn solve2(input: &str) -> String {
    let mut parts = input.split("\n\n");
    let mut stacks = parse_stacks(parts.next().unwrap());

    for instruction in parts.next().unwrap()
        .lines()
        .map(|line| line.split(' ')
            .filter_map(|p| p.parse::<usize>().ok())
            .collect::<Vec<_>>()) {
        let [count, origin, destination] = instruction.as_slice() else { continue; };

        let i = stacks[*destination - 1].len();
        for _ in 0..*count {
            let Some(value) = stacks[*origin - 1].pop() else { break; };
            stacks[*destination - 1].insert(i, value);
        }
    }

    stacks.iter()
        .map(|stack| stack.last().map(|s| s.to_owned()).unwrap_or_default())
        .fold(String::new(), |str, v| str + &v.to_string())
}
