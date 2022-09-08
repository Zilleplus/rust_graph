mod algorithms;
mod graphs;
mod dijkstra;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Edge {
    from: u32,
    to: u32,
    cost: u32
}

impl PartialOrd for Edge{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cost.partial_cmp(&other.cost)
    }
}

impl Ord for Edge{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.cost.cmp(&other.cost);
    }
}

pub trait Graph {
    fn adjacent_edges(&self, node: u32) -> Vec<Edge>;
    fn add_edge(&mut self, from: u32, to: u32, cost: u32) -> Edge;

    fn add_node(&mut self) -> u32;
    fn num_of_nodes(&self) -> u32;

    fn add_double_edge(&mut self, from: u32, to: u32, cost: u32){
        self.add_edge(from, to, cost);
        self.add_edge(to, from, cost);
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
