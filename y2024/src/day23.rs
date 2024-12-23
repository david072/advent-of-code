use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../day23.txt");

fn main() {
    // adjacency list for every node
    let mut graph = HashMap::<&str, Vec<&str>>::new();
    for l in INPUT.lines() {
        let (node1, node2) = l.split_once('-').unwrap();
        graph
            .entry(node1)
            .and_modify(|v| v.push(node2))
            .or_insert_with(|| vec![node2]);
        graph
            .entry(node2)
            .and_modify(|v| v.push(node1))
            .or_insert_with(|| vec![node1]);
    }

    let mut triples = HashSet::<[&str; 3]>::new();
    for (node1, adjacent_nodes) in graph.iter() {
        if !node1.starts_with('t') {
            continue;
        }

        for node2 in adjacent_nodes {
            for node3 in graph[node2].iter().filter(|n| adjacent_nodes.contains(n)) {
                let mut v = [*node1, *node2, *node3];
                v.sort();
                triples.insert(v);
            }
        }
    }

    println!("triples: {}", triples.len());

    // Go through all triples and try to "expand" it with nodes that are connected to all other
    // nodes already in the group.
    // I can't think of proof that this works for every input though lol...
    let mut largest_group = vec![];
    for [t1, t2, t3] in triples {
        let mut group = vec![t1, t2, t3];
        for (node, adjacent_nodes) in graph.iter() {
            if group.contains(node) {
                continue;
            }

            if group.iter().all(|n| adjacent_nodes.contains(n)) {
                group.push(node);
            }
        }

        if group.len() > largest_group.len() {
            group.sort();
            largest_group = group;
        }
    }

    println!("password: {}", largest_group.join(","));
}
