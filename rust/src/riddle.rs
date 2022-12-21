use std::collections::HashMap;
use itertools::Itertools;
use num::Rational64;

#[derive(Clone, Debug)]
pub struct Poly {
    coeff : Vec<Rational64>
}

enum Op {
    Num(i64),
    Add(String,String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String)
}

pub struct Riddle {
    ops : HashMap<String, Op>
}

impl Op {
    fn from_string(string : &str) -> Option<Op> {
        if let Ok(num) = string.parse::<i64>() {
            Some(Op::Num(num))
        } else {
            if let Some((a,b)) = string.split_once(" + ") {
                Some(Op::Add(a.to_string(), b.to_string()))
            } else {
                if let Some((a,b)) = string.split_once(" - ") {
                    Some(Op::Sub(a.to_string(), b.to_string()))
                } else {
                    if let Some((a,b)) = string.split_once(" * ") {
                        Some(Op::Mul(a.to_string(), b.to_string()))
                    } else {
                        if let Some((a,b)) = string.split_once(" / ") {
                            Some(Op::Div(a.to_string(), b.to_string()))
                        } else {
                            None
                        }
                    }
                }
            }
        }
    }
}

fn parse_line(line : &str) -> Option<(String, Op)> {
    let (name, op_s) = line.split_once(": ")?;
    let op = Op::from_string(op_s)?;
    Some((name.to_string(), op))
}

impl Riddle {
    pub fn from_lines(lines : &Vec<String>) -> Riddle {
        let ops = lines.iter().filter_map(
            |line| parse_line(line)
        ).collect();
        Riddle { ops: ops }
    }

    pub fn solve(&self, name : &str) -> Option<i64> {
        self.solve_cached(name, &mut HashMap::new())
    }

    fn solve_cached(&self, name : &str, cache : &mut HashMap<String, i64>) -> Option<i64> {
        if let Some(res) = cache.get(name) {
            return Some(*res);
        }
        let op = self.ops.get(name)?;
        let res = match op {
            Op::Num(num) => Some(*num),
            Op::Add(a,b) => {
                let a_res = self.solve_cached(&a, cache)?;
                let b_res = self.solve_cached(&b, cache)?;
                Some(a_res + b_res)
            },
            Op::Sub(a,b) => {
                let a_res = self.solve_cached(&a, cache)?;
                let b_res = self.solve_cached(&b, cache)?;
                Some(a_res - b_res)
            },
            Op::Mul(a,b) => {
                let a_res = self.solve_cached(&a, cache)?;
                let b_res = self.solve_cached(&b, cache)?;
                Some(a_res * b_res)
            },
            Op::Div(a,b) => {
                let a_res = self.solve_cached(&a, cache)?;
                let b_res = self.solve_cached(&b, cache)?;
                Some(a_res / b_res)
            }
        }?;
        cache.insert(name.to_string(), res);
        Some(res)
    }

    pub fn root_eqn(&self, var : &str) -> Poly {
        let mut cache = HashMap::new();
        let op = self.ops.get("root").unwrap();
        match op {
            Op::Num(_) => unreachable!(),
            Op::Add(a,b) => {
                let a_pol = self.solve_poly_cached(a, var, &mut cache);
                let b_pol = self.solve_poly_cached(b, var, &mut cache);
                a_pol.sub(&b_pol)
            },
            Op::Sub(a,b) => {
                let a_pol = self.solve_poly_cached(a, var, &mut cache);
                let b_pol = self.solve_poly_cached(b, var, &mut cache);
                a_pol.sub(&b_pol)
            },
            Op::Mul(a,b) => {
                let a_pol = self.solve_poly_cached(a, var, &mut cache);
                let b_pol = self.solve_poly_cached(b, var, &mut cache);
                a_pol.sub(&b_pol)
            },
            Op::Div(a,b) => {
                let a_pol = self.solve_poly_cached(a, var, &mut cache);
                let b_pol = self.solve_poly_cached(b, var, &mut cache);
                a_pol.sub(&b_pol)
            }
        }
    }

    fn solve_poly_cached(&self, name : &str, var : &str, cache : &mut HashMap<String, Poly>) -> Poly {
        if let Some(p) = cache.get(name) {
            return p.clone();
        }
        if name==var {
            return Poly::id()
        }
        
        let op = self.ops.get(name).unwrap();
        let res = match op {
            Op::Num(num) => Poly::num(*num),
            Op::Add(a, b) => {
                let p_a = self.solve_poly_cached(&a, var, cache);
                let p_b = self.solve_poly_cached(&b, var, cache);
                p_a.add(&p_b)
            },
            Op::Sub(a, b) => {
                let p_a = self.solve_poly_cached(&a, var, cache);
                let p_b = self.solve_poly_cached(&b, var, cache);
                p_a.sub(&p_b)
            },
            Op::Mul(a, b) => {
                let p_a = self.solve_poly_cached(&a, var, cache);
                let p_b = self.solve_poly_cached(&b, var, cache);
                p_a.mul(&p_b)
            },
            Op::Div(a, b) => {
                let p_a = self.solve_poly_cached(&a, var, cache);
                let p_b = self.solve_poly_cached(&b, var, cache);
                p_a.div(&p_b)
            }
        };
        cache.insert(name.to_string(), res.clone());
        res
    }
}

impl Poly {
    fn num(num : i64) -> Poly {
        Poly { coeff: vec!(num.into()) }
    }

    fn id() -> Poly {
        Poly { coeff: vec!(1.into(), 0.into()) }
    }

    fn add(&self, p : &Poly) -> Poly {
        let coeff = self.coeff.iter().rev().zip_longest(
            p.coeff.iter().rev()
        ).map(
            |v| match v {
                itertools::EitherOrBoth::Both(a, b) => a+b,
                itertools::EitherOrBoth::Left(a) => *a,
                itertools::EitherOrBoth::Right(a) => *a
            }
        ).rev().collect();
        Poly { coeff: coeff }
    }

    fn sub(&self, p : &Poly) -> Poly {
        let coeff = self.coeff.iter().rev().zip_longest(
            p.coeff.iter().rev()
        ).map(
            |v| match v {
                itertools::EitherOrBoth::Both(a, b) => a-b,
                itertools::EitherOrBoth::Left(a) => *a,
                itertools::EitherOrBoth::Right(a) => -a
            }
        ).rev().collect();
        Poly { coeff: coeff }
    }

    fn mul(&self, p : &Poly) -> Poly { 
        if p.coeff.len()!=1 {
            assert!(self.coeff.len()==1);
            p.mul(&self)
        } else {
            let scalar = *p.coeff.get(0).unwrap();
            let coeff = self.coeff.iter().map(
                |a| a*scalar
            ).collect();
            Poly { coeff : coeff }
        }
    }

    fn div(&self, p : &Poly) -> Poly { 
        assert!(p.coeff.len()==1);
        let scalar = *p.coeff.get(0).unwrap();
        let coeff = self.coeff.iter().map(
            |a| a/scalar
        ).collect();
        Poly { coeff : coeff }
    }

    pub fn solve(&self) -> i64 {
        assert!(self.coeff.len()==2);
        let a = self.coeff.get(0).unwrap();
        let b = self.coeff.get(1).unwrap();
        // at + b = 0   => t = -b / a
        let res = -b / a;
        assert!(*res.denom() == 1);
        *res.numer()
    }
}