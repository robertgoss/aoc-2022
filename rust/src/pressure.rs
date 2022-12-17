use std::collections::{HashMap, BTreeSet};
use petgraph::graph::{NodeIndex, UnGraph};


pub struct Network {
    names : HashMap<String, NodeIndex<u32>>,
    graph : UnGraph::<u32,()>
}

fn parse_line(line : &String) -> Option<(String, u32, Vec<String>)> {
    let (valve_str, tunnel_str) = line.split_once("; ")?;
    let (name_s, valve_s) = valve_str.split_once(" has flow rate=")?;
    let name = name_s.strip_prefix("Valve ")?;
    let flow = valve_s.parse::<u32>().ok()?;
    let tunnel_s = tunnel_str.strip_prefix("tunnels lead to valves ").or(
        tunnel_str.strip_prefix("tunnel leads to valve ")
    )?;
    let tunnels = tunnel_s.split(", ").map(|s| s.to_string()).collect();
    Some((name.to_string(), flow, tunnels))
}

impl Network {
    pub fn from_lines(lines : &Vec<String>) -> Network {
        let mut network = Network { 
            names : HashMap::new(),
            graph : UnGraph::new_undirected()
        };
        let mut tunnels : Vec<(String, String)> = Vec::new();
        for line in lines.iter() {
            if let Some((name, flow, next)) = parse_line(line) {
                let id = network.graph.add_node(flow);
                network.names.insert(name.clone(), id);
                for other in next {
                    tunnels.push((name.clone(),other));
                }
            }
        }
        for (from, to) in tunnels {
            let id1 = network.names.get(&from).unwrap();
            let id2 = network.names.get(&to).unwrap();
            network.graph.update_edge(*id1, *id2, ());
        }
        network
    }

    fn start_node(&self) -> NodeIndex<u32> {
        *self.names.get("AA").unwrap()
    }

    pub fn maximum_pressure(&self) -> usize {
        0
    }
}