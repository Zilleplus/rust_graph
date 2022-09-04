use crate::Graph;
use std::collections::{HashSet, VecDeque};

/// Loops through the graph in a breadth first order.
/// 
/// # Arguments
/// 
/// * `g` - The graph
/// * `F` - Function called with node
/// * `start` - node to start the depth first search
pub fn breadth_first_search<F, G>(graph: &G, f: &mut F, start: u32) -> ()
where
    G: Graph,
    F: FnMut(u32),
{
    let mut q = VecDeque::new();
    let mut visited = HashSet::new();
    q.push_back(start);

    loop {
        if let Some(n) = q.pop_front() {
            f(n.clone());
            visited.insert(n.clone());

            for e in graph
                .adjacent_edges(n.clone())
                .iter()
                .filter(|e| !visited.contains(&e.to))
            {
                q.push_back(e.to);
            }
        } else {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graphs::*;
    use crate::Graph;

    #[test]
    fn depth_first_search_test() {
        let mut g = AdjacentListGraph::new();
        let n0 = g.add_node();
        let n1 = g.add_node();
        g.add_double_edge(n0, n1);
        let n2 = g.add_node();
        g.add_double_edge(n1, n2);

        let n3 = g.add_node();
        let n4 = g.add_node();
        g.add_double_edge(n1, n3);
        g.add_double_edge(n1, n4);

        let mut res: Vec<u32> = Vec::new();

        let mut f = |x: u32| -> () {
            res.push(x.clone());
        };

        breadth_first_search(&g, &mut f, n0);

        let exp = vec![n0, n1, n2, n3, n4];

        assert_eq!(res, exp);
    }
}
