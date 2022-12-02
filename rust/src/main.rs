#![feature(iter_array_chunks)]


mod io;
mod calories;
mod game;
mod packing;

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

   
    pub fn challenge(num : u8) {
        match num {
            1 => challenge_1(),
            2 => challenge_2(),
            3 => challenge_3(),
            4 => challenge_4(),
            5 => challenge_5(),
            6 => challenge_6(),
            _ => () 
        }
    }
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    let ver = args.get(1).unwrap().parse::<u8>().unwrap();
    challenge::challenge(ver);
}