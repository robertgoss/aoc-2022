enum Instruction {
    Noop,
    AddX(i64)
}

pub struct CPU {
    code : Vec<Instruction>
}

impl Instruction {
    fn from_line(line : &str) -> Option<Instruction> {
        if line == "noop" {
            Some(Instruction::Noop)
        } else {
            let (ins, num_s) = line.split_once(" ")?;
            if ins == "addx" {
                let num = num_s.parse::<i64>().ok()?;
                Some(Instruction::AddX(num))
            } else {
                None
            }
        }
    }
}

impl CPU {
    pub fn from_lines(lines : &Vec<String>) -> CPU {
        CPU {
            code : lines.iter().filter_map(
                |line| Instruction::from_line(line)
            ).collect()
        }
    }

    fn x_vals(&self) -> Vec<i64> {
        let mut x : i64 = 1;
        let mut xs : Vec<i64> = Vec::new();
        for inst in self.code.iter() {
            match inst {
                Instruction::Noop => {
                    xs.push(x);
                },
                Instruction::AddX(val) => {
                    xs.push(x);
                    xs.push(x);
                    x += val;
                }
            }
        }
        xs
    }

    pub fn signals(&self) -> Vec<i64> {
        let xs = self.x_vals();
        xs.iter().enumerate().map(
            |(cyc, x)| (cyc as i64+1) * x
        ).collect()
    }

    pub fn display(&self) -> String {
        let xs = self.x_vals();
        let lines : Vec<String> = xs.chunks(40).map(
            |x_line| self.display_line(x_line)
        ).collect();
        lines.join("\n")
    }

    fn display_line(&self, x_line : &[i64]) -> String {
        String::from_iter(
            x_line.iter().enumerate().map(
                |(pos, &x), | if (pos as i64-x).abs() <= 1 {'#'} else {'.'}
            )
        )
    }
}