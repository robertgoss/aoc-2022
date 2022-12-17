use std::cmp::max;

#[derive(Debug)]
enum Shape {
    Horizontal,
    Cross,
    Corner,
    Vertical,
    Square
}

pub struct Game {
    heights : [usize; 7]
}

pub struct Jets {
    jets_r : Vec<bool>,
    index : usize
}

impl Game {
    pub fn new() -> Game {
        Game { heights : [0; 7] }
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
        println!("Simulating {:?}", shape);
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
        self.add_heights(shape, x, y);
        println!("Heights now {:?}", self.heights);
    }

    pub fn height(&self) -> usize {
        *self.heights.iter().max().unwrap_or(&0)
    }

    fn add_heights(&mut self, shape : &Shape, x :usize , y : usize) {
        for local_x in 0..shape.width() {
            let new_y = shape.y_over(local_x) + y - 1;
            self.heights[x+local_x] = max(self.heights[x+local_x], new_y);
        }
    }

    fn collide(&self, shape : &Shape, x :usize , y : usize) -> bool {
        (0..shape.width()).any(
            |local_x| {
                let block_y = shape.y_under(local_x) + y;
                let game_y =  self.heights[x+local_x];
                block_y <= game_y
            }
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

    fn y_under(&self, local_x : usize) -> usize {
        match self {
            Shape::Cross => {
                if local_x == 1 {0} else {1}
            },
            _ => 0
        }
    }

    fn y_over(&self, local_x : usize) -> usize {
        match self {
            Shape::Horizontal => 1,
            Shape::Square => 2,
            Shape::Vertical => 4,
            Shape::Corner => {
                if local_x == 2 {3} else {1}
            },
            Shape::Cross => {
                if local_x == 1 {3} else {2}
            }
        }
    }
}

