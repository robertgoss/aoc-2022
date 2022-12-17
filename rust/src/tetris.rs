use std::collections::HashSet;

#[derive(Debug)]
enum Shape {
    Horizontal,
    Cross,
    Corner,
    Vertical,
    Square
}

pub struct Game {
    rocks : HashSet<(usize, usize)>
}

pub struct Jets {
    jets_r : Vec<bool>,
    index : usize
}

impl Game {
    pub fn new() -> Game {
        Game { rocks : HashSet::from_iter(
            (0..7).map(|i| (i as usize ,0))
        ) }
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
        self.add_shape(shape, x, y);
        //self.print();
    }

    pub fn simulate_cycle(&mut self, jets : &mut Jets) -> usize {
        let shapes = [
            Shape::Horizontal,
            Shape::Cross,
            Shape::Corner,
            Shape::Vertical,
            Shape::Square
        ];
        for (i, shape) in shapes.iter().cycle().enumerate() {
            if i > 0 && self.flat_top() {
                return i;
            }
            self.simulate_shape(&shape, jets);
            self.print();
        };
        0
    }

    pub fn height(&self) -> usize {
        *self.rocks.iter().map(
            |(_,y)| y
        ).max().unwrap_or(&0)
    }

    fn flat_top(&self) -> bool {
        let y = self.height();
        (0..7).all(
            |x| self.rocks.contains(&(x, y))
        )
    }

    fn add_shape(&mut self, shape : &Shape, x :usize , y : usize) {
        println!("Adding at {} {}", x, y);
        for (pt_x, pt_y) in shape.points() {
            self.rocks.insert((pt_x + x, pt_y + y));
        }
    }

    fn collide(&self, shape : &Shape, x :usize , y : usize) -> bool {
        shape.points().into_iter().any(
            |(pt_x, pt_y)| self.rocks.contains(&(pt_x+x, pt_y+y))
        )
    }

    fn print(&self) {
        for i in (0..=self.height()).rev() {
            let line : String = (0..7).map(
                |j| if self.rocks.contains(&(j, i)) {'#'} else {'.'} 
            ).collect();
            println!("|{}|", line);
        }
        println!("{:?}", self.rocks);
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

