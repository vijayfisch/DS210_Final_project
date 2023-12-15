mod read_file;
mod bfs;
use bfs::Graph;
use read_file::read_file;

#[cfg(test)]
mod tests;

fn main() {
    let mut graph = Graph::new();

    let chicago = "edges.csv";
    let edges = read_file(chicago);

    for &(from, to) in &edges {
        graph.add_edge(from, to);
    }

    let centrality_values = graph.graph_centralities();

    // Sort nodes by centrality in descending order
    let mut sorted_nodes: Vec<_> = centrality_values.iter().collect();
    sorted_nodes.sort_by(|&(_, centrality1), &(_, centrality2)| centrality2.partial_cmp(centrality1).unwrap());

    println!("The top 5 central roads in Chicago are as follows:");
    for (i, (&node, &centrality)) in sorted_nodes.iter().take(5).enumerate() {
        println!("Number {} - Node {}: Centrality: {}", i + 1, node, centrality);
    
        // Print detailed information for the top nodes
        let path_to_1 = graph.bfs_shortest_path(node, 1);
    
        if let Some(path) = path_to_1 {
            println!("Shortest Path to road 1: {:?}", path);
        } else {
            println!("No path to road 1 from Node {}", node);
        }
    }  
}