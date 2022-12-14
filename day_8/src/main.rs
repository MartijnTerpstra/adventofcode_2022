use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let heightmap = create_heightmap();
    count_visible_trees(&heightmap);
    max_visiblity_treehouse(&heightmap);
}

fn max_visiblity_treehouse(heightmap: &HeightMap) {
    let mut best_score = 0;
    for ((row, column), tree_height) in heightmap.trees.iter() {
        let trees = &heightmap.trees;
        let left_score = (1
            + (0..*column)
                .rev()
                .map(|c| trees.get(&(*row, c)).unwrap())
                .take_while(|h| h < &tree_height)
                .count())
        .min(*column);
        let up_score = (1
            + (0..*row)
                .rev()
                .map(|r| trees.get(&(r, *column)).unwrap())
                .take_while(|h| h < &tree_height)
                .count())
        .min(*row);
        let right_score = (1
            + (column + 1..heightmap.columns)
                .map(|c| trees.get(&(*row, c)).unwrap())
                .take_while(|h| h < &tree_height)
                .count())
        .min(heightmap.columns - column - 1);
        let down_score = (1
            + (row + 1..heightmap.rows)
                .map(|r| trees.get(&(r, *column)).unwrap())
                .take_while(|h| h < &tree_height)
                .count())
        .min(heightmap.rows - row - 1);
        let score = left_score * right_score * up_score * down_score;
        if score > best_score {
            best_score = score;
        }
    }
    println!("Best tree house visibility score: {}", best_score);
}

fn count_visible_trees(heightmap: &HeightMap) {
    let mut count = 0;
    for ((row, column), tree_height) in heightmap.trees.iter() {
        let trees = &heightmap.trees;
        let is_visible = (0..*column)
            .map(|c| trees.get(&(*row, c)).unwrap())
            .all(|h| h < tree_height)
            || (0..*row)
                .map(|r| trees.get(&(r, *column)).unwrap())
                .all(|h| h < tree_height)
            || (column + 1..heightmap.columns)
                .map(|c| trees.get(&(*row, c)).unwrap())
                .all(|h| h < tree_height)
            || (row + 1..heightmap.rows)
                .map(|r| trees.get(&(r, *column)).unwrap())
                .all(|h| h < tree_height);
        if is_visible {
            count += 1;
        }
    }
    println!("Amount of visible trees: {}", count);
}

struct HeightMap {
    trees: HashMap<(usize, usize), u32>,
    rows: usize,
    columns: usize,
}

fn create_heightmap() -> HeightMap {
    let mut heightmap = HashMap::new();
    if let Ok(lines) = read_lines("input.txt") {
        let mut rows: usize = 0;
        let mut columns: usize = 0;
        lines.filter_map(|e| e.ok()).enumerate().for_each(|(r, l)| {
            rows = r + 1;
            l.chars().enumerate().for_each(|(c, ch)| {
                heightmap.insert((r, c), ch as u32);
                columns = c + 1;
            });
        });
        return HeightMap {
            trees: heightmap,
            rows: rows,
            columns: columns,
        };
    }
    return HeightMap {
        trees: heightmap,
        rows: 0,
        columns: 0,
    };
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
