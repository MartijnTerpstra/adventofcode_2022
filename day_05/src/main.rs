use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    handle_crate_instructions(false);
    handle_crate_instructions(true);
}

fn handle_crate_instructions(keep_order: bool) {
    if let Ok(lines) = read_lines("input.txt") {
        let mut stack = setup_stacks();
        for l in lines.filter_map(|e| e.ok()) {
            handle_instruction(l, keep_order, &mut stack);
        }

        let result = stack
            .into_iter()
            .sorted_by(|l, r| Ord::cmp(&l.0, &r.0))
            .filter(|(_, v)| !v.is_empty())
            .filter_map(|(_, v)| v.last().cloned())
            .collect::<String>();
        println!("Result with keep order {}: {}", keep_order, result);
    }
}

fn handle_instruction(l: String, keep_order: bool, stack: &mut HashMap<char, Vec<char>>) {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let groups = re.captures(&l).unwrap();

    let count = groups[1].parse::<usize>().unwrap();
    let from = groups[2].chars().nth(0).unwrap();
    let to = groups[3].chars().nth(0).unwrap();
    assert!(from != to);
    if keep_order {
        let mut cs = Vec::new();
        for _ in 0..count {
            cs.push(stack.get_mut(&from).unwrap().pop().unwrap());
        }
        while !cs.is_empty() {
            stack.get_mut(&to).unwrap().push(cs.pop().unwrap());
        }
    } else {
        for _ in 0..count {
            let c = stack.get_mut(&from).unwrap().pop().unwrap();
            stack.get_mut(&to).unwrap().push(c);
        }
    }
}

fn setup_stacks() -> HashMap<char, Vec<char>> {
    if let Ok(lines) = read_lines("setup.txt") {
        let stacks = lines
            .collect_vec()
            .into_iter()
            .filter_map(|e| e.ok())
            .rev()
            .collect_vec();

        return stacks[0]
            .chars()
            .enumerate()
            .filter(|e| e.1.is_ascii_digit())
            .map(|e| (e.1, create_stacks(e.0, &stacks)))
            .collect();
    }
    return HashMap::new();
}

fn create_stacks(e: usize, stacks: &Vec<String>) -> Vec<char> {
    return stacks
        .into_iter()
        .skip(1)
        .map(|s| s.chars().nth(e).unwrap())
        .filter(|c| c.is_alphabetic())
        .collect_vec();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
