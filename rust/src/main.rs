#![feature(iter_array_chunks)]
#![feature(btree_drain_filter)]
#![feature(hash_drain_filter)]
#![feature(option_result_contains)]
#![feature(iter_advance_by)]

extern crate pest;
#[macro_use]
extern crate pest_derive;

mod io;
mod calories;
mod game;
mod packing;
mod camp;
mod crates;
mod signal;
mod files;
mod trees;
mod rope;
mod crt;
mod monkey;
mod heights;
mod packets;
mod sand;
mod sensors;
mod pressure;
mod tetris;
mod lava;
mod robot;
mod riddle;
mod encrypted;
mod map;
mod planting;

mod challenge {
    use crate::packets::Packet;
    use crate::tetris::Game;
    use crate::encrypted;

    use super::io as io;

    fn challenge_1() {
        let data = io::input_as_elfs(1);
        println!("{:?}", data.best_elf());
    }

    fn challenge_2() {
        let data = io::input_as_elfs(1);
        println!("{:?}", data.best_elfs(3));
    }

    fn challenge_3() {
        let data = io::input_as_game(2);
        println!("{:?}", data.score());
    }

    fn challenge_4() {
        let data = io::input_as_guide(2);
        println!("{:?}", data.score());
    }

    fn challenge_5() {
        let data = io::input_as_rucksack(3);
        let res : usize = data.iter().map(|r| r.score()).sum();
        println!("{:?}", res);
    }

    fn challenge_6() {
        let data = io::input_as_groups(3);
        let res : usize = data.iter().map(|r| r.score()).sum();
        println!("{:?}", res);
    }

    fn challenge_7() {
        let data = io::input_as_assignment(4);
        let res : usize = data.iter().filter(|&r| r.contains()).count();
        println!("{:?}", res);
    }

    fn challenge_8() {
        let data = io::input_as_assignment(4);
        let res : usize = data.iter().filter(|&r| r.overlap()).count();
        println!("{:?}", res);
    }

    fn challenge_9() {
        let (mut data, moves) = io::input_as_crates(5);
        data.simulate(&moves, false);
        println!("{}", data.tops());
    }

    fn challenge_10() {
        let (mut data, moves) = io::input_as_crates(5);
        data.simulate(&moves, true);
        println!("{}", data.tops());
    }

    fn challenge_11() {
        let data = io::input_as_line(6);
        let res = crate::signal::start_of_packet(&data, 4);
        println!("{}", res);
    }

    fn challenge_12() {
        let data = io::input_as_line(6);
        let res = crate::signal::start_of_packet(&data, 14);
        println!("{}", res);
    }

    fn challenge_13() {
        let data = io::input_as_commands(7);
        let files = data.filesystem();
        let res = files.total(100000);
        println!("{}", res);
    }

    fn challenge_14() {
        let data = io::input_as_commands(7);
        let files = data.filesystem();
        let size = files.size();
        let res = files.smallest_dir(size - 40000000).unwrap();
        println!("{}", res);
    }

    fn challenge_15() {
        let data = io::input_as_forest(8);
        let res = data.count_visible();
        println!("{}", res);
    }

    fn challenge_16() {
        let data = io::input_as_forest(8);
        let res = data.scenic_max();
        println!("{}", res);
    }

    fn challenge_17() {
        let data = io::input_as_directions(9);
        let res = data.simulate(2).len();
        println!("{}", res);
    }

    fn challenge_18() {
        let data = io::input_as_directions(9);
        let res = data.simulate(10).len();
        println!("{}", res);
    }

    fn challenge_19() {
        let data = io::input_as_code(10);
        let signals = data.signals();
        let res = signals[19] + signals[59] + signals[99] + signals[139] + signals[179] + signals[219];
        println!("{}", res);
    }

    fn challenge_20() {
        let data = io::input_as_code(10);
        let res = data.display();
        println!("{}", res);
    }

    fn challenge_21() {
        let mut data = io::input_as_monkeys(11);
        data.simulate(20, true);
        let res = data.monkey_buisness();
        println!("{}", res);
    }

    fn challenge_22() {
        let mut data = io::input_as_monkeys(11);
        data.simulate(10000, false);
        let res = data.monkey_buisness();
        println!("{}", res);
    }

    fn challenge_23() {
        let data = io::input_as_heights(12);
        let res = data.distance();
        println!("{}", res);
    }

    fn challenge_24() {
        let data = io::input_as_heights(12);
        let res = data.distance_start();
        println!("{}", res);
    }

    fn challenge_25() {
        let data = io::input_as_packet_pairs(13);
        let res : usize = data.iter().enumerate().filter_map(
            |(i, pair)| if pair.ordered() { Some(i+1) } else { None }
        ).sum();
        println!("{}", res);
    }

    fn challenge_26() {
        let (div1, div2) = Packet::dividers();
        let mut data = io::input_as_packets(13);
        data.push(div1.clone());
        data.push(div2.clone());
        data.sort();
        let res : usize = data.iter().enumerate().filter_map(
            |(i,p)| if *p==div1 || *p==div2 { Some(i+1) } else { None }
        ).product();
        println!("{}", res);
    }

    fn challenge_27() {
        let mut data = io::input_as_cave(14);
        let res = data.pour_void();
        println!("{}", res);
    }

    fn challenge_28() {
        let mut data = io::input_as_cave(14);
        let res = data.pour_wall();
        println!("{}", res);
    }

    fn challenge_29() {
        let data = io::input_as_sensors(15);
        let res = data.no_becons_row(2000000);
        println!("{}", res);
    }

    fn challenge_30() {
        let data = io::input_as_sensors(15);
        let pt = data.find_beacon(0, 4000000, 0, 4000000).unwrap();
        let res = pt.0 * 4000000 + pt.1;
        println!("{}", res);
    }

    fn challenge_31() {
        let data = io::input_as_network(16);
        let res = data.maximum_pressure();
        println!("{}", res);
    }

    fn challenge_32() {
        let data = io::input_as_network(16);
        let res = data.maximum_pressure_dual();
        println!("{}", res);
    }

    fn challenge_33() {
        let mut data = io::input_as_jets(17);
        let mut game = Game::new();
        game.simulate(2022, &mut data);
        let res = game.height();
        println!("{}", res);
    }

    fn challenge_34() {
        let mut data = io::input_as_jets(17);
        let mut game = Game::new();
        let res = game.simulate_long(1000000000000, &mut data);
        println!("{}", res);
    }

    fn challenge_35() {
        let data = io::input_as_lava(18);
        let res = data.surface_area();
        println!("{}", res);
    }

    fn challenge_36() {
        let data = io::input_as_lava(18);
        let res = data.outside_surface_area();
        println!("{}", res);
    }

    fn challenge_37() {
        let data = io::input_as_blueprints(19);
        let res : usize = data.iter().enumerate().map(
            |(i, bp)|(i+1) * bp.max_geodes(26)
        ).sum();
        println!("{}", res);
    }

    fn challenge_39() {
        let data = io::input_as_ints(20);
        let encrypted = encrypted::File::make_file(&data);
        let res = encrypted.data(1000) + encrypted.data(2000) + encrypted.data(3000);
        println!("{}", res);
    }

    fn challenge_40() {
        let data = io::input_as_ints(20);
        let encrypted = encrypted::File::make_file_key(&data, 811589153, 10);
        let res = encrypted.data(1000) + encrypted.data(2000) + encrypted.data(3000);
        println!("{}", res);
    }

    fn challenge_41() {
        let data = io::input_as_riddles(21);
        let res = data.solve("root").unwrap();
        println!("{}", res);
    }

    fn challenge_42() {
        let data = io::input_as_riddles(21);
        let poly = data.root_eqn("humn");
        let res = poly.solve();
        println!("{:?}", res);
    }

    fn challenge_43() {
        let data = io::input_as_map(22);
        let res = data.simulate().val();
        println!("{:?}", res);
    }

    fn challenge_45() {
        let mut data = io::input_as_field(23);
        data.simulate(10);
        let res = data.empty_ground();
        println!("{:?}", res);
    }

    fn challenge_46() {
        let mut data = io::input_as_field(23);
        let res = data.simulate_till_still();
        println!("{:?}", res);
    }
   
    pub fn challenge(num : u8) {
        match num {
            1 => challenge_1(),
            2 => challenge_2(),
            3 => challenge_3(),
            4 => challenge_4(),
            5 => challenge_5(),
            6 => challenge_6(),
            7 => challenge_7(),
            8 => challenge_8(),
            9 => challenge_9(),
            10 => challenge_10(),
            11 => challenge_11(),
            12 => challenge_12(),
            13 => challenge_13(),
            14 => challenge_14(),
            15 => challenge_15(),
            16 => challenge_16(),
            17 => challenge_17(),
            18 => challenge_18(),
            19 => challenge_19(),
            20 => challenge_20(),
            21 => challenge_21(),
            22 => challenge_22(),
            23 => challenge_23(),
            24 => challenge_24(),
            25 => challenge_25(),
            26 => challenge_26(),
            27 => challenge_27(),
            28 => challenge_28(),
            29 => challenge_29(),
            30 => challenge_30(),
            31 => challenge_31(),
            32 => challenge_32(),
            33 => challenge_33(),
            34 => challenge_34(),
            35 => challenge_35(),
            36 => challenge_36(),
            37 => challenge_37(),
            39 => challenge_39(),
            40 => challenge_40(),
            41 => challenge_41(),
            42 => challenge_42(),
            43 => challenge_43(),
            45 => challenge_45(),
            46 => challenge_46(),
            _ => () 
        }
    }
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    let ver = args.get(1).unwrap().parse::<u8>().unwrap();
    challenge::challenge(ver);
}