use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::path::PathBuf;

use clap::Parser;
use csv::Reader;
use anyhow::anyhow;

type Edge = Result<(u32, u32), anyhow::Error>;

/// Read the graph.  Return the edges as tuples. Return
/// errors lazily.
fn read_edges(csv: Reader<File>) -> impl Iterator<Item = Edge> {
    csv
        .into_records()
        .map(|r| {
            let r = r?;
            Ok::<(u32, u32), anyhow::Error>((
                r.get(0).ok_or(anyhow!("get 0"))?.parse()?,
                r.get(1).ok_or(anyhow!("get 1"))?.parse()?,
            ))
        })
}


type Graph = HashMap<u32, HashSet<u32>>;

/// Build the adjacency map. Ensure bidirectionality.
fn build_graph(edges: impl Iterator<Item = Edge>) -> Result<Graph, anyhow::Error> {
    let mut graph: Graph = HashMap::new();
    for edge in edges {
        // Stop on broken edge.
        let (start, end) = edge?;
        graph.entry(start).or_default().insert(end);
        graph.entry(end).or_default().insert(start);
    }
    Ok(graph)
}

/// Breadth-First Search the graph.
fn bfs(graph: &Graph, init: u32, goal: u32) -> Result<Option<Vec<u32>>, anyhow::Error> {
    let mut q = VecDeque::from([init]);
    // Keep the parent of each node along the shortest path.
    // If the node is encountered later, it must not be
    // along a shortest path.
    let mut parents = HashMap::from([(init, None)]);
    while let Some(mut node) = q.pop_front() {
        if node == goal {
            println!("path found");
            let mut path = vec![node];
            while let Some(parent) = parents[&node] {
                path.push(parent);
                node = parent;
            }
            path.reverse();
            return Ok(Some(path));
        }
        for &child in graph.get(&node).ok_or(anyhow!("no node {}", node))?.iter() {
            if parents.contains_key(&child) {
                continue;
            }
            parents.insert(child, Some(node));
            q.push_back(child);
        }
    }
    Ok(None)
}


#[derive(Parser)]
struct Args {
    path: PathBuf,
    init: u32,
    goal: u32,
}

fn main() -> Result<(), anyhow::Error> {
    // Get arguments. Allow non-UTF8 pathnames.
    let args = Args::parse();

    // Read the edges.
    let csv = Reader::from_path(args.path)?;
    let edges = read_edges(csv);

    // Build the graph.
    let graph = build_graph(edges)?;

    // Show the solution.
    if let Some(path) = bfs(&graph, args.init, args.goal)? {
        for p in path.iter() {
            println!("{}", p);
        }
    } else {
        println!("no path found");
    }

    Ok(())
}    
