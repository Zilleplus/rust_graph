use core::num;
use std::collections::BinaryHeap;

use crate::{Graph, Edge};

fn Dijkstra<G>(graph: &G, source: u32, target: u32) -> Result<Vec<u32>, String>
where G: Graph
{
    let edges_size = usize::try_from(graph.num_of_nodes())
        .unwrap_or( {
            return Err("unable to cast size type of graph edges".to_owned());
        });
    
    // Find the spanning tree.
    

    // contains all outgoing edges from the spanning tree
    // greatest element is on top -> so we invert the cost
    let q = BinaryHeap::<Edge>::new();
    let mut cost = vec![u32::MAX; edges_size];
    let mut prev: Vec<Option<u32>> = vec![None; edges_size];

    if let Ok(i) = usize::try_from(source){
        cost[i] = 0;
        prev[i] = Some(source);
    }

    for e in graph.adjacent_edges(source){
        q.push(e);
    }

    let mut target_found = false;
    while !q.is_empty(){
        if let Some(e) = q.pop()
        {
            if let Ok(i) = usize::try_from(e.to.clone()){
                if cost[i] < e.cost
                {
                    cost[i] = e.cost;
                    prev[i] = Some(e.from);
                }
            }

            if e.to == target{
                target_found = true;
                break;
            }

            for e in graph.adjacent_edges(e.to){
                q.push(e);
            }
        }
    }
    


    // Find the path in the spanning tree.
    let mut path = Vec::<u32>::new();
    return Ok(path);
}