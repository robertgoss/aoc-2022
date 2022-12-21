use std::collections::HashMap;
use std::cmp::max;

#[derive(Clone, Copy, Debug)]
enum Dir {
    N,
    E,
    S,
    W
}

#[derive(Debug)]
pub struct State {
    dir : Dir,
    pos : (i64, i64)
}

#[derive(Debug)]
enum Instruction {
    Left,
    Right,
    Forward(usize)
}

pub struct Map {
    map : HashMap<(i64, i64), bool>,
    min : (i64, i64),
    max : (i64, i64),
    instructions : Vec<Instruction>
}

fn parse_instructions(string : &str) -> Vec<Instruction> {
    let mut iter = string.chars();
    let mut end = false;
    let mut instructions = Vec::new();
    while !end {
        let num_s = iter.clone().take_while(|c| c.is_numeric()).collect::<String>();
        if !num_s.is_empty() {
            let res = iter.advance_by(num_s.len());
            if res.is_err() {
                end = true
            }
            instructions.push(Instruction::Forward(num_s.parse().unwrap()));
        } else {
            match iter.next() {
                Some('R') => instructions.push(Instruction::Right),
                Some('L') => instructions.push(Instruction::Left),
                _ => end = true
            }
        }
    } 
    println!("{:?}", instructions);
    instructions
}

impl Map {
    pub fn from_lines(lines : &Vec<String>) -> Map {
        let last_str = lines.last().unwrap();
        let mut map = HashMap::new();
        let mut max_i : (i64, i64) = (0,0);
        for (j, line) in lines.split_last().unwrap().1.iter().enumerate() {
            for (i,c) in line.char_indices() {
                if c == '.' {
                    map.insert((i as i64, j as i64), false);
                }
                if c == '#' {
                    map.insert((i as i64, j as i64), true);
                }
                max_i.1 = max(max_i.1, j as i64);
            }
            max_i.0 = max(max_i.0, j as i64);
        }
        let instructions = parse_instructions(last_str);

        Map {
            map : map,
            min : (0,0),
            max : max_i,
            instructions : instructions
        }
    }

    pub fn simulate(&self) -> State {
        let top = self.min.1;
        let start = (self.range_row(top).0, top);
        let mut state = State { pos : start, dir : Dir::E };
        println!("Start: {:?}", state);
        for instruction in self.instructions.iter() {
            state = self.do_instruction(instruction, &state);
            println!("{:?}", state);
        }
        state
    }

    fn range_row(&self, i : i64) -> (i64, i64) {
        let min = (self.min.0 ..= self.max.0).find(
            |j| self.map.contains_key(&(*j,i))
        ).unwrap();
        let max = (self.min.0 ..= self.max.0).rev().find(
            |j| self.map.contains_key(&(*j,i))
        ).unwrap();
        (min, max)
    }

    fn range_col(&self, i : i64) -> (i64, i64) {
        let min = (self.min.1 ..= self.max.1).find(
            |j| self.map.contains_key(&(i,*j))
        ).unwrap();
        let max = (self.min.1 ..= self.max.1).rev().find(
            |j| self.map.contains_key(&(i,*j))
        ).unwrap();
        (min, max)
    }

    fn do_instruction(&self, instruction : &Instruction, state : &State) -> State {
        match instruction {
            Instruction::Left => State { pos : state.pos, dir : state.dir.left() },
            Instruction::Right => State { pos : state.pos, dir : state.dir.right() },
            Instruction::Forward(n) => State { pos : self.move_forward(state.pos, *n, &state.dir), dir : state.dir}
        }
    }

    fn move_forward(&self, pos : (i64, i64), num : usize, dir : &Dir) -> (i64, i64) {
        let mut new_pos = pos;
        for _ in 0..num {
            new_pos = self.move_one(new_pos, dir);
        }
        new_pos
    }

    fn move_one(&self, pos : (i64, i64), dir : &Dir) -> (i64,i64) {
        let new_pos = match dir {
            Dir::N => self.move_one_col(pos, true),
            Dir::S => self.move_one_col(pos, false),
            Dir::W => self.move_one_row(pos, true),
            Dir::E => self.move_one_row(pos, false)
        };
        if *self.map.get(&new_pos).unwrap() {
            pos
        } else {
            new_pos
        }
    }

    fn move_one_row(&self, pos : (i64, i64), sign : bool) -> (i64,i64) {
        let (min, max) = self.range_row(pos.1);
        let mut new_pos = if sign {
            (pos.0 - 1, pos.1)
        } else {
            (pos.0 + 1, pos.1)
        };
        if new_pos.0 < min {
            new_pos.0 = max;
        }
        if new_pos.0 > max {
            new_pos.0 = min;
        }
        new_pos
    }

    fn move_one_col(&self, pos : (i64, i64), sign : bool) -> (i64,i64) {
        let (min, max) = self.range_col(pos.0);
        let mut new_pos = if sign {
            (pos.0, pos.1 - 1)
        } else {
            (pos.0, pos.1 + 1)
        };
        if new_pos.1 < min {
            new_pos.1 = max;
        }
        if new_pos.1 > max {
            new_pos.1 = min;
        }
        new_pos
    }


}

impl State {
    pub fn val(&self) -> usize {
        (1000*(self.pos.1+1) + 4*(self.pos.0+1) + self.dir.val()) as usize
    }
}

impl Dir {
    fn val(&self) -> i64 {
        match self {
            Dir::E => 0,
            Dir::S => 1,
            Dir::W => 2,
            Dir::N => 3
        }
    }

    fn left(&self) -> Dir {
        match self {
            Dir::E => Dir::N,
            Dir::S => Dir::E,
            Dir::W => Dir::S,
            Dir::N => Dir::W
        }
    }

    fn right(&self) -> Dir {
        match self {
            Dir::E => Dir::S,
            Dir::S => Dir::W,
            Dir::W => Dir::N,
            Dir::N => Dir::E
        }
    }
}