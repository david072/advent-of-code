use regex::Regex;

const INPUT: &str = include_str!("../day3.txt");

fn part1() {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let res: i32 = re
        .captures_iter(INPUT)
        .map(|c| c.extract())
        .map(|(_, [n1, n2])| n1.parse::<i32>().unwrap() * n2.parse::<i32>().unwrap())
        .sum();
    println!("Result: {res}");
}

fn part2() {
    let re = Regex::new(r"(do\(\))|(don't\(\))|(mul\((\d{1,3}),(\d{1,3})\))").unwrap();
    let mut enable_mul = true;
    let mut result = 0i32;
    for c in re.captures_iter(INPUT) {
        // do instruction
        if c.get(1).is_some() {
            enable_mul = true;
            continue;
        }
        // don't instruction
        else if c.get(2).is_some() {
            enable_mul = false;
            continue;
        }

        if enable_mul {
            // mul instruction
            result += c.get(4).unwrap().as_str().parse::<i32>().unwrap()
                * c.get(5).unwrap().as_str().parse::<i32>().unwrap();
        }
    }

    println!("Result: {result}");
}

fn main() {
    part1();
    part2();
}
