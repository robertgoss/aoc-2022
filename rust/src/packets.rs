use std::cmp::Ordering;

use pest::Parser;

#[derive(Parser)]
#[grammar = "packet.pest"]
struct PacketParser;

#[derive(Clone, Debug)]
pub enum Packet {
    Num(usize),
    List(Vec<Packet>)
}

pub struct PacketPair {
    packets : (Packet, Packet)
}

use pest::iterators::Pair;

fn parse_packet(pair: Pair<Rule>) -> Packet {
    match pair.as_rule() {
        Rule::num => Packet::Num(pair.as_str().parse().unwrap()),
        Rule::list => {
            Packet::List(pair.into_inner().map(parse_packet).collect())
        },
        _ => unreachable!()
    }
}

impl Packet {
    pub fn from_line(line : &str) -> Option<Packet> {
        let packet = PacketParser::parse(Rule::packet, line).ok()?.next().unwrap();
        Some(parse_packet(packet))
    } 

    pub fn dividers() -> (Packet, Packet) {
        (Packet::List(vec!(Packet::Num(2))), Packet::List(vec!(Packet::Num(6))))
    }

    fn compare(&self, other : &Packet) -> Ordering {
        match (self, other) {
            (Packet::Num(val1), Packet::Num(val2)) => val1.cmp(val2),
            (Packet::Num(_), _) => {
                Packet::List(vec!(self.clone())).compare(other)    
            },
            (_, Packet::Num(_)) => {
                self.compare(&Packet::List(vec!(other.clone())))
            },
            (Packet::List(l1), Packet::List(l2)) => {
                for (p1,p2) in l1.iter().zip(l2.iter()) {
                    let ord = p1.compare(p2);
                    if ord.is_ne() {
                        return ord
                    }
                }
                l1.len().cmp(&l2.len())
            }
        }
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.compare(other).is_eq()
    }
}

impl Eq for Packet {

}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.compare(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.compare(other)
    }
}

impl PacketPair {
    pub fn from_string(string : &str) -> Option<PacketPair> {
        let (fst,snd) = string.split_once("\n")?;
        let fst_pack = Packet::from_line(fst)?;
        let snd_pack = Packet::from_line(snd)?;
        Some(PacketPair { packets : (fst_pack,snd_pack) })
    }

    pub fn ordered(&self) -> bool {
        self.packets.0.compare(&self.packets.1) == Ordering::Less
    }
}