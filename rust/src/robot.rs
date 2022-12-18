use std::{cmp::{min, max}, collections::HashMap};

pub struct Blueprint {
    ore_ore : usize,
    clay_ore : usize,
    obsidean_ore : usize,
    obsidean_clay : usize,
    geode_ore : usize,
    geode_obsidean : usize
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct State {
    ore_robots : usize,
    clay_robots : usize,
    obsidean_robots : usize,
    geode_robots : usize,
    ore : usize,
    clay : usize,
    obsidean : usize,
    geodes : usize
}

impl Blueprint {
    pub fn from_line(line : &str) -> Option<Blueprint> {
        let numbers : Vec<usize> = line.split(" ").filter_map(
            |part| part.parse::<usize>().ok()
        ).collect();
        let ore_ore = *numbers.get(0)?;
        let clay_ore = *numbers.get(1)?;
        let obsidean_ore = *numbers.get(2)?;
        let obsidean_clay = *numbers.get(3)?;
        let geode_ore = *numbers.get(4)?;
        let geode_obsidean = *numbers.get(5)?;
        Some(Blueprint { 
            ore_ore : ore_ore, 
            clay_ore : clay_ore, 
            obsidean_ore : obsidean_ore, 
            obsidean_clay : obsidean_clay,
            geode_ore : geode_ore,
            geode_obsidean : geode_obsidean
        })
    }

    pub fn max_geodes(&self, time : usize) -> usize {
        let initial_state = State {
            ore_robots : 1,
            clay_robots : 0,
            obsidean_robots : 0,
            geode_robots : 0,
            ore : 0,
            clay : 0,
            obsidean : 0,
            geodes : 0
        };
        self.max_geodes_state(time, &initial_state, &mut HashMap::new())
    }

    fn max_geodes_state(
        &self, 
        time : usize, 
        state : &State,
        cache : &mut HashMap<(usize, State), usize>
    ) -> usize {
        if time == 0 {
            return state.geodes;
        }
        let key = (time, state.clone());
        if let Some(res) = cache.get(&key) {
            return *res;
        }
        let mut best_count = 0;
        // Do all combinations of robot making we can do - there are dependencies!
        for ore_made_state in self.state_create_ore(state) {
            for clay_made_state in self.state_create_clay(&ore_made_state) {
                for obsidean_made_state in self.state_create_obsidean(&clay_made_state) {
                    for mut geode_made_state in self.state_create_geode(&obsidean_made_state) {
                        // Gather 
                        geode_made_state.ore += state.ore_robots;
                        geode_made_state.clay += state.clay_robots;
                        geode_made_state.obsidean += state.obsidean_robots;
                        geode_made_state.geodes += state.geode_robots;
                        best_count = max(
                            best_count,
                            self.max_geodes_state(time-1, &geode_made_state, cache)
                        );

                    }
                }
            }
        }
        cache.insert(key, best_count);
        println!("Cache size: {}", cache.len());
        best_count
    }

    fn state_create_ore(&self, state : &State) -> Vec<State> {
        let max_ore_robots = state.ore / self.ore_ore;
        let mut new_states = Vec::new();
        for i in 0..=max_ore_robots {
            let mut new_state = state.clone();
            new_state.ore_robots += i;
            new_state.ore -= i*self.ore_ore;
            new_states.push(new_state);
        }
        new_states
    }

    fn state_create_clay(&self, state : &State) -> Vec<State> {
        let max_clay_robots = state.ore / self.clay_ore;
        let mut new_states = Vec::new();
        for i in 0..=max_clay_robots {
            let mut new_state = state.clone();
            new_state.clay_robots += i;
            new_state.ore -= i*self.clay_ore;
            new_states.push(new_state);
        }
        new_states
    }

    fn state_create_obsidean(&self, state : &State) -> Vec<State> {
        let max_obsidean_robots_ore = state.ore / self.obsidean_ore;
        let max_obsidean_robots_clay = state.clay / self.obsidean_clay;
        let max_obsidean_robots = min(max_obsidean_robots_ore, max_obsidean_robots_clay);
        let mut new_states = Vec::new();
        for i in 0..=max_obsidean_robots {
            let mut new_state = state.clone();
            new_state.obsidean_robots += i;
            new_state.ore -= i*self.obsidean_ore;
            new_state.clay -= i*self.obsidean_clay;
            new_states.push(new_state);
        }
        new_states
    }

    fn state_create_geode(&self, state : &State) -> Vec<State> {
        let max_geode_robots_ore = state.ore / self.geode_ore;
        let max_geode_robots_obsidean = state.clay / self.geode_obsidean;
        let max_geode_robots = min(max_geode_robots_ore, max_geode_robots_obsidean);
        let mut new_states = Vec::new();
        for i in 0..=max_geode_robots {
            let mut new_state = state.clone();
            new_state.geode_robots += i;
            new_state.ore -= i*self.geode_ore;
            new_state.obsidean -= i*self.geode_obsidean;
            new_states.push(new_state);
        }
        new_states
    }
}