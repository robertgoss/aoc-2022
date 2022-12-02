use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use crate::calories;
use crate::game;
use crate::packing;

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

pub fn input_as_guide(day: i8) -> game::Guide {
    game::Guide::from_lines(&input_as_lines(day))
}

pub fn input_as_game(day: i8) -> game::Game {
    game::Game::from_lines(&input_as_lines(day))
}

pub fn input_as_rucksack(day: i8) -> Vec<packing::Rucksack> {
    input_as_lines(day).iter().map(
        |line| packing::Rucksack::from_line(line)
    ).collect()
}

pub fn input_as_groups(day: i8) -> Vec<packing::Group> {
    input_as_lines(day).iter().array_chunks::<3>().map(
        |line| packing::Group::from_lines(line)
    ).collect()
}


