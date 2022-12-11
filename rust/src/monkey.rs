enum Operation {
    Square,
    Add(usize),
    Mult(usize)
}

struct Monkey {
    items : Vec<usize>,
    operation : Operation,
    test_div : usize,
    next_true : usize,
    next_false : usize,
    activity : usize
}

impl Operation {
    fn from_string(string : &str) -> Option<Operation> {
        if string == "new = old * old" {
            Some(Operation::Square)
        } else {
            if let Some(val_s) = string.strip_prefix("new = old * ") {
                let val = val_s.parse::<usize>().ok()?;
                Some(Operation::Mult(val))
            } else {
                if let Some(val_s) = string.strip_prefix("new = old + ") {
                    let val = val_s.parse::<usize>().ok()?;
                    Some(Operation::Add(val))
                } else {
                    None
                }
            }
        }
    }

    fn op(&self, item : &usize) -> usize {
        match self {
            Operation::Square => item * item,
            Operation::Add(val) => item + val,
            Operation::Mult(val) => item * val
        }
    } 
}

impl Monkey {
    fn from_string(string : &str) -> Option<Monkey> {
        let lines : Vec<&str> = string.lines().collect();
        let start_str = *lines.get(1)?;
        let items_s = start_str.strip_prefix("  Starting items: ")?;
        let items = items_s.split(", ").filter_map(
            |part| part.parse::<usize>().ok()
        ).collect();
        let operation_str = *lines.get(2)?;
        let operation_s = operation_str.strip_prefix("  Operation: ")?;
        let operation = Operation::from_string(operation_s)?;
        let div_str = *lines.get(3)?;
        let div_s = div_str.strip_prefix("  Test: divisible by ")?;
        let div = div_s.parse::<usize>().ok()?;
        let true_str = *lines.get(4)?;
        let true_s = true_str.strip_prefix("    If true: throw to monkey ")?;
        let true_v = true_s.parse::<usize>().ok()?;
        let false_str = *lines.get(5)?;
        let false_s = false_str.strip_prefix("    If false: throw to monkey ")?;
        let false_v = false_s.parse::<usize>().ok()?;
        Some(
            Monkey {
                items: items, 
                operation: operation, 
                test_div: div, 
                next_true: true_v, 
                next_false: false_v,
                activity : 0
            }
        )

    }

    fn inspect(&mut self, lcm : usize, div3: bool) -> (Vec<usize>, Vec<usize>) {
        let mut next_true = Vec::new();
        let mut next_false = Vec::new();
        for item in self.items.iter() {
            let mut new_item = self.operation.op(item);
            if div3 {
                new_item = new_item / 3;
            }
            new_item = new_item % lcm;
            if new_item % self.test_div == 0 {
                next_true.push(new_item);
            } else {
                next_false.push(new_item);
            }
        }
        self.activity += self.items.len();
        self.items.clear();
        (next_true, next_false)
    }

    fn add(&mut self, mut new : Vec<usize>) {
        self.items.append(&mut new);
    }
}

pub struct Monkeys {
    monkeys : Vec<Monkey>
}


impl Monkeys {
    pub fn from_string(string : &str) -> Monkeys {
        let monkeys = string.split("\n\n").filter_map(
            |string| Monkey::from_string(string)
        ).collect();
        Monkeys { monkeys: monkeys }
    }

    pub fn simulate(&mut self, steps : usize, div3: bool) {
        let lcm = self.lcm();
        for _ in 0..steps {
            for i in 0..self.monkeys.len() {
                self.simulate_monkey(i, lcm, div3);
            }
        };
    }

    fn lcm(&self) -> usize {
        let mut lcm : usize = 1;
        for monkey in self.monkeys.iter() {
            lcm = num::integer::lcm(lcm, monkey.test_div);
        }
        lcm
    }

    pub fn monkey_buisness(&self) -> usize {
        let mut activity : Vec<usize> = self.monkeys.iter().map(
            |monkey| monkey.activity
        ).collect();
        activity.sort();
        activity.reverse();
        activity[0] * activity[1]
    }

    fn simulate_monkey(&mut self, i : usize, lcm : usize, div3: bool) {
        let (next_true, next_false) = self.monkeys[i].inspect(lcm, div3);
        let next_true_i = self.monkeys[i].next_true;
        let next_false_i = self.monkeys[i].next_false;
        self.monkeys[next_true_i].add(next_true);
        self.monkeys[next_false_i].add(next_false);
    }


}