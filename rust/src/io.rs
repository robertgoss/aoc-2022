use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;


pub fn input_as_list(day: i8) -> Vec<i64> {
    let filename = format!("../data/day-{}.txt", day);
    let file = File::open(filename).expect("Issue opening file");
    let reader = BufReader::new(&file);
    reader.lines().map(
        |s| s.expect("Read failure").parse::<i64>().unwrap()
    ).collect()
}

