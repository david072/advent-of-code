use std::collections::HashMap;

use regex::Regex;

const INPUT: &str = include_str!("../day24.txt");

enum Operation {
    And,
    Or,
    Xor,
}

struct Gate {
    lhs: &'static str,
    rhs: &'static str,
    output: &'static str,
    operation: Operation,
}

impl Gate {
    fn perform_operation(&self, lhs: bool, rhs: bool) -> bool {
        match self.operation {
            Operation::And => lhs && rhs,
            Operation::Or => lhs || rhs,
            Operation::Xor => lhs ^ rhs,
        }
    }
}

fn main() {
    let (initial_wires, gates) = INPUT.split_once("\n\n").unwrap();
    let mut wires =
        HashMap::<&str, Option<bool>>::from_iter(initial_wires.trim().lines().map(|l| {
            let (name, value) = l.split_once(": ").unwrap();
            (name, Some(value == "1"))
        }));

    let re = Regex::new(r"(\w+) (AND|OR|XOR) (\w+) -> (\w+)").unwrap();
    let mut gates = re
        .captures_iter(gates.trim())
        .map(|caps| caps.extract())
        .map(|(_, [lhs, op, rhs, output])| Gate {
            lhs,
            rhs,
            output,
            operation: match op {
                "AND" => Operation::And,
                "OR" => Operation::Or,
                "XOR" => Operation::Xor,
                _ => unreachable!(),
            },
        })
        .collect::<Vec<_>>();

    while !gates.is_empty() {
        let mut i = 0usize;
        while i < gates.len() {
            let gate = &gates[i];
            if wires.get(gate.lhs).and_then(|v| *v).is_some()
                && wires.get(gate.rhs).and_then(|v| *v).is_some()
            {
                if !wires.contains_key(gate.output) {
                    wires.insert(gate.output, None);
                }

                *wires.get_mut(gate.output).unwrap() = Some(
                    gate.perform_operation(wires[gate.lhs].unwrap(), wires[gate.rhs].unwrap()),
                );
                gates.remove(i);
                continue;
            }

            i += 1;
        }
    }

    let mut result_wires = wires
        .into_iter()
        .filter(|(k, _)| k.starts_with('z'))
        .collect::<Vec<_>>();
    result_wires.sort_by_key(|(k, _)| *k);
    let result = result_wires.into_iter().rev().fold(0usize, |acc, (_, v)| {
        (acc | if v.unwrap() { 1 } else { 0 }) << 1
    }) >> 1;
    println!("result: {result}");
}
