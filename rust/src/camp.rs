struct Assignment {
    min : usize,
    max : usize
}

pub struct AssignmentPair {
    elf1 : Assignment,
    elf2 : Assignment
}

impl Assignment {
    fn from_string(string : &str) -> Option<Assignment> {
        let parts = string.split_once("-")?;
        let min = parts.0.parse::<usize>().ok()?;
        let max = parts.1.parse::<usize>().ok()?;
        Some(Assignment {min : min, max : max} )
    }

    fn contains(&self, other : &Assignment) -> bool {
        self.min <= other.min && self.max >= other.max
    }

    fn overlap(&self, other : &Assignment) -> bool {
        self.min <= other.max && self.max >= other.min
    }
}

impl AssignmentPair {
    pub fn from_line(line : &String) -> Option<AssignmentPair> {
        let parts = line.split_once(",")?;
        let elf1 = Assignment::from_string(parts.0)?;
        let elf2 = Assignment::from_string(parts.1)?;
        Some(
            AssignmentPair { elf1: elf1, elf2: elf2 }
        )
    }

    pub fn contains(&self) -> bool {
        self.elf1.contains(&self.elf2) || self.elf2.contains(&self.elf1)
    }

    pub fn overlap(&self) -> bool {
        self.elf1.overlap(&self.elf2)
    }
}