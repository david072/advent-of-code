pub fn main() -> utils::Result<()> {
    let input = utils::get_input(2022, 10)?;

    println!("Part One: {}", solve1(&input));
    println!("Part Two: {}", solve2(&input));

    Ok(())
}

fn solve1(input: &str) -> i32 {
    const SEARCHED_CYCLES: [usize; 6] = [20, 60, 100, 140, 180, 220];

    let mut result = 0i32;
    let mut cycle = 0usize;
    let mut searched_cycle_idx = 0usize;
    let mut x = 1i32;
    for line in input.lines().collect::<Vec<_>>() {
        let mut parts = line.split(' ');
        let to_add = match parts.next().unwrap() {
            "addx" => {
                cycle += 2;
                parts.next().unwrap().parse::<i32>().unwrap()
            }
            "noop" => {
                cycle += 1;
                0
            }
            _ => unreachable!(),
        };

        let searched_cycle = SEARCHED_CYCLES[searched_cycle_idx];
        if cycle == searched_cycle || cycle - 1 == searched_cycle {
            result += searched_cycle as i32 * x;
            searched_cycle_idx += 1;
            if searched_cycle_idx >= SEARCHED_CYCLES.len() {
                break;
            }
        }

        x += to_add;
    }

    result
}

fn solve2(input: &str) -> String {
    let mut image = "\n".to_string();

    let instructions = input.lines().collect::<Vec<_>>();
    let mut pixel_index = 0i32;
    let mut current_instruction = 0usize;
    let mut cycles_left_for_instruction = 0usize;
    let mut to_add = 0i32;
    let mut x = 1i32;

    for _ in 0..240 {
        let char = if (x - 1..=x + 1).contains(&pixel_index) { '#' } else { '.' };
        image.push(char);
        pixel_index += 1;
        if pixel_index == 40 {
            pixel_index = 0;
            image.push('\n');
        }

        if cycles_left_for_instruction > 0 {
            cycles_left_for_instruction -= 1;
            x += to_add;
        }
        else {
            let mut parts = instructions[current_instruction].split(' ');
            match parts.next().unwrap() {
                "addx" => {
                    to_add = parts.next().unwrap().parse::<i32>().unwrap();
                    cycles_left_for_instruction = 1;
                },
                "noop" => {
                    to_add = 0;
                    cycles_left_for_instruction = 0;
                },
                _ => unreachable!(),
            }
            current_instruction += 1;
        }
    }

    image
}
