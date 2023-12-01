use std::fmt::{Display, Formatter};

pub fn main() -> utils::Result<()> {
    let input = utils::get_input(2022, 13)?;

    println!("Part One: {}", solve1(&input));
    println!("Part Two: {}", solve2(&input));

    Ok(())
}

#[derive(Debug, Clone)]
enum Packet {
    Int(u8),
    List(Vec<Packet>),
}

impl Display for Packet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Packet::Int(n) => write!(f, "{n}"),
            Packet::List(packets) => {
                write!(f, "[")?;
                for (i, packet) in packets.iter().enumerate() {
                    write!(f, "{}", packet)?;
                    if i != packets.len() - 1 {
                        write!(f, ",")?;
                    }
                }
                write!(f, "]")
            }
        }
    }
}

impl Packet {
    pub fn parse_list(input: &str) -> Packet {
        let mut list = Vec::new();

        let mut i = 0;
        let chars = input[1..input.len() - 1].chars().collect::<Vec<_>>();
        while i < chars.len() {
            match chars[i] as u8 {
                b'0'..=b'9' => {
                    let mut number = String::new();
                    while i < chars.len() && chars[i].is_ascii_digit() {
                        number.push(chars[i]);
                        i += 1;
                    }
                    list.push(Packet::Int(number.parse::<u8>().unwrap()));
                }
                b'[' => {
                    let sub_list_start = i + 1;
                    let mut sub_list_end = 0usize;
                    let mut nesting_level = 0usize;
                    i += 1;
                    while i < chars.len() {
                        match chars[i] {
                            '[' => nesting_level += 1,
                            ']' => {
                                if nesting_level == 0 {
                                    sub_list_end = i + 1;
                                    break;
                                }
                                nesting_level -= 1;
                            }
                            _ => {}
                        }
                        i += 1;
                    }

                    list.push(Packet::parse_list(&input[sub_list_start..=sub_list_end]));
                }
                _ => {}
            }
            i += 1;
        }

        Packet::List(list)
    }

    pub fn compare(&self, other: &Packet) -> Option<bool> {
        match self {
            Packet::Int(num) => {
                match other {
                    Packet::Int(other) => {
                        if num == other {
                            None
                        } else {
                            Some(num <= other)
                        }
                    }
                    _ => {
                        let first = Packet::List(vec![Packet::Int(*num)]);
                        first.compare(other)
                    }
                }
            }
            Packet::List(list) => {
                match other {
                    Packet::List(other) => {
                        let mut i = 0usize;
                        while i < list.len() {
                            let Some(other) = other.get(i) else {
                                return Some(false);
                            };
                            if let Some(result) = list[i].compare(other) {
                                return Some(result);
                            }
                            i += 1;
                        }
                        if list.len() == other.len() {
                            None
                        } else {
                            Some(true)
                        }
                    }
                    Packet::Int(other) => {
                        let other = Packet::List(vec![Packet::Int(*other)]);
                        self.compare(&other)
                    }
                }
            }
        }
    }
}

fn solve1(input: &str) -> usize {
    input.split("\n\n")
        .map(|pair| {
            let mut lines = pair.lines();
            let first = lines.next().unwrap();
            let second = lines.next().unwrap();
            (Packet::parse_list(first), Packet::parse_list(second))
        })
        .enumerate()
        .filter_map(|(i, (first, second))| {
            if let Some(true) = first.compare(&second) {
                Some(i + 1)
            } else {
                None
            }
        })
        .sum()
}

fn solve2(input: &str) -> usize {
    let mut packets = input.lines()
        .filter(|line| !line.is_empty())
        .map(Packet::parse_list)
        .collect::<Vec<_>>();
    packets.push(Packet::parse_list("[[2]]"));
    packets.push(Packet::parse_list("[[6]]"));

    let mut result: Vec<Packet> = Vec::new();
    'outer: for packet in packets {
        if result.is_empty() {
            result.push(packet);
            continue;
        }

        for (i, first) in result.iter().enumerate() {
            if !first.compare(&packet).unwrap_or(false) {
                continue;
            }
            if i != result.len() - 1 {
                let other = &result[i + 1];
                if !packet.compare(other).unwrap_or(false) {
                    continue;
                }
            }
            result.insert(i + 1, packet);
            continue 'outer;
        }

        result.insert(0, packet);
    }

    result.iter()
        .enumerate()
        .filter(|(_, packet)| {
            let Packet::List(list) = packet else { return false; };
            if list.len() != 1 { return false; }
            let Packet::List(list) = &list[0] else { return false; };
            if list.len() != 1 { return false; }
            let Packet::Int(num) = &list[0] else { return false; };
            *num == 2 || *num == 6
        })
        .map(|(i, _)| i + 1)
        .product()
}
