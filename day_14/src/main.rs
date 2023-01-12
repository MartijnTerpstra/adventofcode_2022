use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;

fn main() {
    drop_sand_till_filled(&mut CaveMap::new(), "part_1");

    drop_sand_till_filled(&mut CaveMap::new_with_infinite_floor(), "part_2");
}

fn drop_sand_till_filled(map: &mut CaveMap, name: &str) {
    let mut sand_counter = 0;
    let mut result: Result;
    loop {
        result = drop_sand((500, 0), map);
        if result != Result::NewSand {
            break;
        }
        sand_counter += 1;
    }
    print_map(&map, name);
    match result {
        Result::LastSandPlaced => println!("Poured {} sand till it stabilized", sand_counter + 1),
        Result::SanddFallsInVoid => {
            println!("Poured {} sand till it fell into the void", sand_counter)
        }
        Result::NewSand => panic!("result should not be NewSand"),
    };
}

#[derive(PartialEq)]
enum Result {
    NewSand,
    SanddFallsInVoid,
    LastSandPlaced,
}

fn drop_sand(sand: (i32, i32), cave: &mut CaveMap) -> Result {
    for iy in sand.1 + 1..cave.depth {
        if cave.map.contains_key(&(sand.0, iy)) {
            if !cave.map.contains_key(&(sand.0 - 1, iy)) {
                return drop_sand((sand.0 - 1, iy), cave);
            }
            if !cave.map.contains_key(&(sand.0 + 1, iy)) {
                return drop_sand((sand.0 + 1, iy), cave);
            }
            let new_sand_location = (sand.0, iy - 1);
            cave.map.insert(new_sand_location, NodeType::Sand);

            if new_sand_location == (500, 0) {
                return Result::LastSandPlaced;
            } else {
                return Result::NewSand;
            }
        }
    }
    return Result::SanddFallsInVoid;
}

fn print_map(cave: &CaveMap, name: &str) {
    let file = File::create(format!("output_{}.txt", name)).unwrap();
    let mut writer = io::BufWriter::new(file);

    for y in 0..cave.depth {
        let mut line = String::new();
        for x in cave.start_x..cave.start_x + cave.width {
            match cave.map.get(&(x, y)) {
                Some(NodeType::Rock) => line.push('#'),
                Some(NodeType::Sand) => line.push('o'),
                None => line.push('.'),
            };
        }
        writeln!(&mut writer, "{}", line).unwrap();
    }
}

enum NodeType {
    Rock,
    Sand,
}

struct CaveMap {
    map: HashMap<(i32, i32), NodeType>,
    width: i32,
    depth: i32,
    start_x: i32,
}

impl CaveMap {
    fn new() -> CaveMap {
        let re = Regex::new(r"(\d+),(\d+)").unwrap();

        let mut occupied_points = HashMap::new();
        let mut width = 0;
        let mut depth = 0;
        let mut start_x = 500;
        for line in read_lines("input.txt") {
            let mut prev_location = (-1, -1);
            for captures in re.captures_iter(line.as_str()) {
                let x = captures[1].parse().unwrap();
                let y = captures[2].parse().unwrap();
                width = width.max(x + 1);
                depth = depth.max(y + 1);
                start_x = start_x.min(x);

                if prev_location != (-1, -1) {
                    if prev_location.0 == x {
                        let from_y = prev_location.1.min(y);
                        let till_y = prev_location.1.max(y) + 1;
                        for iy in from_y..till_y {
                            occupied_points.insert((x, iy), NodeType::Rock);
                        }
                    } else {
                        assert!(prev_location.0 != x, "{:?} != {:?}", prev_location, (x, y));
                        let from_x = prev_location.0.min(x);
                        let till_x = prev_location.0.max(x) + 1;
                        for ix in from_x..till_x {
                            occupied_points.insert((ix, y), NodeType::Rock);
                        }
                    }
                }
                prev_location = (x, y);
            }
        }

        return CaveMap {
            map: occupied_points,
            width,
            depth,
            start_x,
        };
    }

    fn new_with_infinite_floor() -> CaveMap {
        let mut map = CaveMap::new();
        let y = map.depth + 1;
        let min_x = 500 - map.depth - 5;
        let max_x = 500 + map.depth + 6;
        map.depth += 2;
        for ix in min_x..max_x {
            map.map.insert((ix, y), NodeType::Rock);
        }
        map.start_x = min_x;
        map.width = max_x - min_x;
        print_map(&map, "test");
        return map;
    }
}

fn read_lines<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    return io::BufReader::new(file)
        .lines()
        .filter_map(|e| e.ok())
        .collect_vec();
}
