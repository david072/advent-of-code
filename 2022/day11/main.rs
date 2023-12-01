pub fn main() -> utils::Result<()> {
    let input = utils::get_input(2022, 11)?;

    println!("Part One: {}", solve1(&input));
    println!("Part Two: {}", solve2(&input));

    Ok(())
}

struct Monkey1 {
    items: Vec<u64>,
    items_inspected: u64,
    operation: (bool, Option<u64>),
    test_divisor: u64,
    test_success_dest: usize,
    test_failure_dest: usize,
}

impl Monkey1 {
    pub fn handle_next_item(&mut self) -> Option<usize> {
        let Some(item) = self.items.first_mut() else { return None; };
        self.items_inspected += 1;

        let new_worry_level = (if self.operation.0 {
            *item * self.operation.1.unwrap_or(*item)
        } else {
            *item + self.operation.1.unwrap_or(*item)
        }) / 3;
        *item = new_worry_level;

        if new_worry_level % self.test_divisor == 0 {
            Some(self.test_success_dest)
        } else {
            Some(self.test_failure_dest)
        }
    }
}

struct Monkey2 {
    items: Vec<Vec<u64>>,
    items_inspected: u64,
    operation: (bool, Option<u64>),
    test_divisor: i64,
    test_success_dest: usize,
    test_failure_dest: usize,
}

impl Monkey2 {
    pub fn from_monkey1(monkey: &Monkey1) -> Self {
        Self {
            items: monkey.items.iter().map(|n| vec![*n]).collect::<Vec<_>>(),
            items_inspected: monkey.items_inspected,
            operation: monkey.operation,
            test_divisor: monkey.test_divisor as i64,
            test_success_dest: monkey.test_success_dest,
            test_failure_dest: monkey.test_failure_dest,
        }
    }

    pub fn handle_next_item(&mut self) -> Option<usize> {
        let Some(numbers) = self.items.first_mut() else { return None; };
        self.items_inspected += 1;

        Some(self.test_success_dest)
    }
}

fn parse_monkeys(input: &str) -> Vec<Monkey1> {
    input.split("\n\n")
        .map(|monkey_line| {
            let mut lines = monkey_line.lines();
            let _ = lines.next();

            let starting_items_line = lines.next().unwrap();
            let colon = starting_items_line.find(|c| c == ':').unwrap();
            let starting_items = starting_items_line[colon + 2..]
                .split(", ")
                .map(|item| item.parse::<u64>().unwrap())
                .collect::<Vec<_>>();

            let operation_line = lines.next().unwrap();
            let equals_sign = operation_line.find('=').unwrap();
            let mut parts = operation_line[equals_sign + 6..].split(' ');
            let is_multiplication = parts.next().unwrap() == "*";
            let number = parts.next().unwrap().parse::<u64>().ok();

            let test_divisor = lines.next().unwrap()
                .split(' ').last()
                .and_then(|n| n.parse::<u64>().ok())
                .unwrap();

            let test_success_dest = lines.next().unwrap()
                .split(' ').last()
                .and_then(|n| n.parse::<usize>().ok())
                .unwrap();
            let test_failure_dest = lines.next().unwrap()
                .split(' ').last()
                .and_then(|n| n.parse::<usize>().ok())
                .unwrap();

            Monkey1 {
                items: starting_items,
                items_inspected: 0,
                operation: (is_multiplication, number),
                test_divisor,
                test_success_dest,
                test_failure_dest,
            }
        })
        .collect::<Vec<_>>()
}

fn solve1(input: &str) -> u64 {
    let mut monkeys = parse_monkeys(input);

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            for _ in 0..monkeys[i].items.len() {
                let Some(destination) = monkeys[i].handle_next_item() else { continue; };
                let item = monkeys[i].items.remove(0);
                monkeys[destination].items.push(item);
            }
        }
    }

    let mut items_inspected = monkeys.iter()
        .map(|monkey| monkey.items_inspected)
        .collect::<Vec<_>>();
    items_inspected.sort();

    items_inspected[items_inspected.len() - 1] * items_inspected[items_inspected.len() - 2]
}

fn solve2(input: &str) -> u64 {
    let mut monkeys = parse_monkeys(input).iter()
        .map(Monkey2::from_monkey1)
        .collect::<Vec<_>>();

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            for _ in 0..monkeys[i].items.len() {
                let Some(destination) = monkeys[i].handle_next_item() else { continue; };
                let item = monkeys[i].items.remove(0);
                monkeys[destination].items.push(item);
            }
        }
    }

    let mut items_inspected = monkeys.iter()
        .map(|monkey| monkey.items_inspected)
        .collect::<Vec<_>>();
    println!("items_inspected: {items_inspected:?}");
    items_inspected.sort();

    items_inspected[items_inspected.len() - 1] * items_inspected[items_inspected.len() - 2]
}
