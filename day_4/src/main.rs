use itertools::{Itertools};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    fully_overlapping();
    partially_overlapping();
}

fn fully_overlapping() {
    if let Ok(lines) = read_lines("input.txt") {
        let overlaps = lines.filter_map(|e| e.ok()).filter(|e| total_overlaps(e)).count();
        println!("Fully overlapping areas: {}", overlaps);
    }
}

fn partially_overlapping() {
    if let Ok(lines) = read_lines("input.txt") {
        let overlaps = lines.filter_map(|e| e.ok()).filter(|e| partial_overlaps(e)).count();
        println!("Partially overlapping areas: {}", overlaps);
    }
}

fn to_range(s: &str) -> (i32, i32) {
    let parts = s.split('-').collect_vec();
    return (parts[0].parse().unwrap(), parts[1].parse().unwrap());
}

fn total_overlaps(s: &String) -> bool {
    let split = s.split(',').collect_vec();

    let range0 = to_range(split[0]);
    let range1 = to_range(split[1]);

    return (range0.0 <= range1.0 && range0.1 >= range1.1)
        || (range0.0 >= range1.0 && range0.1 <= range1.1);
}

fn partial_overlaps(s: &String) -> bool {
    let split = s.split(',').collect_vec();

    let range0 = to_range(split[0]);
    let range1 = to_range(split[1]);

    return range0.0 <= range1.1 && range0.1 >= range1.0;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
