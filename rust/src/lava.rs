use std::collections::HashMap;
use std::{collections::HashSet, hash::Hash};
use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::algo::kosaraju_scc;

#[derive(PartialEq, Eq, Hash, Clone)]
enum Dir {
    X,
    Y, 
    Z
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Surface {
    x : i64,
    y : i64, 
    z : i64,
    dir : Dir
}

pub struct Droplets {
    pts : HashSet<(i64, i64, i64)>
}

fn parse_pt(line : &str) -> Option<(i64, i64, i64)> {
    let parts : Vec<&str> = line.split(",").collect();
    let x = parts.get(0).and_then(|s| s.parse::<i64>().ok())?;
    let y = parts.get(1).and_then(|s| s.parse::<i64>().ok())?;
    let z = parts.get(2).and_then(|s| s.parse::<i64>().ok())?;
    Some((x,y,z))
} 

fn insert_rem<T>(set : &mut HashSet<T>, t : T)
 where T : Eq, T: Hash
{
    if set.contains(&t) {
        set.remove(&t);
    } else {
        set.insert(t);
    }
}

impl Droplets {
    pub fn from_lines(lines : &Vec<String>) -> Droplets {
        let pts = lines.iter().filter_map(
            |line| parse_pt(line)
        ).collect();
        Droplets { pts: pts }
    }

    pub fn surface_area(&self) -> usize {
        self.surfaces().len()
    }

    pub fn outside_surface_area(&self) -> usize {
        let comps = self.surface_components();
        let top_comp = comps.iter().max_by_key(|comp| comp.iter().map(|s| s.z).max()).unwrap();
        top_comp.len()
    }

    fn surface_components(&self) -> Vec<Vec<Surface>> {
        let mut surface_map : HashMap<NodeIndex, Surface> = HashMap::new();
        let mut surface_map_inv : HashMap<Surface, NodeIndex> = HashMap::new();
        let mut adjacent_graph : UnGraph<(), ()>= UnGraph::new_undirected();
        for surface in self.surfaces() {
            let index = adjacent_graph.add_node(());
            surface_map.insert(index, surface.clone());
            surface_map_inv.insert(surface, index);
        }
        for (index, surface) in surface_map.iter() {
            for adjacent in surface.adjacent() {
                if let Some(index_ad) = surface_map_inv.get(&adjacent) {
                    adjacent_graph.add_edge(*index, *index_ad, ());
                }
            }
        }
        let comps = kosaraju_scc(&adjacent_graph);
        let s : Vec<usize> = comps.iter().map(|comp| comp.len()).collect();
        println!("Sizes {:?}",s);
        comps.iter().map(
            |comp| comp.iter().map(
                |index| surface_map.get(index).unwrap().clone()
            ).collect()
        ).collect()
    }

    fn surfaces(&self) -> HashSet<Surface> {
        let mut surfaces = HashSet::new();
        for pt in self.pts.iter() {
            let x = pt.0;
            let y = pt.1;
            let z = pt.2;
            insert_rem(&mut surfaces, Surface {x:x, y:y, z:z, dir:Dir::X});
            insert_rem(&mut surfaces, Surface {x:x-1, y:y, z:z, dir:Dir::X});
            insert_rem(&mut surfaces, Surface {x:x, y:y, z:z, dir:Dir::Y});
            insert_rem(&mut surfaces, Surface {x:x, y:y-1, z:z, dir:Dir::Y});
            insert_rem(&mut surfaces, Surface {x:x, y:y, z:z, dir:Dir::Z});
            insert_rem(&mut surfaces, Surface {x:x, y:y, z:z-1, dir:Dir::Z});
        }
        surfaces
    }
}

impl Surface {
    fn adjacent(&self) -> [Surface; 12] {
        let x = self.x;
        let y = self.y;
        let z = self.z;
        match self.dir {
            Dir::X => {
                [
                    Surface { x : x, y : y, z : z, dir : Dir::Y },
                    Surface { x : x, y : y, z : z, dir : Dir::Z },
                    Surface { x : x, y : y-1, z : z, dir : Dir::Y },
                    Surface { x : x, y : y, z : z-1, dir : Dir::Z },
                    Surface { x : x+1, y : y, z : z, dir : Dir::Y },
                    Surface { x : x+1, y : y, z : z, dir : Dir::Z },
                    Surface { x : x+1, y : y-1, z : z, dir : Dir::Y },
                    Surface { x : x+1, y : y, z : z-1, dir : Dir::Z },
                    Surface { x: x, y : y+1, z : z, dir : Dir::X},
                    Surface { x: x, y : y-1, z : z, dir : Dir::X},
                    Surface { x: x, y : y, z : z+1, dir : Dir::X},
                    Surface { x: x, y : y, z : z-1, dir : Dir::X}
                ]
            },
            Dir::Y => {
                [
                    Surface { x : x, y : y, z : z, dir : Dir::X },
                    Surface { x : x, y : y, z : z, dir : Dir::Z },
                    Surface { x : x-1, y : y, z : z, dir : Dir::X },
                    Surface { x : x, y : y, z : z-1, dir : Dir::Z },
                    Surface { x : x, y : y+1, z : z, dir : Dir::X },
                    Surface { x : x, y : y+1, z : z, dir : Dir::Z },
                    Surface { x : x-1, y : y+1, z : z, dir : Dir::X },
                    Surface { x : x, y : y+1, z : z-1, dir : Dir::Z },
                    Surface { x: x+1, y : y, z : z, dir : Dir::Y},
                    Surface { x: x-1, y : y, z : z, dir : Dir::Y},
                    Surface { x: x, y : y, z : z+1, dir : Dir::Y},
                    Surface { x: x, y : y, z : z-1, dir : Dir::Y}
                ]
            },
            Dir::Z => {
                [
                    Surface { x : x, y : y, z : z, dir : Dir::X },
                    Surface { x : x, y : y, z : z, dir : Dir::Y },
                    Surface { x : x-1, y : y, z : z, dir : Dir::X },
                    Surface { x : x, y : y-1, z : z, dir : Dir::Y },
                    Surface { x : x, y : y, z : z+1, dir : Dir::X },
                    Surface { x : x, y : y, z : z+1, dir : Dir::Y },
                    Surface { x : x-1, y : y, z : z+1, dir : Dir::X },
                    Surface { x : x, y : y-1, z : z+1, dir : Dir::Y },
                    Surface { x: x, y : y+1, z : z, dir : Dir::Z},
                    Surface { x: x, y : y-1, z : z, dir : Dir::Z},
                    Surface { x: x+1, y : y, z : z, dir : Dir::Z},
                    Surface { x: x-1, y : y, z : z, dir : Dir::Z}
                ]
            }
        }
    }
}
