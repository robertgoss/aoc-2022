use std::collections::HashSet;
use std::cmp::{min, max};

#[derive(Debug)]
pub struct Cave {
    max_y : Option<i64>,
    rock : HashSet<(i64,i64)>,
    sand : HashSet<(i64,i64)>
}

fn parse_coords(string : &str) -> Vec<(i64,i64)> {
    string.split(" -> ").filter_map(
        |part| {
            let (p1,p2) = part.split_once(",")?;
            let i1 = p1.parse::<i64>().ok()?;
            let i2 = p2.parse::<i64>().ok()?;
            Some((i1,i2))
        }
    ).collect()
}

impl Cave {
    pub fn from_lines(lines : &Vec<String>) -> Cave {
        let mut cave = Cave { max_y : None, rock : HashSet::new(), sand : HashSet::new() };
        for line in lines.iter() {
            cave.add_rock_row(&parse_coords(line));
        }
        cave.max_y = cave.rock.iter().map(
            |(_,y)| *y + 1
        ).max();
        cave
    }

    fn add_rock_row(&mut self, coords : &[(i64, i64)]) {
        let mut start = coords[0];
        for end in coords[1..].iter() {
            self.add_rock_line(&start, end);
            start = *end;
        }
    }

    fn add_rock_line(&mut self, start : &(i64,i64), end : &(i64, i64)) {
        if start.0 == end.0 {
            let min_y = min(start.1, end.1);
            let max_y = max(start.1, end.1);
            for i in min_y..=max_y {
                self.rock.insert((start.0, i));
            }
        } else {
            let min_x = min(start.0, end.0);
            let max_x = max(start.0, end.0);
            for i in min_x..=max_x {
                self.rock.insert((i, start.1));
            }
        }
    }

    fn simulate_sand(&self, start : &(i64,i64)) -> (i64,i64) {
        let mut curr = *start;
        while curr.1 < self.max_y.unwrap() {
            let next = self.step_sand(&curr);
            if next == curr {
                break;
            }
            curr = next;
        };
        curr
    }

    fn step_sand(&self, start : &(i64,i64)) -> (i64, i64) {
        // if below free go there
        let down = (start.0, start.1+1);
        if self.is_free(&down) {
            down
        } else {
            let down_left = (start.0-1, start.1+1);
            if self.is_free(&down_left) {
                down_left
            } else {
                let down_right = (start.0+1, start.1+1);
                if self.is_free(&down_right) {
                    down_right
                } else {
                    *start
                }
            }
        }
    }

    fn is_free(&self, pos : &(i64,i64)) -> bool {
        !( self.rock.contains(pos) || self.sand.contains(pos) )
    }

    pub fn pour_wall(&mut self) -> usize {
        let start = (500,0);
        let mut pos = self.simulate_sand(&start);
        while pos != start {
            pos = self.simulate_sand(&start);
            self.sand.insert(pos);
        }
        self.sand.len()
    }

    pub fn pour_void(&mut self) -> usize {
        let start = (500,0);
        let mut pos = self.simulate_sand(&start);
        while pos.1 <= self.max_y.unwrap() {
            self.sand.insert(pos);
            pos = self.simulate_sand(&start);
        }
        self.sand.len()
    }
}