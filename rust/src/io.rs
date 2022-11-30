use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use crate::calories;

pub fn input_as_lines(day: i8) -> Vec<String> {
    let filename = format!("../data/day-{}.txt", day);
    let file = File::open(filename).expect("Issue opening file");
    let reader = BufReader::new(&file);
    reader.lines().map(
        |s| s.expect("Read failure")
    ).collect()
}

pub fn input_as_elfs(day: i8) -> calories::Elfs {
    calories::Elfs::from_lines(&input_as_lines(day))
}

