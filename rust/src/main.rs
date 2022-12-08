#![feature(iter_array_chunks)]


mod io;
mod calories;
mod game;
mod packing;
mod camp;
mod crates;
mod signal;
mod files;
mod trees;

mod challenge {
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
            _ => () 
        }
    }
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    let ver = args.get(1).unwrap().parse::<u8>().unwrap();
    challenge::challenge(ver);
}