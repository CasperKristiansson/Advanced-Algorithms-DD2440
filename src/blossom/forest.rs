use std::collections::HashMap;

use crate::Vertex;

pub struct Forest {
    trees: HashMap<Vertex, Vec<Vertex>>,
}

impl Forest {
    pub fn new() -> Forest {
        Forest {
            trees: HashMap::new(),
        }
    }

    pub fn from_singletons(vertices: &[Vertex]) -> Forest {
        Forest {
            trees: vertices.iter().map(|&x| (x, vec![x])).collect(),
        }
    }

    pub fn depth(&self, vertex: Vertex) -> Option<usize> {
        self.trees.get(&vertex).map(|p| p.len())
    }

    pub fn find_path(&self, vertex: Vertex) -> Option<&[Vertex]> {
        self.trees.get(&vertex).map(|p| &p[..])
    }

    pub fn push(&mut self, v: Vertex, p: Vec<Vertex>) {
        self.trees.insert(v, p);
    }

    pub fn append(&mut self, other: &mut Forest) {
        for (v, p) in other.trees.drain() {
            self.trees.insert(v, p);
        }
    }
}
