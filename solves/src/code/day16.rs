use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::time::Instant;

use graph::prelude::*;
use nom::branch::alt;
use nom::bytes::complete::{tag, take, take_till};
use nom::IResult;
use nom::multi::separated_list0;
use nom::sequence::preceded;
use priority_queue::PriorityQueue;

type Data = Rc<UndirectedCsrGraph<usize, usize>>;

pub fn run(data: Data) -> usize {
    let mut queue = PriorityQueue::new();
    let mut visits: Vec<HashSet<(usize, usize)>> = vec![HashSet::new(); data.node_count()];

    let mut initial_activated: u64 = 0;
    for i in 0..data.node_count() {
        if *data.node_value(i) == 0 {
            initial_activated |= 1 << i;
        }
    }
    queue.push((0, 0, initial_activated, 0usize), (Reverse(0usize), 0));
    while !queue.is_empty() {
        let ((added_pressure, node, activated_valves, _), (time_taken, pressure)) = queue.pop().unwrap();
        let time_taken = time_taken.0;
        visits[node].retain(|x| !(x.0 <= pressure && x.1 <= added_pressure));
        visits[node].insert((pressure, added_pressure));

        if time_taken == 29 {
            return pressure + added_pressure;
        }

        let new_pressure = pressure + added_pressure;
        'neighbors: for i in data.neighbors(node) {
            let visits_here = &visits[*i];
            if visits_here.iter().any(|v| v.0 >= new_pressure && v.1 >= added_pressure) {
                // If the last time we visited (which is always earlier) we had better pressure and better added pressure,
                // we don't need to visit again; that version was purely better
                continue 'neighbors;
            }
            queue.push_increase((added_pressure, *i, activated_valves, time_taken + 1), (Reverse(time_taken + 1), new_pressure));
        }

        if activated_valves & (1 << node) == 0 {
            queue.push_increase((added_pressure + *data.node_value(node), node, activated_valves | (1 << node), time_taken + 1), (Reverse(time_taken + 1), new_pressure));
        }
    }

    unreachable!()
}

#[cfg(feature = "part-two")]
pub fn run_step2(data: Data) -> usize {
    let mut queue = PriorityQueue::new();
    let mut visits: Vec<HashSet<(usize, usize)>> = vec![HashSet::new(); data.node_count()];

    let mut initial_activated: u64 = 0;
    for i in 0..data.node_count() {
        if *data.node_value(i) == 0 {
            initial_activated |= 1 << i;
        }
    }
    queue.push((0, 0, 0, initial_activated, 0usize), (Reverse(0usize), 0));
    while !queue.is_empty() {
        let ((added_pressure, node, other_node, activated_valves, _), (time_taken, pressure)) = queue.pop().unwrap();
        let time_taken = time_taken.0;
        visits[node].retain(|x| !(x.0 <= pressure && x.1 <= added_pressure));
        visits[node].insert((pressure, added_pressure));
        visits[other_node].retain(|x| !(x.0 <= pressure && x.1 <= added_pressure));
        visits[other_node].insert((pressure, added_pressure));

        if time_taken == 25 {
            return pressure + added_pressure;
        }
        let new_pressure = pressure + added_pressure;

        for i in data.neighbors(node) {
            let visits_here = &visits[*i];
            if !visits_here.iter().any(|v| v.0 >= new_pressure && v.1 >= added_pressure) {
                // If the last time we visited (which is always earlier) we had better pressure and better added pressure,
                // we don't need to visit again; that version was purely better, so we go don't need to next.
                // You might think that if we need to not move, we'd have problems, but we don't care about that, because all of the other nodes have a lower pressure (otherwise how did we get here).

                for j in data.neighbors(other_node) {
                    let next_node = (*i).min(*j);
                    let next_other_node = (*i).max(*j);
                    let visits_here = &visits[*j];
                    if visits_here.iter().any(|v| v.0 >= new_pressure && v.1 >= added_pressure) {
                        // If the last time we visited (which is always earlier) we had better pressure and better added pressure,
                        // we don't need to visit again; that version was purely better
                        continue;
                    }
                    queue.push_increase((added_pressure, next_node, next_other_node, activated_valves, time_taken + 1), (Reverse(time_taken + 1), new_pressure));
                }

                if activated_valves & (1 << other_node) == 0 {
                    let next_node = (*i).min(other_node);
                    let next_other_node = (*i).max(other_node);
                    queue.push_increase((added_pressure + *data.node_value(other_node), next_node, next_other_node, activated_valves | (1 << other_node), time_taken + 1), (Reverse(time_taken + 1), new_pressure));
                }
            } else if activated_valves & (1 << other_node) == 0 &&
                !visits_here.iter().any(|v| v.0 >= new_pressure && v.1 >= added_pressure + *data.node_value(other_node)) {
                // We're activating other_node because we can move if we do that.

                let next_node = (*i).min(other_node);
                let next_other_node = (*i).max(other_node);
                queue.push_increase((added_pressure + *data.node_value(other_node), next_node, next_other_node, activated_valves | (1 << other_node), time_taken + 1), (Reverse(time_taken + 1), new_pressure));
            }
        }

        if activated_valves & (1 << node) == 0 {
            for j in data.neighbors(other_node) {
                let next_node = node.min(*j);
                let next_other_node = node.max(*j);
                let visits_here = &visits[*j];
                if visits_here.iter().any(|v| v.0 >= new_pressure && v.1 >= added_pressure + *data.node_value(node)) {
                    // If the last time we visited (which is always earlier) we had better pressure and better added pressure,
                    // we don't need to visit again; that version was purely better
                    continue;
                }
                queue.push_increase((added_pressure + *data.node_value(node), next_node, next_other_node, activated_valves | (1 << node), time_taken + 1), (Reverse(time_taken + 1), new_pressure));
            }

            if activated_valves & (1 << other_node) == 0  {
                queue.push_increase((added_pressure + if other_node != node { *data.node_value(other_node) } else { 0 } + *data.node_value(node),
                                     node, other_node, activated_valves | (1 << other_node) | (1 << node), time_taken + 1),
                                    (Reverse(time_taken + 1), new_pressure));
            }
        }
    }

    unreachable!()
}

type ConvertData<'a> = Vec<&'a str>;

pub fn convert(data: ConvertData, _profiling: Instant) -> Data {
    let mut map = HashMap::new();
    let mut counter: usize = 0;
    fn get_or_insert<'a>(map: &mut HashMap<&'a str, usize>, counter: &mut usize, index: &'a str) -> usize {
        if map.contains_key(&index) {
            *map.get(&index).unwrap()
        } else {
            map.insert(index, *counter);
            *counter += 1;
            *counter - 1
        }
    }

    let mut edges = Vec::new();
    let mut nodes = vec![0; data.len()];
    for i in data {
        let x: IResult<_, _> = preceded(tag("Valve "), take(2usize))(i);
        let (rest, name) = x.unwrap();
        let x: IResult<_, _> = preceded(tag(" has flow rate="), take_till(|c: char| c == ';'))(rest);
        let (rest, flow): (&str, &str) = x.unwrap();
        let x: IResult<_, _> = preceded(alt((tag("; tunnel leads to valve "), tag("; tunnels lead to valves "))), separated_list0(tag(", "), take(2usize)))(rest);
        let list = x.unwrap().1;

        let node = get_or_insert(&mut map, &mut counter, name);

        for i in list {
            edges.push((node, get_or_insert(&mut map, &mut counter, i)));
        }
        nodes[node] = flow.parse::<usize>().unwrap();
    }

    let graph : UndirectedCsrGraph<usize, usize> = GraphBuilder::new().csr_layout(CsrLayout::Deduplicated).edges(edges).node_values(nodes).build();
    Rc::new(graph)
}

pub fn free_convert(data: Vec<&str>) -> ConvertData {
    data
}