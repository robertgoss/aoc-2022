use std::collections::HashSet;

#[derive(PartialEq,Eq,Hash,Debug,Copy,Clone)]
struct Item {
    id : char
}


pub struct Rucksack {
    comp1 : HashSet<Item>,
    comp2 : HashSet<Item>
}

pub struct Group {
    packs : Vec<Rucksack>
}

impl Rucksack {
    pub fn from_line(line : &String) -> Rucksack {
        let half = line.len() / 2;
        let comp1 = line.chars().take(half).map(
            |ch| Item{id : ch}
        ).collect();
        let comp2 = line.chars().skip(half).map(
            |ch| Item{id : ch}
        ).collect();
        Rucksack { comp1: comp1, comp2: comp2 }
    }

    fn union(&self) -> HashSet<Item> {
        self.comp1.union(&self.comp2).cloned().collect()
    }

    pub fn score(&self) -> usize {
        self.comp1.intersection(&self.comp2).next().unwrap().priority()
    }
}

impl Item {
    fn priority(&self) -> usize {
        let res = match self.id {
            'a'..='z' => (self.id as u8) - ('a' as u8) + 1,
            'A'..='Z' => (self.id as u8) - ('A' as u8) + 27,
            _ => 100
        };
        res as usize
    }
}

impl Group {
    pub fn from_lines(lines : [&String; 3]) -> Group {
        let packs = lines.iter().map(
            |line| Rucksack::from_line(*line)
        ).collect();
        Group { packs: packs }
    }

    pub fn score(&self) -> usize {
        let partial : HashSet<Item>= self.packs[0].union().intersection(
            &self.packs[1].union()
        ).cloned().collect();
        partial.intersection(&self.packs[2].union()).next().unwrap().priority()
    }
}