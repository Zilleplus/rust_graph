use std::collections::HashSet;

use crate::Graph;

/// Loops through the graph in a depth first order.
/// 
/// # Arguments
/// 
/// * `g` - The graph
/// * `F` - Function called with node
/// * `start` - node to start the depth first search
pub fn depth_first_search<F, G>(g: &G, f: &mut F, start: u32) -> ()
where
    F: FnMut(u32),
    G: Graph,
{
    let mut visited = HashSet::<u32>::new();
    let mut to_visit = vec![start];

    while !to_visit.is_empty() {
        if let Some(n) = to_visit.pop() {
            if !visited.contains(&n) {
                f(n.clone());
                visited.insert(n.clone());
            }

            // Add this children if not yet visited
            for e in g.adjacent_edges(n) {
                let child = e.to;
                if !visited.contains(&child) {
                    to_visit.push(child);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graphs::*;

    #[test]
    fn depth_first_search_test() {
        let mut g = AdjacentListGraph::new();
        let n0 = g.add_node();
        let n1 = g.add_node();
        g.add_double_edge( n0, n1, 1);
        let n2 = g.add_node();
        g.add_double_edge(n1, n2, 1);

        let n3 = g.add_node();
        let n4 = g.add_node();
        g.add_double_edge(n1, n3, 1);
        g.add_double_edge(n1, n4, 1);

        let mut res: Vec<u32> = Vec::new();

        let mut f = |x: u32| -> () {
            res.push(x.clone());
        };

        depth_first_search(&g, &mut f, n0);

        let exp = vec![n0, n1, n4, n3, n2];

        assert_eq!(res, exp);
    }
}
