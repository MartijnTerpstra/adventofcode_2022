use itertools::Itertools;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

fn main() {
    start_of_signal(4);
    start_of_signal(14);
}

fn start_of_signal(char_count: usize) {
    if let Ok(signal) = read_text("input.txt") {
        let sop = signal
            .chars()
            .batching(|e| {
                let r = e.clone().take(char_count).collect_vec();
                match e.next() {
                    None => None,
                    Some(_) => Some(r),
                }
            })
            .enumerate()
            .find(|e| e.1.clone().into_iter().all_unique())
            .unwrap()
            .0
            + char_count;

        println!("Start of signal with {} distinct chars: {}", char_count, sop);
    }
}

fn read_text<P>(filename: P) -> io::Result<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let mut s = String::new();
    return io::BufReader::new(file)
        .read_to_string(&mut s)
        .and_then(|_| Ok(s));
}
