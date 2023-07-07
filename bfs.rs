use std::collections::{HashMap, HashSet, VecDeque};
use std::path::PathBuf;

use clap::Parser;
use csv::Reader;
use anyhow::anyhow;

#[derive(Parser)]
struct Args {
    path: PathBuf,
    init: u32,
    goal: u32,
}

fn main() -> Result<(), anyhow::Error> {
    // Get arguments. Allow non-UTF8 pathnames.
    let args = Args::parse();
    // Open the CSV reader on the file.
    let mut csv = Reader::from_path(&args.path)?;

    // Read the graph from the file.  Iterate over the edges
    // as tuples.
    let edges = csv
        .records()
        .map(|r| {
            let r = r?;
            Ok::<(u32, u32), anyhow::Error>((
                r.get(0).ok_or(anyhow!("get 0"))?.parse()?,
                r.get(1).ok_or(anyhow!("get 1"))?.parse()?,
            ))
        });
    // Build the adjacency map. Ensure bidirectionality.
    let mut graph: HashMap<u32, HashSet<u32>> = HashMap::new();
    for edge in edges {
        // Stop on broken edge.
        let (start, end) = edge?;
        graph.entry(start).or_default().insert(end);
        graph.entry(end).or_default().insert(start);
    }

    // BFS
    let mut q = VecDeque::from([args.init]);
    // Keep the parent of each node along the shortest path.
    // If the node is encountered later, it must not be
    // along a shortest path.
    let mut parents = HashMap::from([(args.init, None)]);
    while let Some(mut node) = q.pop_front() {
        if node == args.goal {
            println!("path found");
            let mut path = vec![node];
            while let Some(parent) = parents[&node] {
                path.push(parent);
                node = parent;
            }
            for p in path.iter().rev() {
                println!("{}", p);
            }
            return Ok(());
        }
        for &child in graph.get(&node).ok_or(anyhow!("no node {}", node))?.iter() {
            if parents.contains_key(&child) {
                continue;
            }
            parents.insert(child, Some(node));
            q.push_back(child);
        }
    }

    println!("no path found");
    Ok(())
}    
