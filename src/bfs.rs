use std::collections::{HashMap, HashSet, VecDeque};

pub(crate) type Node = usize;
// pub(crate) means that node is visible throughout the entire crate (bfs.rs) but not in other modules 
// each node is designated as usize

pub struct Graph {
    adjacency_list: HashMap<Node, Vec<Node>>,
}
// Creates a graph struct, which graphs the edges csv as hashmap, with a node connected to a vector of associated nodes it leads to

impl Graph {
    pub fn new() -> Self {
        Graph {
            adjacency_list: HashMap::new(),
        }
    }
    //The "new" function makes a new adjacency list and initializes an empty hashmap in which we can enter our data

    pub fn add_edge(&mut self, from: Node, to: Node) {
        self.adjacency_list // accesses empty adjacency list
            .entry(from) //finds entry in the hash map equal to the item we are looking for
            .or_insert_with(Vec::new) //inserts a default value if there is no entry key
            .push(to); //adds the to node (connected node) from the given element
    }
    //"add_edge" takes in a data set (tuples, wich an origin and destination node) 

    #[cfg(test)]
    pub fn num_nodes(&self) -> usize {
        self.adjacency_list.keys().count()
    }
    //This is important for our test function. It calculates the number of nodes by doing the following:
    // iterates over the key values in the adjacency list and counts them, telling us the total number of nodes 

    #[cfg(test)]
    pub fn num_edges(&self) -> usize {
        self.adjacency_list.values().map(|v| v.len()).sum()
    }
    //This is important for our test function. It calculates the number of edges by doing the following:
    //It iterates through the vectors associated with each node in the adjacency list
    // The length of each vector is calculated, which tells you the number of edges associated with each node
    // These edges are summed, telling us the total number of edges

    pub fn in_degree(&self, node: Node) -> usize {
        self.adjacency_list
            .values()
            .filter(|neighbors| neighbors.contains(&node))
            .count()
    }
    //This function calculates the total number of incoming edges to a specific node (ie. how many nodes connect *into* node _______)
    // calculated by going into the adjacency list, finding the values, filtering into the neighbors that contain the node you input
    // by couting the number of neighbors containing that node, you can tell how many nodes connect into a given node 

    pub fn out_degree(&self, node: Node) -> usize {
        self.adjacency_list
            .get(&node)
            .map_or(0, |neighbors| neighbors.len())
    }
    //This function calculates the total number of outgoing edges from a specific node (ie. how many outgoing connections does node _____ have)
    // The out degree is calculated by finding the adjacency list connected to a specific inputted node, and simply calculating the length of that list 

    pub fn centrality(&self, node: Node) -> f64 {
        let in_degree = self.in_degree(node) as f64;
        let out_degree = self.out_degree(node) as f64;
        let total_nodes = self.adjacency_list.len() as f64;

        if total_nodes > 1.0 {
            (in_degree + out_degree) / (total_nodes)
        } else {
            0.0 // Avoid division by zero
        }
    }
    // centrality for a given node is calculated by the total number of nodes connected into and out of a node divided by the total number of nodes
    // for the else case where there are no total nodes, it will print out 0.0 

    pub fn graph_centralities(&self) -> HashMap<Node, f64> {
        let mut centrality_values = HashMap::new();

        for &node in self.adjacency_list.keys() {
            let centrality = self.centrality(node);
            centrality_values.insert(node, centrality);
        }
        centrality_values
    }
    // Creates an empty hashmap 
    //Iterates through every node in an inputted graph, and prints out the centrality values for each node 
    // We now craete a new hashmap in which a node and it's cenrality can be accessed 
    // Cenrality is f64 because very long decimals were likely (I guessed maybe 10-20 max degrees divided by over 10000 nodes)

    pub fn bfs_shortest_path(&self, start: Node, end: Node) -> Option<Vec<Node>> {
        // input a list, the origin and destination, returns a vector of nodes to get from one to another
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut prev = HashMap::new();
        // visited: A HashSet to keep track of visited nodes. Hashset, not hashmap because these are not pairs of values, and we want no repeated values
        // queue: A queue for the breadth-first search traversal, thorugh which we can use push_back and pop_front to continue the traversal
        // prev: A HashMap to store the nodes used for the shortest path

        queue.push_back(start);
        visited.insert(start);
        // initializes bfs and start node is considered visited

        while let Some(current) = queue.pop_front() {
            // starts bfs with a clause that continues until the queue is exhausted
            if current == end {
                // Makes sure the current node is the destination, and reconstructs a path until it finds a path that does so 
                let mut path = Vec::new();
                let mut node = current;

                while let Some(&prev_node) = prev.get(&node) {
                    //while there is a revious node associated with the node
                    path.push(node);
                    // push the current node into the path
                    node = prev_node;
                    // now this node is the previous node and we move to the next iteration
                }

                path.push(start);
                path.reverse();
                return Some(path);
            }

            if let Some(neighbors) = self.adjacency_list.get(&current) {
                // if there are neighbors for the current node in the adjacency list 
                for &neighbor in neighbors {
                    // for every neighbor in the adjacency list 
                    if !visited.contains(&neighbor) {
                        // if this neighbor hasn't been visited 
                        visited.insert(neighbor);
                        // insert this neigbor to be visited
                        prev.insert(neighbor, current);
                        // Explore the path with this neighbor
                        queue.push_back(neighbor);
                    }
                }
            }
        }
        None
        // in case there is no best path from one node to another 
        // There are likely 0 none cases in the chicago road network. no road is not accessible in some way by other roads. 
    }
}