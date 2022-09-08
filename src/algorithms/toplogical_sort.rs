use std::collections::HashSet;
use crate::Graph;

/// Loops through the graph in a breadth first order.
/// 
/// # Arguments
/// 
/// * `g` - The graph
/// * `F` - Function called with node
/// * `start` - node to start the depth first search
/// 
/// # Returns
/// * `Result<Vec<u32>, String>' - List of nodes or string with error.
pub fn topological_sort<G>(graph: &G) -> Result<Vec<u32>, String>
    where G : Graph
{
    let mut out = Vec::new();

    // only works with Directed acyclic graph (DAG)
    let mut visited = HashSet::new();
    let mut visiting = HashSet::new();

    let num_nodes = graph.num_of_nodes();
    for n in 0..num_nodes
    {
        visit(n.clone(), graph, &mut out, &mut visited, &mut visiting)?;
    }

    out.reverse();

    return Ok(out);
}

fn visit<G>(n: u32, graph: &G, out: &mut Vec<u32>, visited: &mut HashSet<u32>, visiting: & mut HashSet<u32>) -> Result<(), String>
    where G : Graph
{
    if visiting.contains(&n){
        // This is not a DAG
        return Err("Found cylic dependency on node {n}".to_owned());
    }

    if visited.contains(&n){
        return Ok(());
    }

    visiting.insert(n.clone());

    for e in graph.adjacent_edges(n){
        visit(e.to.clone(), graph, out, visited, visiting)?;
    }

    out.push(n.clone());
    visiting.remove(&n);
    visited.insert(n.clone());

    return Ok(());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graphs::*;

    #[test]
    fn topological_sort_test() {
        let mut g = AdjacentListGraph::new();
        let n0 = g.add_node();
        let n1 = g.add_node();
        let n2 = g.add_node();
        let n3 = g.add_node();
        let n4 = g.add_node();
        let n5 = g.add_node();
        let n6 = g.add_node();
        let n7 = g.add_node();

        g.add_edge(n0.clone(), n2.clone(), 1);
        g.add_edge(n2.clone(), n4.clone(), 1);
        g.add_edge(n4.clone(), n7.clone(), 1);

        g.add_edge(n1.clone(), n3.clone(), 1);
        g.add_edge(n3.clone(), n4.clone(), 1);

        g.add_edge(n5.clone(), n6.clone(), 1);
        g.add_edge(n6.clone(), n7.clone(), 1);

        let expected_sorted_nodes = vec![n5, n6, n1, n3, n0, n2,n4, n7];
        let sorted_nodes = topological_sort(&g).unwrap();

        assert_eq!(sorted_nodes, expected_sorted_nodes);

    }
}