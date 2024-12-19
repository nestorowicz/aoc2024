use std::io::{read_to_string, stdin};
use scanf::sscanf;

use aoc::{debug, is_debug};

const ADV: u64 = 0;
const BXL: u64 = 1;
const BST: u64 = 2;
const JNZ: u64 = 3;
const BXC: u64 = 4;
const OUT: u64 = 5;
const BDV: u64 = 6;
const CDV: u64 = 7;

#[derive(Debug)]
struct Input {
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,
    program: Vec<u64>,
    ip: usize
}

fn main() {
    let input = parse_input();
    debug!("{:?}", input);

    run_program(input);
}

fn dv(prog: &Input, operand: u64) -> u64 {
    return prog.reg_a / 2u64.pow(combo(&prog, operand) as u32)
}

fn run_program(mut prog: Input) {
    let mut output: Vec<u64> = vec![];
    loop {
        let Some(opcode) = prog.program.get(prog.ip).map(|c| *c) else { break };
        prog.ip += 1;
        let Some(operand) = prog.program.get(prog.ip).map(|c| *c) else { break };
        prog.ip += 1;

        debug!("{:?} {:?} {:?}", opcode, operand, prog);

        // The adv instruction (opcode 0) performs division. The numerator is the value in the A register. The denominator is found by raising 2 to the power of the instruction's combo operand. (So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A by 2^B.) The result of the division operation is truncated to an integer and then written to the A register.
        if opcode == ADV { prog.reg_a = dv(&prog, operand); continue; }

        // The bxl instruction (opcode 1) calculates the bitwise XOR of register B and the instruction's literal operand, then stores the result in register B.
        if opcode == BXL { prog.reg_b = prog.reg_b ^ operand; continue; }

        // The bst instruction (opcode 2) calculates the value of its combo operand modulo 8 (thereby keeping only its lowest 3 bits), then writes that value to the B register.
        if opcode == BST { prog.reg_b = combo(&prog, operand) % 8; continue; }

        // The jnz instruction (opcode 3) does nothing if the A register is 0. However, if the A register is not zero, it jumps by setting the instruction pointer to the value of its literal operand; if this instruction jumps, the instruction pointer is not increased by 2 after this instruction. 
        if opcode == JNZ { if prog.reg_a != 0 { prog.ip = operand as usize; } continue; }

        // The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C, then stores the result in register B. (For legacy reasons, this instruction reads an operand but ignores it.)
        if opcode == BXC { prog.reg_b = prog.reg_b ^ prog.reg_c; continue; }

        // The out instruction (opcode 5) calculates the value of its combo operand modulo 8, then outputs that value. (If a program outputs multiple values, they are separated by commas.)
        if opcode == OUT { output.push(combo(&prog, operand) % 8); continue; }

        // The bdv instruction (opcode 6) works exactly like the adv instruction except that the result is stored in the B register. (The numerator is still read from the A register.)
        if opcode == BDV { prog.reg_b = dv(&prog, operand); continue; }

        // The cdv instruction (opcode 7) works exactly like the adv instruction except that the result is stored in the C register. (The numerator is still read from the A register.)
        if opcode == CDV { prog.reg_c = dv(&prog, operand); continue; }
    }

    let output_str = output.iter().map(|n| n.to_string()).collect::<Vec<String>>().join(",");
    println!("{}", output_str);
}

fn combo(prog: &Input, op: u64) -> u64 {
    // Combo operands 0 through 3 represent literal values 0 through 3.
    // Combo operand 4 represents the value of register A.
    // Combo operand 5 represents the value of register B.
    // Combo operand 6 represents the value of register C.
    // Combo operand 7 is reserved and will not appear in valid programs.
    if op <= 3 || op > 6 { return op }
    if op == 4 { return prog.reg_a }
    if op == 5 { return prog.reg_b }
    if op == 6 { return prog.reg_c }
    panic!("unexpected combo operand");
}   

fn parse_input() -> Input {
    let input = read_to_string(stdin()).unwrap();
    let mut lines = input.lines();
    let mut line = lines.next();

    let mut reg_a: u64 = 0;
    sscanf!(line.unwrap(), "Register A: {}", reg_a).expect("failed to parse register A");
    line = lines.next();

    let mut reg_b: u64 = 0;
    sscanf!(line.unwrap(), "Register B: {}", reg_b).expect("failed to parse register B");
    line = lines.next();

    let mut reg_c: u64 = 0;
    sscanf!(line.unwrap(), "Register C: {}", reg_c).expect("failed to parse register C");
    _ = lines.next();

    line = lines.next();

    let mut program_str: String = String::new();
    sscanf!(line.unwrap(), "Program: {}", program_str).expect("failed to parse program");
    let program = program_str.split(",").filter_map(|s| s.parse::<u64>().ok()).collect();

    return Input{reg_a, reg_b, reg_c, program, ip: 0}
}

