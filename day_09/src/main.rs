use itertools::Itertools;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    count_unique_visited_tail_locations(1);
    count_unique_visited_tail_locations(9);
}

fn count_unique_visited_tail_locations(tail_size: usize) {
    let unique_locations = get_visited_tail_locations(tail_size)
        .into_iter()
        .sorted()
        .unique()
        .count();

    println!(
        "Unique locations visited by tail with size {}: {}",
        tail_size, unique_locations
    );
}

fn get_visited_tail_locations(tail_size: usize) -> Vec<(i32, i32)> {
    let regex = Regex::new(r"(.) (\d+)").unwrap();
    let mut locations = vec![(0, 0)];
    let mut body = Vec::new();
    body.resize(1 + tail_size, (0, 0));
    if let Ok(lines) = read_lines("input.txt") {
        for l in lines.filter_map(|e| e.ok()) {
            let instruction = regex.captures(&l).unwrap();
            let steps = instruction[2].parse::<i32>().unwrap();
            let head = body[0];
            match &instruction[1] {
                "D" => body[0] = (head.0, head.1 - steps),
                "U" => body[0] = (head.0, head.1 + steps),
                "L" => body[0] = (head.0 - steps, head.1),
                "R" => body[0] = (head.0 + steps, head.1),
                _ => panic!("Unknown instruction"),
            };

            let mut has_changes = true;
            while has_changes {
                has_changes = false;
                for i in 1..body.len() {
                    let head = body[i - 1];
                    if is_disconnected(body[i], head) {
                        has_changes = true;
                        body[i] = follow_head(body[i], head);
                        if i == body.len() - 1 {
                            locations.push(body[i]);
                        }
                    }
                }
            }
        }
    }
    return locations;
}

fn print_world(body: &Vec<(i32, i32)>) {
    print!("\x1B[2J\x1B[1;1H");
    for y in -10..11 {
        let mut s = String::new();
        for x in -10..11 {
            if body[0] == (x, y) {
                s.push('H');
                continue;
            }
            let mut any_tail_encountered = false;
            for i in 1..body.len() {
                if body[i] == (x, y) {
                    s.push(i.to_string().chars().next().unwrap());
                    any_tail_encountered = true;
                    break;
                }
            }
            if any_tail_encountered {
                continue;
            }
            if (0, 0) == (x, y) {
                s.push('s');
                continue;
            }
            s.push('#');
        }
        println!("{}", s);
    }
    sleep(Duration::from_secs(1));
}

fn is_disconnected(tail: (i32, i32), head: (i32, i32)) -> bool {
    return tail.0.abs_diff(head.0) > 1 || tail.1.abs_diff(head.1) > 1;
}

fn follow_head(mut tail: (i32, i32), head: (i32, i32)) -> (i32, i32) {
    if tail.0 != head.0 {
        tail.0 += if head.0 > tail.0 { 1 } else { -1 };
    }
    if tail.1 != head.1 {
        tail.1 += if head.1 > tail.1 { 1 } else { -1 };
    }
    return tail;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
