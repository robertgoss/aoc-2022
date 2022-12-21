use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone, Copy, Debug)]
enum Dir {
    N,
    E,
    S,
    W
}

pub struct Field {
    elves : HashSet::<(i64,i64)>
}

impl Field {
    pub fn from_lines(lines : &Vec<String>) -> Field {
        let mut elfs = HashSet::new();
        for (j, line) in lines.iter().enumerate() {
            for (i, ch) in line.chars().enumerate() {
                if ch == '#' {
                    elfs.insert((i as i64,j as i64));
                }
            }
        }
        Field { elves: elfs }
    }

    pub fn empty_ground(&self) -> usize {
        let max_x = self.elves.iter().map(
            |(x,_)| x
        ).max().unwrap() + 1;
        let min_x = self.elves.iter().map(
            |(x,_)| x
        ).min().unwrap();
        let max_y = self.elves.iter().map(
            |(_,y)| y
        ).max().unwrap() + 1;
        let min_y = self.elves.iter().map(
            |(_,y)| y
        ).min().unwrap();
        let diff_x = (max_x - min_x) as usize;
        let diff_y = (max_y - min_y) as usize;
        let area = diff_x * diff_y;
        area - self.elves.len()
    }

    pub fn simulate(&mut self, rounds : usize) {
        let order = Dir::N.order();
        for dir in order.iter().cycle().take(rounds) {
            self.simulate_one(dir);
        }
    }

    pub fn simulate_till_still(&mut self) -> usize {
        let order = Dir::N.order();
        for (i,dir) in order.iter().cycle().enumerate() {
            let any_move = self.simulate_one(dir);
            if !any_move {
                return i+1;
            }
        }
        0
    }

    fn simulate_one(&mut self, base_dir : &Dir) -> bool {
        let mut proposed_directions : HashMap<(i64, i64), Dir> = HashMap::new();
        let mut proposed_pos_count : HashMap<(i64, i64), usize> = HashMap::new();
        let mut any_move = false;
        for pos in self.elves.iter() {
            if let Some(new_dir) = self.propose(pos, base_dir) {
                proposed_directions.insert(*pos, new_dir);
                let new_pos = new_dir.step(pos);
                *proposed_pos_count.entry(new_pos).or_insert(0) += 1;
            }
        }
        // Now have all of them can move elfs
        let mut new_elfs = HashSet::new();
        for pos in self.elves.iter() {
            if let Some(new_dir) = proposed_directions.get(pos) {
                let new_pos = new_dir.step(pos);
                if *proposed_pos_count.get(&new_pos).unwrap() == 1 {
                    new_elfs.insert(new_pos);
                    any_move = true;
                } else {
                    new_elfs.insert(*pos);
                }
            } else {
                new_elfs.insert(*pos);
            }
        }
        self.elves = new_elfs;
        any_move
    }

    fn propose(&self, pos : &(i64,i64), last : &Dir) -> Option<Dir> {
        if self.all_clear(pos) {
            return None;
        }
        for dir in last.order() {
            let clear = dir.check(pos).iter().all(
                |p| self.clear(p)
            );
            if clear {
                return Some(dir);
            }
        }
        None
    }

    fn all_clear(&self, (x,y) : &(i64,i64)) -> bool {
        self.clear(&(x+1, y-1)) &&
        self.clear(&(x+1, *y)) &&
        self.clear(&(x+1, y+1)) &&
        self.clear(&(*x, y-1)) &&
        self.clear(&(*x, y+1)) &&
        self.clear(&(x-1, y-1)) &&
        self.clear(&(x-1, *y)) &&
        self.clear(&(x-1, y+1))
    }

    fn clear(&self, pos : &(i64,i64)) -> bool {
        !self.elves.contains(pos)
    }
}

impl Dir {
    fn step(&self, (x,y) : &(i64,i64)) -> (i64, i64) {
        match self {
            Dir::N => (*x, y-1),
            Dir::E => (x+1, *y),
            Dir::S => (*x, y+1),
            Dir::W => (x-1, *y)
        }
    }

    fn order(&self) -> [Dir; 4] {
        match self {
            Dir::N => [Dir::N, Dir::S, Dir::W, Dir::E],
            Dir::S => [Dir::S, Dir::W, Dir::E, Dir::N],
            Dir::W => [Dir::W, Dir::E, Dir::N, Dir::S],
            Dir::E => [Dir::E, Dir::N, Dir::S, Dir::W]
        }
    }

    fn check(&self, (x,y) : &(i64, i64)) -> [(i64, i64); 3] {
        match self {
            Dir::N => [(*x, y-1), (x-1, y-1), (x+1, y-1)],
            Dir::E => [(x+1, *y), (x+1, y-1), (x+1, y+1)],
            Dir::S => [(*x, y+1), (x-1, y+1), (x+1, y+1)],
            Dir::W => [(x-1, *y), (x-1, y-1), (x-1, y+1)]
        }
    }
}