use ndarray::prelude::*;
use std::cmp;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::env;
use std::fs;

/*--- Day 16: Proboscidea Volcanium ---
The sensors have led you to the origin of the distress signal: yet another handheld device, just
like the one the Elves gave you. However, you don't see any Elves around; instead, the device is
surrounded by elephants! They must have gotten lost in these tunnels, and one of the elephants
apparently figured out how to turn on the distress signal.

The ground rumbles again, much stronger this time. What kind of cave is this, exactly? You scan the
cave with your handheld device; it reports mostly igneous rock, some ash, pockets of pressurized
gas, magma... this isn't just a cave, it's a volcano!

You need to get the elephants out of here, quickly. Your device estimates that you have 30 minutes
before the volcano erupts, so you don't have time to go back out the way you came in.

You scan the cave for other options and discover a network of pipes and pressure-release valves.
You aren't sure how such a system got into a volcano, but you don't have time to complain; your
device produces a report (your puzzle input) of each valve's flow rate if it were opened (in
pressure per minute) and the tunnels you could use to move between the valves.

There's even a valve in the room you and the elephants are currently standing in labeled AA. You
estimate it will take you one minute to open a single valve and one minute to follow any tunnel
from one valve to another. What is the most pressure you could release?

For example, suppose you had the following scan output:

Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II

All of the valves begin closed. You start at valve AA, but it must be damaged or jammed or
something: its flow rate is 0, so there's no point in opening it. However, you could spend one
minute moving to valve BB and another minute opening it; doing so would release pressure during the
remaining 28 minutes at a flow rate of 13, a total eventual pressure release of 28 * 13 = 364.
Then, you could spend your third minute moving to valve CC and your fourth minute opening it,
providing an additional 26 minutes of eventual pressure release at a flow rate of 2, or 52 total
pressure released by valve CC.

Making your way through the tunnels like this, you could probably open many or all of the valves by
the time 30 minutes have elapsed. However, you need to release as much pressure as possible, so
you'll need to be methodical. Instead, consider this approach:

== Minute 1 ==
No valves are open.
You move to valve DD.

== Minute 2 ==
No valves are open.
You open valve DD.

== Minute 3 ==
Valve DD is open, releasing 20 pressure.
You move to valve CC.

== Minute 4 ==
Valve DD is open, releasing 20 pressure.
You move to valve BB.

== Minute 5 ==
Valve DD is open, releasing 20 pressure.
You open valve BB.

== Minute 6 ==
Valves BB and DD are open, releasing 33 pressure.
You move to valve AA.

== Minute 7 ==
Valves BB and DD are open, releasing 33 pressure.
You move to valve II.

== Minute 8 ==
Valves BB and DD are open, releasing 33 pressure.
You move to valve JJ.

== Minute 9 ==
Valves BB and DD are open, releasing 33 pressure.
You open valve JJ.

== Minute 10 ==
Valves BB, DD, and JJ are open, releasing 54 pressure.
You move to valve II.

== Minute 11 ==
Valves BB, DD, and JJ are open, releasing 54 pressure.
You move to valve AA.

== Minute 12 ==
Valves BB, DD, and JJ are open, releasing 54 pressure.
You move to valve DD.

== Minute 13 ==
Valves BB, DD, and JJ are open, releasing 54 pressure.
You move to valve EE.

== Minute 14 ==
Valves BB, DD, and JJ are open, releasing 54 pressure.
You move to valve FF.

== Minute 15 ==
Valves BB, DD, and JJ are open, releasing 54 pressure.
You move to valve GG.

== Minute 16 ==
Valves BB, DD, and JJ are open, releasing 54 pressure.
You move to valve HH.

== Minute 17 ==
Valves BB, DD, and JJ are open, releasing 54 pressure.
You open valve HH.

== Minute 18 ==
Valves BB, DD, HH, and JJ are open, releasing 76 pressure.
You move to valve GG.

== Minute 19 ==
Valves BB, DD, HH, and JJ are open, releasing 76 pressure.
You move to valve FF.

== Minute 20 ==
Valves BB, DD, HH, and JJ are open, releasing 76 pressure.
You move to valve EE.

== Minute 21 ==
Valves BB, DD, HH, and JJ are open, releasing 76 pressure.
You open valve EE.

== Minute 22 ==
Valves BB, DD, EE, HH, and JJ are open, releasing 79 pressure.
You move to valve DD.

== Minute 23 ==
Valves BB, DD, EE, HH, and JJ are open, releasing 79 pressure.
You move to valve CC.

== Minute 24 ==
Valves BB, DD, EE, HH, and JJ are open, releasing 79 pressure.
You open valve CC.

== Minute 25 ==
Valves BB, CC, DD, EE, HH, and JJ are open, releasing 81 pressure.

== Minute 26 ==
Valves BB, CC, DD, EE, HH, and JJ are open, releasing 81 pressure.

== Minute 27 ==
Valves BB, CC, DD, EE, HH, and JJ are open, releasing 81 pressure.

== Minute 28 ==
Valves BB, CC, DD, EE, HH, and JJ are open, releasing 81 pressure.

== Minute 29 ==
Valves BB, CC, DD, EE, HH, and JJ are open, releasing 81 pressure.

== Minute 30 ==
Valves BB, CC, DD, EE, HH, and JJ are open, releasing 81 pressure.
This approach lets you release the most pressure possible in 30 minutes with this valve layout, 1651.

Work out the steps to release the most pressure in 30 minutes. What is the most pressure you can
release?


*/

#[derive(Debug)]
struct World {
    valves: Vec<Valve>,
    start_valve: usize, // index we start at
    timeout: usize,

    nexts: Array2<usize>,

    // order to take neighbors during traversal
    // this is a plan, its not updatd during traversal
    permutation_number: usize,
    permutations: Vec<Vec<usize>>,
}

#[derive(Debug)]
struct Valve {
    name: String,
    rate: i32,
    neighbors: Vec<usize>, // indices into valves array
    is_open: bool,
}

fn get_path(next: &Array2<usize>, u_in: usize, v: usize) -> VecDeque<usize> {
    if next[[u, v]] == usize::MAX {
        println!("No path!");
        return VecDeque::new();
    }

    let mut path = VecDeque::new();
    path.push_back(u);
    while u != v {
        u = next[[u, v]];
        path.push_back(u);
    }
    return path;
}

fn compute_next(valves: Vec<Valve>) -> Array2<usize> {
    // https://en.wikipedia.org/wiki/Floyd%E2%80%93Warshall_algorithm
    let n = valves.len();
    let mut dist = Array::zeros((n, n));
    let mut next = Array::zeros((n, n));
    dist.fill(usize::MAX);
    next.fill(usize::MAX); // max is a sentinal for no path

    for (u, valve) in valves.iter().enumerate() {
        for nbr in valve.neighbors {
            // can go directly, dist = 1, next equals direct
            dist[[u, nbr]] = 1;
            next[[u, nbr]] = nbr;
        }
    }

    for (u, valve) in valves.iter().enumerate() {
        for nbr in valve.neighbors {
            // already here, dist is zero
            dist[[u, u]] = 0;
            next[[u, u]] = u;
        }
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                // if going through k makes the distance shorter, then take it
                if dist[[i, j]] > dist[[i, k]] + dist[[k, j]] {
                    dist[[i, j]] = dist[[i, k]] + dist[[k, j]];
                    next[[i, j]] = next[[i, k]];
                }
            }
        }
    }

    return next;
}

impl std::fmt::Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for v in self.valves.iter() {
            write!(
                f,
                "Valve {} has flow rate={}; tunnels lead to ",
                v.name, v.rate
            )
            .unwrap();

            if v.neighbors.len() > 1 {
                write!(f, "valves ",).unwrap();
            } else {
                write!(f, "valve ").unwrap();
            }

            for n in v.neighbors.iter() {
                write!(f, "{}, ", self.valves[*n].name).unwrap();
            }

            write!(f, "\n").unwrap();
        }
        return Ok(());
    }
}

fn help_compute_permutations(arr: &mut Vec<usize>, k: usize, permutations: &mut Vec<Vec<usize>>) {
    for i in k..arr.len() {
        arr.swap(i, k);
        help_compute_permutations(arr, k + 1, permutations);
        arr.swap(k, i);
    }

    if k == arr.len() - 1 {
        permutations.push(arr.clone());
    }
}

impl World {
    fn parse(contents: &str) -> World {
        // read lines and compute bounds
        let mut start_name: String = Default::default();
        let mut valves: Vec<Valve> = vec![];
        for line in contents.lines() {
            let name = line[6..8].to_string();
            if start_name == "" {
                start_name = name.to_string();
            }
            let (rate_str, tunnels_str) = line[23..].split_once(";").unwrap();
            let rate: i32 = rate_str.parse().unwrap();

            let mut neighbors_str = Default::default();
            match tunnels_str.split_once("valves ") {
                Some(pair) => {
                    neighbors_str = pair.1;
                }
                None => {}
            }
            match tunnels_str.split_once("valve ") {
                Some(pair) => {
                    neighbors_str = pair.1;
                }
                None => {}
            }

            let nbrs = neighbors_str
                .split(",")
                .map(|n| n.trim().to_string())
                .collect::<Vec<String>>();

            let mut nbr_indices: Vec<usize> = vec![];
            for n in nbrs {
                let mut has_match = false;
                for (i, v) in valves.iter().enumerate() {
                    if v.name == n {
                        has_match = true;
                        nbr_indices.push(i);
                        break;
                    }
                }

                if !has_match {
                    // go ahead and create dummy neighbor
                    nbr_indices.push(valves.len());
                    valves.push(Valve {
                        name: n,
                        rate: -1,
                        neighbors: Default::default(),
                        is_open: false,
                    })
                }
            }

            // add to list of valves if it doesn't exist yet
            let mut has_match = false;
            for v in valves.iter_mut() {
                if v.name == name {
                    let mut n_perms = nbr_indices.len();
                    if rate > 0 {
                        n_perms += 1;
                    }

                    has_match = true;
                    v.rate = rate;
                    v.neighbors = nbr_indices.clone();
                    break;
                }
            }

            if !has_match {
                let mut n_perms = nbr_indices.len();
                if rate > 0 {
                    n_perms += 1;
                }

                valves.push(Valve {
                    name: name,
                    rate: rate,
                    neighbors: nbr_indices.clone(),
                    is_open: false,
                })
            }
        }

        let start_index = valves.iter().position(|v| v.name == start_name).unwrap();

        // permute order non-zero indices
        let mut nonzero_count: usize = 0;
        let mut to_visit: Vec<usize> = Default::default();
        for (i, v) in valves.iter().enumerate() {
            if v.rate > 0 {
                to_visit.push(i);
            }
        }

        // compute next graph
        let nexts = compute_next(valves);

        let mut permutations: Vec<Vec<usize>> = vec![];
        help_compute_permutations(&mut to_visit, 0, &mut permutations);

        return World {
            valves: valves,
            start_valve: start_index,
            timeout: 30,
            permutation_number: 0,
            permutations: permutations,
            nexts: nexts,
        };
    }

    fn next_route(&mut self) -> Option<usize> {
        // set all turn_index to 0
        // and tick forward by one permutation

        self.permutation_number += 1;
        if self.permutation_number >= self.permutations.len() {
            return None;
        }

        // close all valves
        for v in self.valves.iter_mut() {
            v.is_open = false;
        }

        println!("Plan: {:?}", self.permutations[self.permutation_number]);
        let mut targets = self.permutations[self.permutation_number].clone();

        // run simulation following current permutation rules
        let mut route: VecDeque<usize> = VecDeque::new();
        let mut pos = self.start_valve;
        let mut tick = 1;
        let mut pressure_released = 0;
        for target in 0..self.timeout {
            // update released pressure
            for v in self.valves.iter() {
                if v.is_open {
                    pressure_released += v.rate;
                    println!("{} open, releasing {}", v.name, v.rate);
                }
            }

            // figure out route
            let mut can_move = true;
            if route.len() == 0 {
                if !self.valves[pos].is_open && self.valves[pos].rate > 0 {
                    // open it
                    self.valves[pos].is_open = true;
                    can_move = false;
                }
                // don't need to open or already open
                // new route
                let next_target = targets.pop().unwrap_or_default();
                route = get_path(&self.nexts, pos, next_target);
            }

            println!("t={} at={:?}", tick, self.valves[pos]);
            if can_move && route.len() > 0 {
                // move toward next target
                match route.pop_front() {
                    Some(new_pos) => {
                        println!(
                            "Moving {} to {}",
                            self.valves[pos].name, self.valves[new_pos].name
                        );
                        pos = new_pos;
                    }
                    None => {
                        println!("Idling");
                    }
                }
            }

            // move forward in time
            tick += 1;
        }
        return Some(pressure_released as usize);
    }
}

fn part1(contents: &str) {
    let mut world = World::parse(contents);
    println!("{:?}", world);

    let mut max_released = 0;
    for i in 0..10000 {
        match world.next_route() {
            Some(released) => {
                println!("Route: {}, Released: {}", i, released);
                if released > max_released {
                    max_released = released;
                }
            }
            _ => {
                println!("Exhausted ordering");
                break;
            }
        }
    }

    println!("Max released: {}", max_released);
    // find fastest path
}

fn part2(contents: &str) {}

fn main() {
    let fname = env::args().nth(1).expect("Should pass 1 filename arg");
    let contents = fs::read_to_string(fname).expect("Should have been able to read the file");

    part1(&contents);
    //part2(&contents);
}
