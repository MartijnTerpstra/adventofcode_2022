use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    gen_n_highest(1);
    gen_n_highest(3);
}

fn gen_n_highest(n : usize)
{
    if let Ok(lines) = read_lines("input.txt") {
        let max_resource : i32 = lines
            .filter_map(|e| e.ok())
            .map(|e| e.parse::<i32>())
            .coalesce(|l, r| {
                if l.is_ok() && r.is_ok() {
                    Ok(Ok(l.unwrap() + r.unwrap()))
                } else {
                    Err((l, r))
                }
            })
            .filter_map(|e| e.ok())
            .sorted()
            .rev()
            .take(n)
            .sum();
        println!("Highest {} groups of numbers from input.txt: {}", n, max_resource);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
