const INPUT: &str = include_str!("../day4.txt");

fn part1() {
    let input = INPUT
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut count = 0;
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            fn check(c1: char, c2: char, c3: char, c4: char) -> bool {
                (c1 == 'X' && c2 == 'M' && c3 == 'A' && c4 == 'S')
                    || (c1 == 'S' && c2 == 'A' && c3 == 'M' && c4 == 'X')
            }

            // vertical
            if y < input.len() - 3
                && check(
                    input[y][x],
                    input[y + 1][x],
                    input[y + 2][x],
                    input[y + 3][x],
                )
            {
                count += 1;
            }

            // horizontal
            if x < input[0].len() - 3
                && check(
                    input[y][x],
                    input[y][x + 1],
                    input[y][x + 2],
                    input[y][x + 3],
                )
            {
                count += 1;
            }

            if x < input[0].len() - 3
                && y < input.len() - 3
                && check(
                    input[y][x],
                    input[y + 1][x + 1],
                    input[y + 2][x + 2],
                    input[y + 3][x + 3],
                )
            {
                count += 1;
            }

            if x > 2
                && y < input.len() - 3
                && check(
                    input[y][x],
                    input[y + 1][x - 1],
                    input[y + 2][x - 2],
                    input[y + 3][x - 3],
                )
            {
                count += 1;
            }
        }
    }

    println!("Count: {count}");
}

fn part2() {
    let input = INPUT
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut count = 0;
    for y in 0..input.len() - 2 {
        for x in 0..input[0].len() - 2 {
            fn check(c1: char, c2: char, c3: char) -> bool {
                (c1 == 'M' && c2 == 'A' && c3 == 'S') || (c1 == 'S' && c2 == 'A' && c3 == 'M')
            }

            if check(input[y][x], input[y + 1][x + 1], input[y + 2][x + 2])
                && check(input[y][x + 2], input[y + 1][x + 1], input[y + 2][x])
            {
                count += 1;
            }
        }
    }

    println!("Count: {count}");
}

fn main() {
    part1();
    part2();
}
