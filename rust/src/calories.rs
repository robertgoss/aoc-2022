struct Elf {
    calories : Vec<usize>
}

pub struct Elfs {
    elfs : Vec<Elf>
}

impl Elf {
    fn from_lines(lines : &[String]) -> Elf {
        let calories = lines.iter().filter_map(
            |str| str.parse::<usize>().ok()
        ).collect();
        Elf { calories : calories }
    }

    fn total(&self) -> usize {
        self.calories.iter().sum()
    }
}

impl Elfs {
    pub fn from_lines(lines : &Vec<String>) -> Elfs {
        let elfs : Vec<Elf> = lines.split(
       |str| str == "" 
        ).map(
            |nums| Elf::from_lines(nums)
        ).collect();
        Elfs { elfs : elfs }
    }

    pub fn best_elf(&self) -> usize {
        self.elfs.iter().map(
            |elf| elf.total()
        ).max().unwrap()
    }

    pub fn best_elfs(&self, count : usize) -> usize {
        let mut totals : Vec<usize> = self.elfs.iter().map(
            |elf| elf.total()
        ).collect();
        totals.sort();
        totals.iter().rev().take(count).sum()
    }
}