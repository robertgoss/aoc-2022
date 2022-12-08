use std::collections::HashSet;
use itertools::repeat_n;

enum Direction {
    Right,
    Left,
    Up,
    Down
}

struct Rope {
    head : (i64, i64),
    tail : Vec<(i64, i64)>
}

pub struct Directions {
    direction : Vec<(Direction, usize)>
}

fn from_line(line : &str) -> Option<(Direction, usize)> {
    let (dir_s, dist_s) = line.split_once(' ')?;
    let dist = dist_s.parse::<usize>().ok()?;
    match dir_s {
        "R" => Some((Direction::Right, dist)),
        "L" => Some((Direction::Left, dist)),
        "U" => Some((Direction::Up, dist)),
        "D" => Some((Direction::Down, dist)),
        _ => None
    }
}

impl Directions {
    pub fn from_lines(lines : &Vec<String>) -> Directions {
        let dirs = lines.iter().filter_map(
            |line| from_line(line)
        ).collect();
        Directions { direction: dirs }
    }

    pub fn simulate(&self, len : usize) -> HashSet<(i64,i64)> {
        let mut pos : HashSet<(i64,i64)> = HashSet::new();
        pos.insert((0,0));
        let mut rope = Rope::new(len);
        for (dir, dist) in self.direction.iter() {
            for _ in 0..*dist {
                rope.update(dir);
                pos.insert(rope.end());
            }
        }
        pos
    }
}

fn sign(i : i64) -> i64 {
    if i > 0 { 
        1 
    } else { 
        if i < 0 {-1} else {0}
    }
}

impl Rope {

    fn new(len : usize) -> Rope {
        Rope {
            head : (0,0),
            tail : Vec::from_iter(repeat_n((0,0), len - 1))
        }
    } 

    fn end(&self) -> (i64, i64) {
        *self.tail.last().unwrap()
    }

    fn update(&mut self, dir : &Direction) {
        match *dir {
            Direction::Right => self.head.1 += 1,
            Direction::Left => self.head.1 -= 1,
            Direction::Up => self.head.0 += 1,
            Direction::Down => self.head.0 -= 1,
        };
        let mut prev = self.head;
        for tail in self.tail.iter_mut() {
            let diff_x = prev.0 - tail.0;
            let diff_y = prev.1 - tail.1;
            if diff_x.abs() > 1 || diff_y.abs() > 1 {
                tail.0 += sign(diff_x);
                tail.1 += sign(diff_y);
            }
            prev = *tail;
        }

    }
}