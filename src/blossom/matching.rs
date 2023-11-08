use std::collections::HashMap;

use crate::blossom::Edge;
use crate::Vertex;


/// Represents a matching in a graph.
#[derive(Clone, Debug)]
pub struct Matching {
    edges: HashMap<Vertex, Vertex>,
}

impl Matching {
    /// Create a new matching from given edges
    pub fn new(edges: &[Edge]) -> Matching {
        let mut map = HashMap::new();
        for &(v, w) in edges {
            map.insert(v, w);
            map.insert(w, v);
        }
        Matching { edges: map }
    }

    /// Returns a value indicating whether the matching is empty.
    pub fn is_empty(&self) -> bool {
        self.edges.is_empty()
    }

    /// Returns the number of edges in the graph.
    pub fn len(&self) -> usize {
        self.edges.len() / 2
    }

    /// Exports all edges in a vector.
    pub fn edges(&self) -> Vec<Edge> {
        self.edges
            .iter()
            .filter(|&(&v, &w)| v < w)
            .map(|(&v, &w)| (v, w))
            .collect()
    }

    /// Exports all edges in a vector.
    pub fn vertices(&self) -> Vec<Vertex> {
        self.edges.keys().cloned().collect()
    }

    /// Gets the vertex that `vertex` is connected to.
    /// Panics if matching does not contain a vertex `vertex`.
    pub fn partner(&self, vertex: Vertex) -> Vertex {
        self.edges[&vertex]
    }

    /// Creates a contracted matching with edges between `leafs` removed.
    pub fn contract(&self, leafs: &[Vertex]) -> Matching {
        let mut edges = self.edges.clone();
        for leaf in leafs {
            edges.remove(leaf);
        }
        Matching { edges }
    }

    /// Creates a expanded matching using the augmenting `path`.
    pub fn augment(&self, path: &[Vertex]) -> Matching {
        let mut edges = self.edges.clone();
        for leaf in path {
            edges.remove(leaf);
        }
        for (i, j) in (0..path.len() / 2).map(|i| (2 * i, 2 * i + 1)) {
            edges.insert(path[i], path[j]);
            edges.insert(path[j], path[i]);
        }
        Matching { edges }
    }

    /// Adds a matching
    pub fn add(&self, other: &Matching) -> Matching {
        let mut edges = self.edges.clone();
        for (&v, &w) in &other.edges {
            edges.insert(v, w);
        }
        Matching { edges }
    }
}
