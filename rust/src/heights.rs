use std::collections::BTreeSet;

use itertools::{repeat_n, Itertools};

pub struct HeightMap {
    grid : Vec<Vec<u8>>,
    start : (usize, usize),
    end : (usize, usize),
    w : usize,
    h : usize
}

fn parse_row(i : usize, line : &String) -> (Vec<u8>, Option<(usize, usize)>, Option<(usize, usize)>) {
    let mut row = Vec::new();
    let mut start = None;
    let mut end = None;
    for (j, ch) in line.char_indices() {
        if ch == 'S' {
            start = Some((i, j));
            row.push('a' as u8);
        } else {
            if ch == 'E' {
                end = Some((i, j));
                row.push('z' as u8);
            } else {
                row.push(ch as u8);
            }
        }
    }
    (row, start, end)
}

fn from_grid(
    grid : &Vec<Vec<Option<u16>>>,
    pos : &(usize, usize)
) -> Option<u16> {
    grid.get(pos.0).and_then(
        |row| row.get(pos.1)
    ).and_then(
        |val| *val
    )
}

impl HeightMap {
    pub fn from_lines(lines : &Vec<String>) -> HeightMap {
        let mut rows = Vec::new();
        let mut start = None;
        let mut end = None;
        for (i, line) in lines.iter().enumerate() {
            let (row, start_r, end_r) = parse_row(i,line);
            start = start.or(start_r);
            end = end.or(end_r);
            rows.push(row);
        }
        let w = rows[0].len();
        let h = rows.len();
        HeightMap {
            grid : rows,
            w : w,
            h : h,
            start : start.unwrap(),
            end : end.unwrap()
        }
    }

    pub fn distance(&self) -> usize {
        let distance_grid = self.distance_grid();
        distance_grid[self.start.0][self.start.1].unwrap() as usize
    }

    pub fn distance_start(&self) -> usize {
        let distance_grid = self.distance_grid();
        (0..self.h).cartesian_product(0..self.w).filter(
            |(i,j)| self.grid[*i][*j] == ('a' as u8)
        ).filter_map(
            |(i,j)| distance_grid[i][j]
        ).min().unwrap() as usize
    }

    fn distance_grid(&self) -> Vec<Vec<Option<u16>>> {
        let mut distance_grid : Vec<Vec<Option<u16>>> = Vec::new();
        for _ in 0..self.h {
            distance_grid.push(
                Vec::from_iter(repeat_n(None, self.w))
            );
        }
        distance_grid[self.end.0][self.end.1] = Some(0);
        let mut to_check = BTreeSet::from_iter(
            self.neighbours(&self.end).into_iter()
        );
        while let Some(pos) = to_check.pop_first() {
            if let Some(val) = self.best_distance(&distance_grid, &pos) {
                let old_val = from_grid(&distance_grid, &pos);
                let replace = old_val.map(
                    |old_dist| old_dist > val
                ).unwrap_or(true);
                if replace {
                    distance_grid[pos.0][pos.1] = Some(val);
                    for n in self.neighbours(&pos) {
                        to_check.insert(n);
                    }
                }
            }
        }
        distance_grid
    }

    fn best_distance(
        &self,
        grid : &Vec<Vec<Option<u16>>>,
        pos : &(usize, usize)
    ) -> Option<u16> {
        self.neighbours(pos).into_iter().filter(
            |p| self.valid_move(pos, p)
        ).filter_map(
            |p| from_grid(grid, &p)
        ).min().map(
            |res| res + 1
        )
    }

    fn valid_move(&self, from : &(usize, usize), to : &(usize, usize)) -> bool{
        let val_start = self.grid[from.0][from.1];
        let val_end = self.grid[to.0][to.1];
        val_start +1 >= val_end
    }

    fn neighbours(&self, pos : &(usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbours = Vec::new();
        if pos.0 > 0 {
            neighbours.push((pos.0 - 1, pos.1));
        } 
        if pos.0 < (self.h-1) {
            neighbours.push((pos.0 + 1, pos.1));
        } 
        if pos.1 > 0 {
            neighbours.push((pos.0, pos.1 - 1));
        } 
        if pos.1 < (self.w-1) {
            neighbours.push((pos.0, pos.1 + 1));
        } 

        neighbours
    }
}