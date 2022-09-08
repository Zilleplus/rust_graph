use crate::{Edge, Graph};

pub struct AdjacentListGraph {
    edges: Vec<Vec<Edge>>,
    number_of_nodes: u32,
}

impl AdjacentListGraph{
    pub fn new() -> AdjacentListGraph
    {
        AdjacentListGraph{edges: Vec::new(), number_of_nodes: 0}
    }
}

impl Graph for AdjacentListGraph {
    fn adjacent_edges<'a>(&'a self, node: u32) -> Vec<Edge>{
        assert!(node < self.number_of_nodes);
        let i = usize::try_from(node).unwrap();
        self.edges[i].clone()
    }

    fn add_edge(&mut self, from: u32, to: u32, cost: u32) -> Edge {
        assert!(from < self.number_of_nodes);
        assert!(to < self.number_of_nodes);

        let new_edge = Edge { from, to, cost };
        let from_i = usize::try_from(new_edge.from).unwrap();
        
        // Don't add the edge if it's already present.
        if !self.edges[from_i].contains(&new_edge)
        {
            self.edges[from_i].push(new_edge.clone());
        }

        new_edge
    }

    fn add_node(&mut self) -> u32 {
        self.number_of_nodes = self.number_of_nodes + 1;
        self.edges.push(Vec::new());
        u32::from(self.number_of_nodes - 1)
    }

    fn num_of_nodes(&self) -> u32 {
        self.number_of_nodes
    }
}