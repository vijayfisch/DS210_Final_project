#[cfg(test)]
mod tests {
    use crate::Graph;
    use crate::read_file;

    #[derive(Debug, PartialEq, Eq)]
    struct TestData {
        origin: usize,
        dest: usize,
    }

    #[test]
    fn test_path() {
        let mut graph = Graph::new();

        let edges = vec![
            TestData {origin: 1, dest: 2},
            TestData {origin: 2, dest: 3},
            TestData {origin: 3, dest: 4},
            TestData {origin: 4, dest: 5},
            TestData {origin: 5, dest: 6},
            TestData {origin: 6, dest: 7},
            TestData {origin: 7, dest: 8},
            TestData {origin: 8, dest: 9},
            TestData {origin: 9, dest: 10},
        ];

        for edge in &edges {
            graph.add_edge(edge.origin, edge.dest);
        }

        let path = graph.bfs_shortest_path(1, 10);
        assert_eq!(path, Some(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));
    }

    #[test]
    fn test_disconnected_nodes() {
        let mut graph = Graph::new();

        let edges = vec![
            TestData {origin: 1, dest: 2},
            TestData {origin: 2, dest: 3},
            TestData {origin: 3, dest: 4},
            TestData {origin: 4, dest: 5},
            TestData {origin: 5, dest: 6},
            TestData {origin: 7, dest: 8},
        ];

        for edge in &edges {
            graph.add_edge(edge.origin, edge.dest);
        }

        // Node 8 is only connected to node 7
        let start_node = 1;
        let end_node = 8;

        let path = graph.bfs_shortest_path(start_node, end_node);

        assert_eq!(path, None);
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
}