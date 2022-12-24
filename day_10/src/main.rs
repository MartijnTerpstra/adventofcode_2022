use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let instructions = get_cycles_and_instructions();
    calculate_sum_of_signal_strengths(&instructions);
    print_crt_display(&instructions);
}

fn print_crt_display(instructions: &[Instruction]) {
    for row_start in (0..240).step_by(40) {
        let mut row_display = String::new();
        for row_x in 0..40 {
            let cycle = row_start + row_x;
            let reg_x = instructions
                .iter()
                .take_while(|e| e.cycle <= cycle)
                .last()
                .map(|e| e.x)
                .unwrap_or(1);

            if reg_x.abs_diff(row_x) <= 1 {
                row_display.push('#');
            } else {
                row_display.push('.');
            }
        }
        println!("{}", row_display);
    }
}

fn calculate_sum_of_signal_strengths(instructions: &[Instruction]) {
    let signal_cycles = (20..221).step_by(40);
    let sum: i32 = signal_cycles
        .map(|c| {
            instructions
                .iter()
                .take_while(|e| e.cycle < c)
                .last()
                .unwrap()
                .x
                * c
        })
        .sum();

    println!("Signal strength: {}", sum);
}

#[derive(Clone)]
struct Instruction {
    cycle: i32,
    x: i32,
}

fn get_instruction_from_line(l: &str, instruction: &mut Instruction) -> Instruction {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(^noop$|addx (-?\d+))").unwrap();
    }

    let captures = RE.captures(l).unwrap();
    match &captures[1] {
        "noop" => instruction.cycle += 1,
        _ => {
            instruction.x += captures[2].parse::<i32>().unwrap();
            instruction.cycle += 2;
        }
    }
    return instruction.clone();
}

fn get_cycles_and_instructions() -> Vec<Instruction> {
    if let Ok(lines) = read_lines("input.txt") {
        let mut instruction = Instruction { cycle: 0, x: 1 };
        let instructions = lines
            .filter_map(|e| e.ok())
            .map(|e| get_instruction_from_line(e.as_str(), &mut instruction))
            .collect_vec();

        return instructions;
    }
    return Vec::new();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
