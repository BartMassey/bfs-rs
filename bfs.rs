use std::collections::{HashMap, HashSet};

use csv::Reader;
use anyhow::anyhow;

fn main() -> Result<(), anyhow::Error> {
    // Get the pathname as an argument. Allow non-UTF8 pathnames.
    let path: std::ffi::OsString = std::env::args_os().nth(1).unwrap();
    // Open the CSV reader on the file.
    let mut csv = Reader::from_path(&path)?;

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

    println!("{:?}", graph);

    Ok(())
}    
