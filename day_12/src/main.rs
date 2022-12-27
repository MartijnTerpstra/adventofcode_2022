use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let heightmap = create_heightmap();
    dijkstra_shortest_path(&heightmap);
}

fn dijkstra_shortest_path(heightmap: &HeightMap) {
    let mut options = vec![(2, heightmap.start_point)]; // Why 2? account for begin and end step
    let mut visited = HashMap::new();
    visited.insert(heightmap.start_point, heightmap.start_point);

    while !options.is_empty() && options[0].1 != heightmap.end_point {
        let (distance, location) = options[0];

        let height = *heightmap.elevation.get(&location).unwrap();

        if location.0 > 0 {
            try_insert_location(
                distance,
                height,
                location,
                (location.0 - 1, location.1),
                &mut visited,
                &mut options,
                heightmap,
            );
        }

        if location.0 < heightmap.rows - 1 {
            try_insert_location(
                distance,
                height,
                location,
                (location.0 + 1, location.1),
                &mut visited,
                &mut options,
                heightmap,
            );
        }

        if location.1 > 0 {
            try_insert_location(
                distance,
                height,
                location,
                (location.0, location.1 - 1),
                &mut visited,
                &mut options,
                heightmap,
            );
        }

        if location.1 < heightmap.columns - 1 {
            try_insert_location(
                distance,
                height,
                location,
                (location.0, location.1 + 1),
                &mut visited,
                &mut options,
                heightmap,
            );
        }

        options.remove(0);
        options.sort_by(|l, r| l.0.cmp(&r.0));
    }

    if options.is_empty() {
        println!("Shortest path to end not found!");
        for r in 0..heightmap.rows {
            let mut line = String::new();
            for c in 0..heightmap.columns {
                let height = heightmap.elevation.get(&(r,c)).unwrap();
                if visited.get(&(r, c)).is_some() {
                    line.push(height.to_ascii_uppercase());
                }
                else {
                    line.push(*height);
                }
            }
            println!("{}", line);
        }
    } else {
        println!("Shortest path to end: {}", options[0].0);
        let mut location = heightmap.end_point;
        let mut path = HashSet::new();
        path.insert(location);
        while location != heightmap.start_point {
            location = *visited.get(&location).unwrap() ;
            path.insert(location);
        }
        let mut step_count_till_first_a = 2; // Starting from 2 to account for begin and end step
        location = heightmap.end_point;
        while *heightmap.elevation.get(&location).unwrap() != 'a' {
            step_count_till_first_a += 1;
            location = *visited.get(&location).unwrap();
        }
        println!("Shorted a to end: {}", step_count_till_first_a);
        for r in 0..heightmap.rows {
            let mut line = String::new();
            for c in 0..heightmap.columns {
                let height = heightmap.elevation.get(&(r,c)).unwrap();
                if path.contains(&(r, c)) {
                    line.push(height.to_ascii_uppercase());
                }
                else {
                    line.push(*height);
                }
            }
            println!("{}", line);
        }
    }
}

fn try_insert_location(
    distance: i32,
    height: char,
    prev_location: (usize, usize),
    location: (usize, usize),
    visited: &mut HashMap<(usize, usize), (usize, usize)>,
    options: &mut Vec<(i32, (usize, usize))>,
    heightmap: &HeightMap,
) {
    let new_height = *heightmap.elevation.get(&location).unwrap();
    let is_visitable = (height == 'S' && new_height == 'a') || new_height == 'E' || height as i32 + 1 >= new_height as i32;

    if is_visitable && !visited.contains_key(&location) {
        visited.insert(location, prev_location);
        options.push((distance + 1, location));
    }
}

struct HeightMap {
    elevation: HashMap<(usize, usize), char>,
    rows: usize,
    columns: usize,
    start_point: (usize, usize),
    end_point: (usize, usize),
}

fn create_heightmap() -> HeightMap {
    let mut heightmap = HashMap::new();
    let mut start_point = None;
    let mut end_point = None;
    let lines = read_lines("input.txt").unwrap();
    let mut rows: usize = 0;
    let mut columns: usize = 0;
    lines.filter_map(|e| e.ok()).enumerate().for_each(|(r, l)| {
        rows = r + 1;
        l.chars().enumerate().for_each(|(c, ch)| {
            heightmap.insert((r, c), ch);
            columns = c + 1;
            if ch == 'S' {
                start_point = Some((r, c));
            }
            if ch == 'E' {
                end_point = Some((r, c));
            }
        });
    });
    return HeightMap {
        elevation: heightmap,
        rows: rows,
        columns: columns,
        start_point: start_point.unwrap(),
        end_point: end_point.unwrap(),
    };
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
