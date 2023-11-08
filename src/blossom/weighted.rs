use std::cmp::Ordering::{Equal, Greater, Less};
use std::f64;

use crate::blossom::graph::AnnotatedGraph;
use crate::blossom::matching::Matching;
use crate::blossom::Edge;

pub type Weight = f64;
pub type WeightedGraph<T = Weight> = AnnotatedGraph<T>;

impl<T> WeightedGraph<T>
where
    T: PartialEq + PartialOrd + Copy,
{
    /// Returns a full matching with iteratively the maximum minimal weight edge.
    pub fn maximin_matching(&self) -> Option<Matching> {
        if self.is_empty() {
            return Some(Matching::new(&[]));
        }

        let cmp = |x: &T, y: &T| x.partial_cmp(y).unwrap_or(Equal);
        let cmp_rev = |x: &T, y: &T| cmp(y, x);
        self.full_matching()?;

        let min_max_value = self
            .vertices()
            .iter()
            .map(|&v| self.edges_from(v).1.iter().cloned().max_by(cmp).unwrap())
            .min_by(cmp)
            .unwrap();
        let mut values: Vec<_> = self
            .vertices()
            .iter()
            .flat_map(|&vertex| self.edges_from(vertex).1)
            .filter(|&w| cmp(w, &min_max_value) != Greater)
            .cloned()
            .collect();
        values.sort_by(cmp_rev);

        for value in values {
            let limited = self.limit(value);
            if let Some(matching) = limited.full_matching() {
                let mut edges: Vec<Edge> = matching
                    .edges()
                    .into_iter()
                    .filter(|edge| limited.weight(edge).eq(&value))
                    .collect();
                edges.sort_by_key(|&(v, w)| limited.vertices_from(v).min(limited.vertices_from(w)));
                let sub_matching = limited
                    .filter_vertices(|&v| v != edges[0].0 && v != edges[0].1)
                    .maximin_matching()
                    .unwrap();
                return Some(sub_matching.add(&Matching::new(&edges[0..1])));
            }
        }

        None
    }

    // Clone graph removing edges with weight less than given limit
    pub fn limit(&self, weight: T) -> WeightedGraph<T> {
        self.filter_edges(|_, _, &w| w.partial_cmp(&weight) != Some(Less))
    }

    fn weight(&self, edge: &Edge) -> T {
        let (v, w) = self.edges_from(edge.0);
        *v.iter().zip(w.iter()).find(|t| *t.0 == edge.1).unwrap().1
    }
}

#[test]
fn find_match_sparse() {
    let g = [
        (0, (vec![2, 3], vec![0.1, 0.9])),
        (1, (vec![2, 3], vec![0.3, 0.2])),
        (2, (vec![0, 1], vec![0.1, 0.3])),
        (3, (vec![0, 1], vec![0.9, 0.2])),
    ]
    .iter()
    .collect::<WeightedGraph>();
    let m = g.maximin_matching();
    assert!(m.is_some());
    if let Some(m) = m {
        assert_eq!(m.partner(0), 3);
        assert_eq!(m.partner(1), 2);
    }
}

#[test]
fn find_match_dense() {
    let g = [
        (0, (vec![1, 2, 3], vec![0.1, 0.3, 0.5])),
        (1, (vec![0, 2, 3], vec![0.1, 0.2, 0.6])),
        (2, (vec![0, 1, 3], vec![0.3, 0.2, 0.4])),
        (3, (vec![0, 1, 2], vec![0.5, 0.6, 0.4])),
    ]
    .iter()
    .collect::<WeightedGraph>();
    let m = g.maximin_matching();
    assert!(m.is_some());
    if let Some(m) = m {
        assert_eq!(m.partner(0), 2);
        assert_eq!(m.partner(1), 3);
    }
}
