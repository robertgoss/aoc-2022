mod io;

mod challenge {
    use super::io as io;

    fn challenge_1() {
        let data = io::input_as_list(1);
        println!("{:?}", data);
    }

   
    pub fn challenge(num : u8) {
        match num {
            1 => challenge_1(),
            _ => () 
        }
    }
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    let ver = args.get(1).unwrap().parse::<u8>().unwrap();
    challenge::challenge(ver);
}