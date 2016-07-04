use std::collections::HashMap;
use std::collections::HashSet;

use tuple_reader::TupleReader;

// graph vertices are represented as integer numbers
pub type Vertex = u32;

// adjacency map contains a list of adjacent vertices for each vertex in the graph
type Adjacencies = HashMap<Vertex, HashSet<Vertex>>;

// list of connected components
pub type ConnectedComponents = Vec<Vec<Vertex>>;

// a graph consists of a list of edges
// TODO: how to represent vertices that do not have any edges?
#[derive(Debug)]
pub struct Graph {
    vertices: HashSet<Vertex>,
    edges: Vec<(Vertex, Vertex)>,
}

impl Graph {

    pub fn new(vertices: HashSet<Vertex>, edges: Vec<(Vertex, Vertex)>) -> Graph {
        Graph { vertices: vertices, edges: edges }
    }

    // loads a graph from an input stream:
    // first line contains the number of vertices v and edges e
    // next e lines contain pairs of vertices representing the edges of the graph
    pub fn load<T: TupleReader>(reader: &mut T) -> Graph {
        let (v, e) = reader.next_tuple();
        let vertices = (1..v+1).collect();
        let mut edges = vec![];
        for _ in 0..e { 
            let edge = reader.next_tuple();
            edges.push(edge)
        }
        Graph::new(vertices, edges)
    }

    // builds the adjacency map for the graph
    fn adjacencies(&self) -> Adjacencies {
        let mut adj = HashMap::new();
        for vertex in &self.vertices {
            adj.insert(*vertex, HashSet::new());
        }
        for edge in &self.edges {
            adj.get_mut(&edge.0).unwrap().insert(edge.1);
            adj.get_mut(&edge.1).unwrap().insert(edge.0);
        }
        adj
    }

    // depth first search of the entire graph
    // returns the set of connected components
    fn depth_first_search(&self) -> ConnectedComponents {
        let mut components = vec![];
        let mut visited = HashSet::new();
        for v in &self.vertices {
            if !visited.contains(v) {
                let mut component = vec![];
                self.explore(v, &mut visited, &mut component);
                components.push(component);
            }
        }
        components
    }

    // depth first search of the graph starting at vertex v
    // marks each vertex visited during the search and returns the list of visited vertices
    fn explore(&self, v: &Vertex, visited: &mut HashSet<Vertex>, component: &mut Vec<Vertex>) {
        fn visit(v: &Vertex, adj: &Adjacencies, visited: &mut HashSet<Vertex>, component: &mut Vec<Vertex>) {
            visited.insert(v.clone());
            component.push(v.clone());
            if let Some(adjacent) = adj.get(v) {
                for w in adjacent {
                    if !visited.contains(w) {
                        visit(w, adj, visited, component);
                    }
                }
            }
        }

        let adj = &self.adjacencies();
        visit(&v, &adj, visited, component);
    }

    // returns true if vertex w can be reached from vertex v
    pub fn is_reachable(&self, v: Vertex, w: Vertex) -> bool {
        let mut visited = HashSet::new();
        let mut component = vec![];
        self.explore(&v, &mut visited, &mut component);
        visited.contains(&w)
    }

    // returns the connected components for the graph
    pub fn connected_components(&self) -> ConnectedComponents {
        self.depth_first_search()
    }
}
