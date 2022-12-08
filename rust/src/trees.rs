use std::collections::{HashSet, HashMap};

pub struct Forest {
    hieghts : Vec<Vec<u32>>
}


impl Forest {
    pub fn from_lines(lines : &Vec<String>) -> Forest {
        let hieghts : Vec<Vec<u32>> = lines.iter().map(
            |line| line.chars().filter_map(
                |ch| ch.to_digit(10)
            ).collect()
        ).collect();
        Forest { hieghts: hieghts }
    }

    pub fn count_visible(&self) -> usize {
        let width = self.hieghts[0].len();
        let height = self.hieghts.len();
        let mut visible = HashSet::new();
        for i in 0..height {
            self.visible_along(
                (0..width).map(|j| (i,j)),
                &mut visible
            );
            self.visible_along(
                (0..width).rev().map(|j| (i,j)),
                &mut visible
            );
        }
        for j in 0..width {
            self.visible_along(
                (0..height).map(|i| (i,j)),
                &mut visible
            );
            self.visible_along(
                (0..height).rev().map(|i| (i,j)),
                &mut visible
            );
        }
        visible.len()
    }

    pub fn scenic_max(&self) -> usize {
        let width = self.hieghts[0].len();
        let height = self.hieghts.len();
        let mut scores = HashMap::new();
        for i in 0..height {
            self.scenic_along(
                (0..width).map(|j| (i,j)),
                &mut scores
            );
            self.scenic_along(
                (0..width).rev().map(|j| (i,j)),
                &mut scores
            );
        }
        for j in 0..width {
            self.scenic_along(
                (0..height).map(|i| (i,j)),
                &mut scores
            );
            self.scenic_along(
                (0..height).rev().map(|i| (i,j)),
                &mut scores
            );
        }
        *scores.values().max().unwrap()
    }

    fn visible_along<I>(&self, mut iter : I, visible : &mut HashSet<(usize, usize)>)
        where I : Iterator<Item = (usize, usize)>
    {
        let first = iter.next().unwrap();
        visible.insert(first);
        let mut best = self.get(&first);
        for index in iter {
            let val = self.get(&index);
            if val > best {
                best = val;
                visible.insert(index);
            }
        }
    }

    fn scenic_along<I>(&self, iter : I, scores : &mut HashMap<(usize, usize), usize>)
        where I : Iterator<Item = (usize, usize)>
    {
        // Distance to the last maxima seen under that height
        let mut prev_dist: [Option<usize>; 11] = [None; 11];
        // Put a pretend 10 hieght tree before the start
        prev_dist[10] = Some(0);
        for index in iter {
            let val = self.get(&index) as usize;
            // Find the last tree higher than this one
            let dist = (val..11).filter_map(
                |v| prev_dist[v]
            ).next().unwrap();
            // Update score
            *scores.entry(index).or_insert(1) *= dist;
            // Update distances - clear anything below val
            for v in 0..val {
                prev_dist[v] = None
            }
            // Add marker for tree
            prev_dist[val] = Some(1);
            // Increment all greater than val
            for v in val+1..11 {
                if let Some(dist) = prev_dist[v].as_mut() {
                    *dist += 1;
                }
            }
        }
    }

    fn get(&self, index : &(usize, usize))  -> u32 {
        self.hieghts[index.0][index.1]
    }
}