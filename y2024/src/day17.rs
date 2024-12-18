use std::{
    rc::Rc,
    sync::{atomic::AtomicBool, Arc},
};

const INPUT: &str = include_str!("../day17.txt");

fn next_output(
    registers: &mut [u64],
    program: &[u64],
    instruction_pointer: &mut usize,
) -> Option<u64> {
    fn get_combo_operand_value(registers: &[u64], operand: u64) -> u64 {
        if operand < 4 {
            operand
        } else {
            registers[(operand - 4) as usize]
        }
    }

    while *instruction_pointer < program.len() - 1 {
        let opcode = program[*instruction_pointer];
        let operand = program[*instruction_pointer + 1];
        match opcode {
            // adv
            0 => {
                // The adv instruction (opcode 0) performs division. The numerator is the value in
                // the A register.
                // The denominator is found by raising 2 to the power of the instruction's combo
                // operand.
                // The result of the division operation is truncated to an integer and then written
                // to the A register.
                registers[0] /= 2u64.pow(get_combo_operand_value(&registers, operand) as u32);
            }
            // bxl
            1 => {
                // The bxl instruction (opcode 1) calculates the bitwise XOR of register B and the
                // instruction's literal operand, then stores the result in register B.
                registers[1] ^= operand;
            }
            // bst
            2 => {
                // The bst instruction (opcode 2) calculates the value of its combo operand modulo
                // 8 (thereby keeping only its lowest 3 bits), then writes that value to the B
                // register.
                registers[1] = get_combo_operand_value(&registers, operand) & 0b111;
            }
            // jnz
            3 => {
                // The jnz instruction (opcode 3) does nothing if the A register is 0.
                if registers[0] != 0 {
                    // However, if the A register is not zero, it jumps by setting the instruction
                    // pointer to the value of its literal operand;
                    *instruction_pointer = operand as usize;
                    // if this instruction jumps, the instruction pointer is not increased by 2
                    // after this instruction.
                    continue;
                }
            }
            // bxc
            4 => {
                // The bxc instruction (opcode 4) calculates the bitwise XOR of register B and
                // register C, then stores the result in register B. (For legacy reasons, this
                // instruction reads an operand but ignores it.)
                registers[1] ^= registers[2];
            }
            // out
            5 => {
                // The out instruction (opcode 5) calculates the value of its combo operand
                // modulo 8, then outputs that value. (If a program outputs multiple values,
                // they are separated by commas.)
                *instruction_pointer += 2;
                return Some(get_combo_operand_value(&registers, operand) & 0b111);
            }
            // bdv
            6 => {
                // The bdv instruction (opcode 6) works exactly like the adv instruction except
                // that the result is stored in the B register. (The numerator is still read from
                // the A register.)
                registers[1] =
                    registers[0] / 2u64.pow(get_combo_operand_value(&registers, operand) as u32);
            }
            // cdv
            7 => {
                // The cdv instruction (opcode 7) works exactly like the adv instruction except
                // that the result is stored in the C register. (The numerator is still read from
                // the A register.)
                registers[2] =
                    registers[0] / 2u64.pow(get_combo_operand_value(&registers, operand) as u32);
            }
            _ => {}
        }

        *instruction_pointer += 2;
    }

    None
}

fn run_program(mut registers: Vec<u64>, program: &[u64]) -> Vec<u64> {
    let mut outputs = vec![];
    let mut instruction_pointer = 0usize;
    while let Some(output) = next_output(&mut registers, program, &mut instruction_pointer) {
        outputs.push(output);
    }
    outputs
}

fn part1(registers: Vec<u64>, program: &[u64]) {
    let result = run_program(registers, program)
        .into_iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(",");
    println!("result: {result}");
}

fn main() {
    let (registers, program) = INPUT.split_once("\n\n").unwrap();
    let registers = registers
        .trim()
        .lines()
        .map(|l| l[12..].parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let program = program[9..]
        .trim()
        .split(',')
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    part1(registers.clone(), &program);
}
