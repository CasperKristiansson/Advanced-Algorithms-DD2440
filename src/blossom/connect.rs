use crate::blossom::{Graph, Vertex};
use std::collections::HashSet;

/// Gets the connected groups in the graph
pub fn connected_groups(graph: &Graph) -> Vec<Vec<Vertex>> {
    let mut groups = vec![];
    let mut seen = HashSet::new();
    let mut todo = HashSet::new();

    loop {
        for &v in graph.vertices() {
            if seen.insert(v) {
                todo.insert(v);
                break;
            }
        }

        if todo.is_empty() {
            return groups;
        }

        while let Some(&v) = todo.iter().next() {
            todo.remove(&v);
            for &e in graph.vertices_from(v) {
                if seen.insert(e) {
                    todo.insert(e);
                }
            }
        }

        groups.push(seen.iter().cloned().collect());
    }
}

#[test]
fn two_groups() {
    let g: Graph = [
        (0, vec![1]),
        (1, vec![0, 2]),
        (2, vec![1]),
        (3, vec![4]),
        (4, vec![3]),
    ]
    .iter()
    .collect();
    assert_eq!(2, connected_groups(&g).len());
}
