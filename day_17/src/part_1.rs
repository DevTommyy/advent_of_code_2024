use std::fs;

fn parse_input(filename: &str) -> (Vec<i64>, Vec<i64>) {
    let input = fs::read_to_string(filename).expect("failed to read file");

    let mut split = input.split("\n\n");

    let registers_unparsed = split.next().unwrap();
    let instructions_unparsed = split.next().unwrap();

    let registers: Vec<i64> = registers_unparsed
        .lines()
        .flat_map(|line| {
            line.split(":")
                .last()
                .map(|reg| reg.trim().parse::<i64>().unwrap())
        })
        .collect();

    let instructions: Vec<i64> = instructions_unparsed
        .split(":")
        .last()
        .unwrap()
        .trim()
        .split(",")
        .map(|val| val.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    (registers, instructions)
}

fn execute(registers: &mut [i64], instructions: Vec<i64>) -> String {
    let mut ip = 0;
    let mut output = String::new();

    while ip < instructions.len() {
        let opcode = instructions[ip];
        let operand = instructions[ip + 1];
        // println!("ip = {ip}, opcode = {opcode}, operand = {operand}");

        match opcode {
            0 => registers[0] /= 1 << get_combo_val(registers, operand),
            1 => registers[1] ^= operand,
            2 => registers[1] = get_combo_val(registers, operand) % 8,
            3 if registers[0] != 0 => {
                ip = operand as usize;
                continue;
            }
            3 => {}
            4 => registers[1] ^= registers[2],
            5 => {
                if !output.is_empty() {
                    output.push(',');
                }
                output.push_str(&format!("{}", get_combo_val(registers, operand) % 8));
            }
            6 => registers[1] = registers[0] / (1 << get_combo_val(registers, operand)),
            7 => registers[2] = registers[0] / (1 << get_combo_val(registers, operand)),
            _ => unreachable!(),
        }

        ip += 2;
    }

    output
}

fn get_combo_val(registers: &[i64], operand: i64) -> i64 {
    match operand {
        x @ 0..=3 => x,
        4 => registers[0],
        5 => registers[1],
        6 => registers[2],
        _ => unreachable!(),
    }
}

fn main() {
    let filename = advent_of_code_2024::load_input();
    let (mut registers, instructions) = parse_input(&filename);

    let output = execute(&mut registers, instructions);
    println!("{output}");
}
