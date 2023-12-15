#[cfg(test)]
mod tests {
    use crate::Graph;
    use crate::read_file;

    #[test]
    fn test_path() {
        //makes a new graph
        let mut graph = Graph::new();

        let chicago = "edges.csv";
        let edges = read_file(chicago);

        for &(from, to) in &edges {
            graph.add_edge(from, to);
        }

        // Tests a random path that does work, makes sure it finds ths shortest path 
        let path = graph.bfs_shortest_path(8017, 8740);
        assert_eq!(path, Some(vec![8017, 8739, 8740]));
    }

    #[test]
    fn test_data_size() {
        let mut graph = Graph::new();
     
        let chicago = "edges.csv";
        let edges = read_file(chicago);
    
        for &(from, to) in &edges {
            graph.add_edge(from, to);
        }
    
        // Checks num nodes and edges are over 1000 using function in the BFS module
        assert!(graph.num_nodes() > 1000);
        assert!(graph.num_edges() > 1000);
    }   

    #[test]
    fn test_disconnected_nodes() {
        let mut graph = Graph::new();

        let chicago = "edges.csv";
        let edges = read_file(chicago);

        for &(from, to) in &edges {
            graph.add_edge(from, to);
        }

        // I know that node 12983 is disconnected
        let start_node = 1;
        let end_node = 12983;

        // Perform BFS on disconnected nodes
        let path = graph.bfs_shortest_path(start_node, end_node);

        // Assert that there is no path between these nodes
        assert_eq!(path, None);
    }
}
