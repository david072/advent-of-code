use std::collections::HashMap;

pub fn main() -> utils::Result<()> {
    let input = utils::get_input(2022, 7)?;

    println!("Part One: {}", solve1(&input));
    println!("Part Two: {}", solve2(&input));

    Ok(())
}

fn solve1(input: &str) -> u64 {
    get_sizes(input)
        .values()
        .filter(|size| **size <= 100_000)
        .sum()
}

fn get_sizes(input: &str) -> HashMap<String, u64> {
    let mut sizes: HashMap<String, u64> = HashMap::new();
    let mut current_path: Vec<String> = Vec::new();

    let mut i = 0usize;
    let lines = input.lines().collect::<Vec<_>>();
    while let Some(line) = lines.get(i) {
        let mut args = line[2..].split(' ');
        let command = args.next().unwrap();
        match command {
            "cd" => {
                match args.next().unwrap() {
                    ".." => { current_path.pop(); }
                    "/" => current_path = vec!["".to_string()],
                    name => { current_path.push(name.to_string()); }
                }
                i += 1;
            }
            "ls" => {
                i += 1;
                while let Some(output_line) = lines.get(i) {
                    if output_line.starts_with('$') { break; }
                    let mut parts = output_line.split(' ');
                    i += 1;
                    let Ok(file_size) = parts.next().unwrap().parse::<u64>() else { continue; };

                    let mut dir_name = String::from("/");
                    for dir in &current_path {
                        dir_name.push('/');
                        dir_name.push_str(dir);
                        let size = sizes.entry(dir_name.clone()).or_insert(0);
                        *size += file_size;
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    sizes
}

fn solve2(input: &str) -> u64 {
    let sizes = get_sizes(input);
    let used_size = sizes.iter()
        .find(|(name, _)| *name == "//")
        .map(|(_, size)| *size)
        .unwrap();
    let needed_size = 30000000 - (70000000 - used_size);

    sizes.values()
        .filter(|size| **size >= needed_size)
        .copied()
        .min()
        .unwrap()
}
