use std::iter;
use std::iter::FromIterator;

use crate::blossom::Vertex;

use crate::blossom::graph::AnnotatedGraph;

pub type Graph = AnnotatedGraph<()>;

impl FromIterator<(Vertex, Vec<Vertex>)> for Graph {
    fn from_iter<I: IntoIterator<Item = (Vertex, Vec<Vertex>)>>(iter: I) -> Self {
        Self::new(
            iter.into_iter()
                .map(|(v, e)| {
                    let len = e.len();
                    (v, (e, iter::repeat(()).take(len).collect()))
                })
                .collect(),
        )
    }
}

impl<'a> FromIterator<&'a (Vertex, Vec<Vertex>)> for Graph {
    fn from_iter<I: IntoIterator<Item = &'a (Vertex, Vec<Vertex>)>>(iter: I) -> Self {
        iter.into_iter().cloned().collect()
    }
}
