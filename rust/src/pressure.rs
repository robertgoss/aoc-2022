use std::{collections::{HashMap, BTreeSet}, cmp::max};
use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::algo::dijkstra;


pub struct Network {
    names : HashMap<String, NodeIndex<u32>>,
    graph : UnGraph::<u32,()>,
    min_distance : HashMap<(NodeIndex, NodeIndex), usize>
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
            graph : UnGraph::new_undirected(),
            min_distance : HashMap::new()
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
        for node in network.graph.node_indices() {
            let distances = dijkstra(
                &network.graph, 
                node, 
                None, 
                |_| 1
            );
            for (other, dist) in distances.iter() {
                network.min_distance.insert((node, *other), *dist as usize);
            }
        }
        network
    }

    fn start_node(&self) -> NodeIndex<u32> {
        *self.names.get("AA").unwrap()
    }

    fn max_pressure(
        &self,
        node : NodeIndex, 
        left : usize, 
        remaining : &BTreeSet<NodeIndex>,
        cache : &mut HashMap<(NodeIndex, usize, BTreeSet<NodeIndex>), usize>
    ) -> usize {
        if let Some(res) = cache.get(&(node, left, remaining.clone())) {
            return *res;
        }
        if left == 0 {
            return 0;
        }
        let mut max_left = self.graph.neighbors(node).map(
            |next| self.max_pressure(next, left-1, remaining, cache)
        ).max().unwrap();
        let flow : usize = *self.graph.node_weight(node).unwrap() as usize;
        if !remaining.contains(&node) && flow > 0 {
            let mut switch_on = remaining.clone();
            switch_on.insert(node);
            let max_sw = self.max_pressure(node, left-1, &switch_on, cache)
             + (left-1) * flow;
            if max_sw > max_left {
                max_left = max_sw;
            }
        }
        cache.insert((node, left, remaining.clone()), max_left);
        max_left
    }

    fn max_pressure_dual(
        &self,
        nodes : (NodeIndex,NodeIndex), 
        left : usize, 
        remaining : &BTreeSet<NodeIndex>,
        cache : &mut HashMap<((NodeIndex,NodeIndex), usize, BTreeSet<NodeIndex>), usize>
    ) -> usize {
        if let Some(res) = cache.get(&(nodes, left, remaining.clone())) {
            return *res;
        }
        if left == 0 {
            return 0;
        }
        let flow0 = *self.graph.node_weight(nodes.0).unwrap() as usize;
        let flow1 = *self.graph.node_weight(nodes.1).unwrap() as usize;
        let can_open0 = !remaining.contains(&nodes.0) && flow0 > 0;
        let can_open1 = !remaining.contains(&nodes.1) && flow1 > 0;
        let mut best_pressure : usize = 0;
        // Both move
        for next0 in self.graph.node_indices() {
            for next1 in self.graph.node_indices() {
                let dist0 = *self.min_distance.get(&(nodes.0, next0)).unwrap();
                let dist1 = *self.min_distance.get(&(nodes.1, next1)).unwrap();
                if dist0 == dist1 && dist0 > 0 && left >= dist0 {
                    let pos_flow0 = *self.graph.node_weight(next0).unwrap() as usize;
                    let pos_flow1 = *self.graph.node_weight(next1).unwrap() as usize;
                    // At leas one is an unseen flow node
                    let possible0 = !remaining.contains(&next0) && pos_flow0 > 0;
                    let possible1 = !remaining.contains(&next1) && pos_flow1 > 0;
                    if possible0 || possible1 {
                        let move_pressure = self.max_pressure_dual((next0, next1), left-dist0, remaining, cache);
                        best_pressure = max(move_pressure, best_pressure);
                    }
                }
            }
        }
        // 0 turn 1 move
        if can_open0 {
            let mut switch_on = remaining.clone();
            switch_on.insert(nodes.0);
            for next1 in self.graph.neighbors(nodes.1) {
                let move_pressure = self.max_pressure_dual((nodes.0, next1), left-1, &switch_on, cache);
                let new_pressure = move_pressure + flow0 * (left-1);
                best_pressure = max(new_pressure, best_pressure);
            }
        }

        // 1 turn 0 move
        if can_open1 {
            let mut switch_on = remaining.clone();
            switch_on.insert(nodes.1);
            for next0 in self.graph.neighbors(nodes.0) {
                let move_pressure = self.max_pressure_dual((next0, nodes.1), left-1, &switch_on, cache);
                let new_pressure = move_pressure + flow1 * (left-1);
                best_pressure = max(new_pressure, best_pressure);
            }
        }

        // Both turn
        if can_open0 && can_open1 && nodes.0 != nodes.1{
            let mut switch_on = remaining.clone();
            switch_on.insert(nodes.0);
            switch_on.insert(nodes.1);
            let move_pressure = self.max_pressure_dual(nodes, left-1,& switch_on, cache);
            let new_pressure = move_pressure + (flow0 + flow1) * (left-1);
            best_pressure = max(new_pressure, best_pressure);
        }
        
        cache.insert((nodes, left, remaining.clone()), best_pressure);
        best_pressure
    }

    pub fn maximum_pressure(&self) -> usize {
        self.max_pressure(self.start_node(), 30, &BTreeSet::new(), &mut HashMap::new())
    }

    pub fn maximum_pressure_dual(&self) -> usize {
        let mut cache = HashMap::new();
        self.max_pressure_dual((self.start_node(),self.start_node()), 26, &BTreeSet::new(), &mut cache)
    }
}