use itertools::Itertools;
use petgraph::algo::astar;
use petgraph::algo::kosaraju_scc;
use petgraph::graphmap::UnGraphMap;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;

use crate::utils::input_process::input_to_lines;

fn nodes_to_key(lhs: &str, rhs: &str) -> String {
    let mut nodes = [lhs, rhs];
    nodes.sort();
    format!("{:}_{:}", nodes[0], nodes[1])
}

fn process_lines(lines: &Vec<String>) -> usize {
    let mut ungraph: UnGraphMap<&str, _> = UnGraphMap::new();

    for line in lines {
        let (source, destinations) = line.split_once(": ").expect("Err: not splittable");
        for destination in destinations.split(' ').collect_vec() {
            ungraph.add_edge(source, destination, 1);
        }
    }

    // Hoping this will be random enough
    let mut paths = ungraph.nodes().combinations(2).collect_vec();
    paths.shuffle(&mut thread_rng());

    let paths = paths
        .iter()
        .take(10000)
        .filter_map(|combination| {
            if let [a, b, ..] = combination[0..2] {
                astar(&ungraph, a, |finish| finish == b, |_| 1, |_| 0)
            } else {
                panic!("Err: refutable panic");
            }
        })
        .map(|res| res.1)
        .collect_vec();

    let mut path_occurrences: HashMap<String, u32> = HashMap::new();
    for path in paths.iter() {
        path.iter()
            .map_windows(|&[a, b]| {
                *path_occurrences.entry(nodes_to_key(a, b)).or_insert(0) += 1;
            })
            .collect_vec();
    }

    let path_occurrences = path_occurrences
        .iter()
        .sorted_by(|a, b| Ord::cmp(b.1, a.1))
        .collect_vec();

    path_occurrences.iter().take(3).for_each(|(key, _)| {
        let (node_a, node_b) = key.split_once('_').expect("Err: no removable nodes");
        ungraph.remove_edge(node_a, node_b);
    });

    let components = kosaraju_scc(&ungraph);
    components.iter().map(|component| component.len()).product()
}

pub fn run() {
    let input = "./days/day25/input.txt";
    let data = input_to_lines(input);
    let result = process_lines(&data);
    println!("\n day25 done with result {result}.");
}
