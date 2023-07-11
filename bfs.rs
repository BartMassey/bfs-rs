//! Breadth-First Search demo.

use std::collections::{HashMap, HashSet, VecDeque};
use std::collections::hash_map::Entry::*;
use std::fs::File;
use std::path::PathBuf;

use anyhow::anyhow;
use clap::Parser;
use csv::{Reader, ReaderBuilder};

type Edge = Result<(u32, u32), anyhow::Error>;

/// Read the graph.  Return the edges as tuples. Return
/// errors lazily.
fn read_edges(csv: Reader<File>) -> impl Iterator<Item = Edge> {
    csv.into_records().map(|r| {
        let r = r?;
        let nr = r.len();
        if nr != 2 {
            return Err(anyhow!("record length {}", nr));
        }
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

#[test]
fn test_build_graph() {
    let edges = [Ok((0, 1)), Ok((1, 0)), Ok((0, 2)), Ok((1, 3)), Ok((2, 4))];
    let graph = build_graph(edges.into_iter()).unwrap();
    assert!(graph[&0].contains(&1));
    assert!(graph[&0].contains(&2));
    assert!(graph[&1].contains(&0));
    assert!(graph[&4].contains(&2));
    assert!(!graph[&4].contains(&1));
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
            let mut path = vec![node];
            while let Some(parent) = parents[&node] {
                path.push(parent);
                node = parent;
            }
            path.reverse();
            return Ok(Some(path));
        }
        let children = graph.get(&node).ok_or_else(|| anyhow!("no node {}", node))?;
        for &child in children {
            match parents.entry(child) {
                Occupied(_) => {
                    continue;
                }
                Vacant(e) => {
                    e.insert(Some(node));
                    q.push_back(child);
                }
            }
        }
    }
    Ok(None)
}

#[test]
fn test_bfs() {
    let edges = [
        (0u32, 1u32),
        (1, 2),
        (1, 3),
        (2, 4),
        (4, 5),
        (3, 5),
        (3, 6),
        (5, 7),
    ];
    let graph = build_graph(edges.into_iter().map(anyhow::Ok)).unwrap();
    let path = bfs(&graph, 0, 7).unwrap().unwrap();
    assert_eq!(&path, &[0u32, 1, 3, 5, 7]);
}

#[derive(Parser)]
struct Args {
    #[arg(long)]
    headers: bool,
    path: PathBuf,
    init: u32,
    goal: u32,
}

fn main() -> Result<(), anyhow::Error> {
    // Get arguments. Allow non-UTF8 pathnames.
    let args = Args::parse();

    // Read the edges.
    let csv = ReaderBuilder::new()
        .has_headers(args.headers)
        .from_path(args.path)?;
    let edges = read_edges(csv);

    // Build the graph.
    let graph = build_graph(edges)?;

    // Show the solution.
    if let Some(path) = bfs(&graph, args.init, args.goal)? {
        println!("path found");
        for p in path.iter() {
            println!("{}", p);
        }
    } else {
        println!("no path found");
    }

    Ok(())
}
