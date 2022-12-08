use itertools::{Chunk, Itertools};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    accumulate_duplicate_items();
    accumulate_badges();
}

fn find_errors(e: String) -> String {
    let splits = e.split_at(e.len() / 2);
    let l = splits.0.chars().sorted().dedup();
    let r = splits.1.chars().sorted().dedup();
    return l.interleave(r).sorted().duplicates().collect::<String>();
}

fn to_score(e: String) -> u32 {
    return e
        .chars()
        .map(|c| {
            if c.is_uppercase() {
                c as u32 - 'A' as u32 + 27
            } else {
                c as u32 - 'a' as u32 + 1
            }
        })
        .sum();
}

fn accumulate_duplicate_items() {
    if let Ok(lines) = read_lines("input.txt") {
        let result: u32 = lines
            .filter_map(|e| e.ok())
            .map(|e| find_errors(e))
            .map(|e| to_score(e))
            .sum();

        println!("Duplicate item score: {}", result);
    }
}

fn find_badge<IterT>(elfs: Chunk<IterT>) -> String
where
    IterT: Iterator,
    IterT::Item: ToString,
{
    let mut v = Vec::new();
    for e in elfs {
        for c in e.to_string().chars().sorted().dedup() {
            v.push(c);
        }
    }
    return v
        .into_iter()
        .counts()
        .into_iter()
        .filter(|e| e.1 == 3)
        .map(|e| e.0)
        .collect::<String>();
}

fn accumulate_badges() {
    if let Ok(lines) = read_lines("input.txt") {
        let result: u32 = lines
            .filter_map(|e| e.ok())
            .chunks(3)
            .into_iter()
            .map(|e| find_badge(e))
            .map(|e| to_score(e))
            .sum();

        println!("Badge item score: {}", result);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
