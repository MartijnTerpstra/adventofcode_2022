use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut step1_scoring = HashMap::<String, i32>::new();
    step1_scoring.insert("A X".to_string(), 4);
    step1_scoring.insert("B X".to_string(), 1);
    step1_scoring.insert("C X".to_string(), 7);
    step1_scoring.insert("A Y".to_string(), 8);
    step1_scoring.insert("B Y".to_string(), 5);
    step1_scoring.insert("C Y".to_string(), 2);
    step1_scoring.insert("A Z".to_string(), 3);
    step1_scoring.insert("B Z".to_string(), 9);
    step1_scoring.insert("C Z".to_string(), 6);
    calculate_input_score("step 1", step1_scoring);

    let mut step2_scoring = HashMap::<String, i32>::new();
    step2_scoring.insert("A X".to_string(), 3);
    step2_scoring.insert("B X".to_string(), 1);
    step2_scoring.insert("C X".to_string(), 2);
    step2_scoring.insert("A Y".to_string(), 4);
    step2_scoring.insert("B Y".to_string(), 5);
    step2_scoring.insert("C Y".to_string(), 6);
    step2_scoring.insert("A Z".to_string(), 8);
    step2_scoring.insert("B Z".to_string(), 9);
    step2_scoring.insert("C Z".to_string(), 7);
    calculate_input_score("step 2", step2_scoring);
}

fn calculate_input_score(algo: &str, score_mapping: HashMap<String, i32>) {
    if let Ok(lines) = read_lines("input.txt") {
        let score: i32 = lines
            .filter_map(|e| e.ok())
            .filter_map(|e| score_mapping.get(&e))
            .sum();

        println!("Score from {}: {}", algo, score);
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
