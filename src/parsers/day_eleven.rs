#![allow(dead_code, unused)]
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};
use std::time::Instant;
use std::{error::Error, fs};
use std::mem;

// it's a directed graph connections are one way
// starting node "you" ending node "out"
// find the number of paths from "you" to "out"

fn parser(path: &str) -> Result<HashMap<String, Vec<String>>, Box<dyn Error>> {
    let mut content = fs::read_to_string(path)
        .map_err(|e| format!("Unexpected file error while reading contents.\n{e:?}"))?;
    let mut node_map = HashMap::new();
    for line in content.lines() {
        let (node, neighbors) = line
            .split_once(":")
            .ok_or("Unable to split line successfully")?;
        node_map.insert(
            node.to_string(),
            neighbors
                .split_whitespace()
                .map(|n| n.to_string())
                .collect(),
        );
    }
    Ok(node_map)
}
