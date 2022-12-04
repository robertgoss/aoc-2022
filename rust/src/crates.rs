pub struct Move {
    num : usize,
    from : usize,
    to : usize
}

pub struct Crates {
    stacks : Vec<Vec<char>>
}

impl Move {
    pub fn from_line(line : &String) -> Option<Move> {
        let mut parts = line.split_whitespace();
        let num_s = parts.nth(1)?;
        let from_s = parts.nth(1)?;
        let to_s = parts.nth(1)?;
        let num = num_s.parse::<usize>().ok()?;
        let from = from_s.parse::<usize>().ok()?;
        let to = to_s.parse::<usize>().ok()?;
        Some(
            Move { num : num, from : from, to : to }
        )
    }
}

impl Crates {
    pub fn from_lines(lines : &Vec<&String>) -> Option<Crates> {
        let last_line = lines.first()?;
        let size = (last_line.len()+1) / 4;
        let mut crates = Crates::empty(size);
        for line in lines.iter().rev().skip(1) {
            crates.add_line(line);
        }
        Some(crates)
    }

    fn empty(size : usize) -> Crates {
        let mut init = Crates { stacks: Vec::new() };
        for _ in 0..size {
            init.stacks.push(Vec::new());
        }
        init
    }

    fn add_line(&mut self, line : &str) {
        for (i,c) in line.char_indices() {
            if i % 4 == 1 && c != ' ' {
                let s_in = i / 4;
                self.stacks[s_in].push(c);
            }
        }
    }

    pub fn simulate(&mut self, moves : &Vec<Move>, multi : bool) {
        for to_move in moves {
            self.do_move(to_move, multi);
        }
    }

    fn do_move(&mut self, to_move : &Move, multi : bool) {
        let mut temp : Vec<char> = Vec::new();
        for _ in 0..to_move.num {
            if let Some(top) = self.pop_top(to_move.from - 1) {
                temp.push(top);
            }
        }
        if multi {
            temp.reverse();
        }
        for ch in temp {
            self.push_top(to_move.to - 1, ch)
        }
    }

    fn pop_top(&mut self, from : usize) -> Option<char> {
        if let Some(stack) = self.stacks.get_mut(from) {
            stack.pop()
        } else {
            None
        }
    }

    fn push_top(&mut self, to : usize, ch : char) {
        if let Some(stack) = self.stacks.get_mut(to) {
            stack.push(ch);
        }
    }


    pub fn tops(&self) -> String {
        self.stacks.iter().map(
            |stack| stack.last().unwrap_or(&' ')
        ).collect()
    }
}