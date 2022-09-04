mod algorithms;
mod graphs;

#[derive(Debug, Clone, PartialEq)]
pub struct Edge {
    from: u32,
    to: u32,
}

pub trait Graph {
    fn adjacent_edges(&self, node: u32) -> Vec<Edge>;
    fn add_edge(&mut self, from: u32, to: u32) -> Edge;

    fn add_node(&mut self) -> u32;
    fn num_of_nodes(&self) -> u32;

    fn add_double_edge(&mut self, from: u32, to: u32){
        self.add_edge(from, to);
        self.add_edge(to, from);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
