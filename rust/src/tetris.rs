use std::collections::{HashSet, HashMap};
use std::cmp::max;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Shape {
    Horizontal,
    Cross,
    Corner,
    Vertical,
    Square
}

pub struct Game {
    rocks : HashSet<(usize, usize)>,
    heights : [usize; 7],
    min_height : usize
}

pub struct Jets {
    jets_r : Vec<bool>,
    index : usize
}

impl Game {
    pub fn new() -> Game {
        Game { 
            rocks : HashSet::from_iter(
              (0..7).map(|i| (i as usize ,0))
            ),
            heights : [0; 7],
            min_height : 0
        }
    }

    pub fn simulate(&mut self, count : usize, jets : &mut Jets) {
        let shapes = [
            Shape::Horizontal,
            Shape::Cross,
            Shape::Corner,
            Shape::Vertical,
            Shape::Square
        ];
        for shape in shapes.iter().cycle().take(count) {
            self.simulate_shape(shape, jets);
        }
    }

    fn simulate_shape(&mut self, shape : &Shape, jets : &mut Jets) {
        let mut y = self.height() + 4;
        let mut x : usize = 2;
        let mut fall = false;
        let mut at_base = false;
        while !at_base {
            if fall {
                if !self.collide(shape, x, y-1) {
                    y = y-1;
                } else {
                    at_base = true;
                }
                fall = false;
            } else {
                let right = jets.next_right();
                let new_x = if right {
                    if x + shape.width() < 7 {
                        x+1
                    } else {
                        x
                    }
                } else {
                    if x > 0 {
                        x-1
                    } else {
                        x
                    }
                };
                if !self.collide(shape, new_x, y) {
                    x = new_x;
                }
                fall = true;
            }
        }
        self.add_shape(shape, x, y);
        //self.print();
    }

    pub fn simulate_long(&mut self, long_count : usize, jets : &mut Jets) -> usize {
        // Simulate till we hit a cycle
        let (cycle_start, cycle_period, heights_at) = 
          self.simulate_cycle(jets);
        assert!(long_count > cycle_period);
        // Work out how many cycles and what remainder we need
        let cycle_num = (long_count - cycle_start) / cycle_period;
        let cycle_rem = (long_count - cycle_start) % cycle_period;
        let delta_height = heights_at[cycle_start + cycle_period] - heights_at[cycle_start];
        let cycle_hieght = (cycle_num-1) * delta_height;
        heights_at[cycle_start + cycle_period + cycle_rem] + cycle_hieght
    }

    fn simulate_cycle(&mut self, jets : &mut Jets) -> (usize, usize, Vec<usize>) {
        let shapes = [
            Shape::Horizontal,
            Shape::Cross,
            Shape::Corner,
            Shape::Vertical,
            Shape::Square
        ];
        let mut heights = Vec::new();
        let mut top_repeat : HashMap<(Shape, [usize; 7], usize), usize> = HashMap::new();
        let mut goal = None;
        let mut cycle_start = 0;
        let mut cycle_period = 0;
        for (i, shape) in shapes.iter().cycle().enumerate() {
            heights.push(self.height());
            if goal.contains(&i) {
                break;
            }
            if goal.is_none() {
                if let Some(prev) = top_repeat.get(&(*shape, self.heights, jets.index)) {
                    cycle_start = *prev;
                    cycle_period = i - *prev;
                    goal = Some(i + cycle_period);
                }
                top_repeat.insert((*shape, self.heights, jets.index), i);
            }
            self.simulate_shape(&shape, jets);
        };
        (cycle_start, cycle_period, heights)
    }

    pub fn height(&self) -> usize {
        *self.heights.iter().max().unwrap_or(&0) + self.min_height
    }

    fn add_shape(&mut self, shape : &Shape, x :usize , y : usize) {
        for (pt_x, pt_y) in shape.points() {
            self.rocks.insert((pt_x + x, pt_y + y));
            self.heights[pt_x + x] = max(self.heights[pt_x+x], pt_y+y - self.min_height);
        }
        let new_min = *self.heights.iter().min().unwrap_or(&0);
        if new_min != 0 {
            self.min_height += new_min;
            self.heights.iter_mut().for_each(
                |h| *h -= new_min
            );
            self.rocks.drain_filter(
                |(_, y)| *y < new_min
            );
        }
    }

    fn collide(&self, shape : &Shape, x :usize , y : usize) -> bool {
        shape.points().into_iter().any(
            |(pt_x, pt_y)| self.rocks.contains(&(pt_x+x, pt_y+y))
        )
    }
}

impl Jets {
    fn next_right(&mut self) -> bool {
        let res = self.jets_r[self.index];
        // Cycle
        self.index += 1;
        if self.index >= self.jets_r.len() {
            self.index -= self.jets_r.len();
        };
        res
    }

    pub fn from_line(line : &str) -> Jets {
        let jets = line.chars().map(
            |ch| ch == '>'
        ).collect();
        Jets { jets_r : jets, index : 0 }
    } 
}

impl Shape {
    fn width(&self) -> usize {
        match self {
            Shape::Horizontal => 4,
            Shape::Square => 2,
            Shape::Vertical => 1,
            Shape::Corner => 3,
            Shape::Cross => 3
        }
    }

    fn points(&self) -> Vec<(usize, usize)> {
        match self {
            Shape::Horizontal => vec!((0,0), (1,0), (2,0), (3,0)),
            Shape::Square => vec!((0,0), (1,0), (0,1), (1,1)),
            Shape::Vertical => vec!((0,0), (0,1), (0,2), (0,3)),
            Shape::Corner => vec!((0,0), (1,0), (2,0), (2,1), (2,2)),
            Shape::Cross => vec!((1,0), (0,1), (2,1), (1,2), (1,1)),
        }
    }
}

