use std::fs::{read_to_string, File};
use std::io::BufReader;
use std::io::BufRead;

use crate::{calories, tetris, riddle};
use crate::game;
use crate::packing;
use crate::camp;
use crate::crates;
use crate::files;
use crate::trees;
use crate::rope;
use crate::crt;
use crate::monkey;
use crate::heights;
use crate::packets;
use crate::sand;
use crate::sensors;
use crate::pressure;
use crate::lava;
use crate::robot;
use crate::map;
use crate::planting;

pub fn input_as_lines(day: i8) -> Vec<String> {
    let filename = format!("../data/day-{}.txt", day);
    let file = File::open(filename).expect("Issue opening file");
    let reader = BufReader::new(&file);
    reader.lines().map(
        |s| s.expect("Read failure")
    ).collect()
}

pub fn input_as_string(day: i8) -> String {
    let filename = format!("../data/day-{}.txt", day);
    read_to_string(filename).expect("Read failure")
}

pub fn input_as_line(day: i8) -> String {
    input_as_lines(day).into_iter().next().unwrap()
}

pub fn input_as_ints(day: i8) -> Vec<i64> {
    input_as_lines(day).into_iter().filter_map(
        |line| line.parse::<i64>().ok()
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

pub fn input_as_forest(day: i8) -> trees::Forest {
    trees::Forest::from_lines(&input_as_lines(day))
}

pub fn input_as_field(day: i8) -> planting::Field {
    planting::Field::from_lines(&input_as_lines(day))
}

pub fn input_as_commands(day : i8) -> files::Commands {
    files::Commands::from_string(&input_as_string(day))
}

pub fn input_as_monkeys(day : i8) -> monkey::Monkeys {
    monkey::Monkeys::from_string(&input_as_string(day))
}

pub fn input_as_directions(day : i8) -> rope::Directions {
    rope::Directions::from_lines(&input_as_lines(day))
}

pub fn input_as_code(day : i8) -> crt::CPU {
    crt::CPU::from_lines(&input_as_lines(day))
}

pub fn input_as_jets(day : i8) -> tetris::Jets {
    tetris::Jets::from_line(&input_as_line(day))
}

pub fn input_as_lava(day : i8) -> lava::Droplets {
    lava::Droplets::from_lines(&input_as_lines(day))
}

pub fn input_as_sensors(day : i8) -> sensors::Sensors {
    sensors::Sensors::from_lines(&input_as_lines(day))
}

pub fn input_as_map(day : i8) -> map::Map {
    map::Map::from_lines(&input_as_lines(day))
}

pub fn input_as_network(day : i8) -> pressure::Network {
    pressure::Network::from_lines(&input_as_lines(day))
}

pub fn input_as_packet_pairs(day : i8) -> Vec<packets::PacketPair> {
    let str = input_as_string(day);
    str.split("\n\n").filter_map(
        |str| packets::PacketPair::from_string(str)
    ).collect()
}

pub fn input_as_packets(day : i8) -> Vec<packets::Packet> {
    input_as_lines(day).iter().filter_map(
        |str| packets::Packet::from_line(str)
    ).collect()
}

pub fn input_as_heights(day : i8) -> heights::HeightMap {
    heights::HeightMap::from_lines(&input_as_lines(day))
}

pub fn input_as_cave(day : i8) -> sand::Cave {
    sand::Cave::from_lines(&input_as_lines(day))
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

pub fn input_as_assignment(day: i8) -> Vec<camp::AssignmentPair> {
    input_as_lines(day).iter().filter_map(
        |line| camp::AssignmentPair::from_line(line)
    ).collect()
}

pub fn input_as_blueprints(day: i8) -> Vec<robot::Blueprint> {
    input_as_lines(day).iter().filter_map(
        |line| robot::Blueprint::from_line(line)
    ).collect()
}

pub fn input_as_riddles(day: i8) -> riddle::Riddle {
    riddle::Riddle::from_lines(&input_as_lines(day))
}

pub fn input_as_crates(day: i8) -> (crates::Crates, Vec<crates::Move>) {
    let all_lines = input_as_lines(day);
    let init_lines : Vec<&String> = all_lines.iter().take_while(
        |l| l.len() > 0
    ).collect();
    let rest_lines : Vec<&String> = all_lines.iter().skip_while(
        |l| l.len() > 0
    ).skip(1).collect();
    let init = crates::Crates::from_lines(&init_lines).unwrap();
    let moves = rest_lines.iter().filter_map(
        |line| crates::Move::from_line(line)
    ).collect();
    (init, moves)
}

