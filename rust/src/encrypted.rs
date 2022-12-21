// Ideally we would use a linked list for the re-ordering
pub struct File {
    vals : Vec<(usize, i64)>
}

impl File {
    pub fn make_file(init_vals : &Vec<i64>) -> File {
        let mut file = File { 
            vals : init_vals.clone().into_iter().enumerate().collect() 
        };
        for id in 0..init_vals.len() {
            file.shift_id(id);
        }
        file
    }

    pub fn make_file_key(init_vals : &Vec<i64>, key : i64, rounds : usize) -> File {
        let mut file = File { 
            vals : init_vals.iter().map(
                |val| val * key
            ).enumerate().collect() 
        };
        for _ in 0..rounds {
            for id in 0..init_vals.len() {
                file.shift_id(id);
            }
        }
        file
    }

    fn find_id(&self, id : usize) -> usize {
        self.vals.iter().enumerate().find(
            |(_,v)| v.0==id
        ).unwrap().0
    }

    fn find_val(&self, val : i64) -> usize {
        self.vals.iter().enumerate().find(
            |(_,v)| v.1==val
        ).unwrap().0
    }

    fn shift_id(&mut self, id : usize) {
        let base_index = self.find_id(id);
        let val = self.vals[base_index].1;
        self.remove(base_index);
        let new_index = (base_index as i64 + val).rem_euclid(self.vals.len() as i64);
        self.insert(new_index as usize, (id, val));
    }

    fn remove(&mut self, index : usize) {
        self.vals.remove(index);
    }

    fn insert(&mut self, index : usize, val : (usize, i64)) {
        self.vals.insert(index, val);
    }

    pub fn data(&self, index : usize) -> i64 {
        let base = self.find_val(0);
        let i = (base + index) % self.vals.len();
        self.vals[i].1
    }
}