use itertools::Itertools;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::slice::Iter;

fn main() {
    let fs = get_filesystem_from_commands();
    part_1(&fs, 100000);
    part_2(&fs, 70000000, 30000000);
}

fn part_1(fs: &FileSystem, at_most: usize) {
    let total_size: usize = fs
        .iter()
        .filter(|e| fs.size(e.id) <= at_most)
        .map(|e| fs.size(e.id))
        .sum();
    println!("Total size with at most 100000 {}", total_size);
}

fn part_2(fs: &FileSystem, total_disk_space: usize, disk_space_req_for_update: usize) {
    let max_allowed_used_space = total_disk_space - disk_space_req_for_update;
    let current_total_used_space = fs.size(0);
    let min_deletion_space = current_total_used_space - max_allowed_used_space;
    let space_to_delete = fs
        .iter()
        .filter(|e| fs.size(e.id) >= min_deletion_space)
        .map(|e| fs.size(e.id))
        .sorted()
        .next()
        .unwrap();

    println!("Space to delete {}", space_to_delete);
}

struct FileSize {
    size: usize,
}

struct Directory {
    id: usize,
    name: String,
    parent: Option<usize>,
    files: Vec<FileSize>,
    dirs: Vec<usize>
}

struct FileSystem {
    dirs: Vec<Directory>,
}

impl FileSystem {
    fn new() -> FileSystem {
        FileSystem {
            dirs: vec![Directory {
                id: 0,
                name: "/".to_string(),
                parent: None,
                files: Vec::new(),
                dirs: Vec::new()
            }],
        }
    }

    fn new_dir(&mut self, n: &str, p: usize) -> &Directory {
        let new_id = self.dirs.len();
        self.dirs.push(Directory {
            id: new_id,
            name: n.to_string(),
            parent: Some(p),
            files: Vec::new(),
            dirs: Vec::new()
        });
        self.dirs.get_mut(p).unwrap().dirs.push(new_id);
        return self.dirs.get(new_id).unwrap();
    }

    fn get(&self, i: usize) -> &Directory {
        return self.dirs.get(i).unwrap();
    }

    fn get_mut(&mut self, i: usize) -> &mut Directory {
        return self.dirs.get_mut(i).unwrap();
    }

    fn iter(&self) -> Iter<'_, Directory> {
        return self.dirs.iter();
    }

    fn size(&self, current: usize) -> usize {
        let mut total_size = 0;
        let dir = self.get(current);
        for f in dir.files.iter() {
            total_size += f.size;
        }
        for d in dir.dirs.iter() {
            total_size += self.size(d.clone());
        }
        return total_size;
    }
}

fn get_filesystem_from_commands() -> FileSystem {
    let mut fs = FileSystem::new();
    let mut current = 0;

    let cd_up = Regex::new(r"^\$ cd \.\.$").unwrap();
    let cd_root = Regex::new(r"^\$ cd /$").unwrap();
    let cd_down = Regex::new(r"^\$ cd (.*)$").unwrap();
    let ls = Regex::new(r"^\$ ls$").unwrap();
    let dir = Regex::new(r"^dir (.*)$").unwrap();
    let file = Regex::new(r"^(\d+) .*$").unwrap();

    if let Ok(lines) = read_lines("input.txt") {
        for l in lines.filter_map(|e| e.ok()) {
            if cd_up.is_match(&l) {
                current = fs.get(current).parent.unwrap();
            } else if cd_root.is_match(&l) {
                current = 0;
            } else if let Some(dirname) = cd_down.captures(&l) {
                current = fs
                    .get(current)
                    .dirs
                    .iter()
                    .find(|e| fs.get(*e.clone()).name == dirname[1])
                    .unwrap()
                    .clone();
            } else if ls.is_match(&l) {
                continue;
            } else if let Some(dirname) = dir.captures(&l) {
                fs.new_dir(&dirname[1], current);
            }
            if let Some(filesize) = file.captures(&l) {
                fs.get_mut(current).files.push(FileSize {
                    size: filesize[1].parse().unwrap(),
                })
            }
        }
    }
    return fs;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
