pub type Vertex = usize;
pub type Edge = (Vertex, Vertex);


use std::collections::{HashMap, HashSet};
use std::iter;
use std::iter::FromIterator;

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





/// A graph with annotations on edges
#[derive(Clone, Debug)]
pub struct AnnotatedGraph<Annotation>
    where
        Annotation: Copy + Sized,
{
    vertices: Vec<Vertex>,
    edges: HashMap<Vertex, (Vec<Vertex>, Vec<Annotation>)>,
}

impl<Annotation> AnnotatedGraph<Annotation>
    where
        Annotation: Copy + Sized,
{
    /// Returns a new AnnotatedGraph instance
    ///
    /// # Arguments
    ///
    /// * `edges` - HashMap from vertex to edges and their meta data
    ///
    /// # Remarks
    ///
    /// Edges is inherently redundant.
    /// It is the responsibility of the caller to guarantee consistency.
    ///
    /// # Example
    ///
    /// ```
    /// use blossom::graph::AnnotatedGraph;
    /// let graph = AnnotatedGraph::new([
    ///   (0, (vec![1, 2], vec![0.1, 0.4])),
    ///   (2, (vec![0], vec![0.4])),
    ///   (1, (vec![0], vec![0.1]))
    /// ].iter().cloned().collect());
    /// ```
    pub fn new(edges: HashMap<Vertex, (Vec<Vertex>, Vec<Annotation>)>) -> Self {
        Self {
            vertices: edges.keys().cloned().collect(),
            edges,
        }
    }

    /// Gets a value indicating whether the graph is empty
    pub fn is_empty(&self) -> bool {
        self.vertices.is_empty()
    }

    /// Gets the number of vertices in the graph
    pub fn len(&self) -> usize {
        self.vertices.len()
    }

    /// Gets the vertices in the graph
    pub fn vertices(&self) -> &[Vertex] {
        &self.vertices
    }

    /// Gets the vertices adjacent to the given vertex
    pub fn vertices_from(&self, vertex: Vertex) -> &[Vertex] {
        &self.edges[&vertex].0
    }

    /// Gets the edges adjacent to the given vertex
    pub fn edges_from(&self, vertex: Vertex) -> (&[Vertex], &[Annotation]) {
        (&self.edges[&vertex].0, &self.edges[&vertex].1)
    }

    fn contract(&self, root: Vertex, leafs: &[Vertex]) -> Graph {
        let with_meta = |vertices: Vec<Vertex>| {
            let len = vertices.len();
            (vertices, iter::repeat(()).take(len).collect::<Vec<_>>())
        };
        let mut edges: HashMap<Vertex, (Vec<Vertex>, Vec<()>)> = self
            .edges
            .iter()
            .filter(|e| *e.0 != root && !leafs.contains(e.0))
            .map(|(&v, &(ref w, _))| {
                let mut has_root = false;
                let mut partners = Vec::with_capacity(w.len());
                for &p in w {
                    if p != root && !leafs.contains(&p) {
                        partners.push(p)
                    } else if !has_root {
                        has_root = true;
                        partners.push(root);
                    }
                }
                (v, with_meta(partners))
            })
            .collect();
        let mut root_partners: Vec<Vertex> = self
            .edges
            .iter()
            .filter(|e| *e.0 == root || leafs.contains(e.0))
            .flat_map(|(_, &(ref w, _))| w.iter().filter(|&p| *p != root && !leafs.contains(p)))
            .cloned()
            .collect();
        root_partners.sort_unstable();
        root_partners.dedup();
        edges.insert(root, with_meta(root_partners));
        Graph {
            vertices: self
                .vertices
                .iter()
                .filter(|&v| !leafs.contains(v))
                .cloned()
                .collect(),
            edges,
        }
    }

    fn trees(&self) -> Option<[Self; 2]> {
        if self.len() < 2 {
            return None;
        }

        let mut min = self.len();
        let mut vertex = 0;
        let mut edges = &vec![];
        for (&v, &(ref e, _)) in &self.edges {
            if e.is_empty() {
                return Some(self.split(&[v].iter().cloned().collect()));
            }
            if e.len() <= min {
                vertex = v;
                edges = e;
                min = e.len();
            }
        }

        let mut vertices: HashSet<Vertex> = HashSet::new();
        vertices.insert(vertex);
        let mut next = edges.iter().cloned().collect::<HashSet<_>>();
        loop {
            for &v in &next {
                vertices.insert(v);
            }
            if vertices.len() == self.len() {
                return None;
            }

            next = next
                .iter()
                .flat_map(|e| &self.edges[e].0)
                .filter(|v| !vertices.contains(v))
                .cloned()
                .collect();
            if next.is_empty() {
                return Some(self.split(&vertices));
            }
        }
    }

    /// Creates a new AnnotatedGraph with vertices filtered by given `predicate`.
    pub fn filter_vertices<P>(&self, predicate: P) -> Self
        where
            P: Fn(&Vertex) -> bool,
    {
        Self::new(
            self.edges
                .iter()
                .filter(|t| predicate(t.0))
                .map(|(v, &(ref e0, ref m0))| {
                    let (e1, m1): (Vec<_>, Vec<_>) =
                        e0.iter().zip(m0.iter()).filter(|t| predicate(t.0)).unzip();
                    (*v, (e1, m1))
                })
                .collect::<HashMap<_, _>>(),
        )
    }

    /// Creates a new AnnotatedGraph with edges filtered by given `predicate`.
    pub fn filter_edges<P>(&self, predicate: P) -> Self
        where
            P: Fn(&Vertex, &Vertex, &Annotation) -> bool,
    {
        Self::new(
            self.edges
                .iter()
                .map(|(v, &(ref e0, ref m0))| {
                    let (e1, m1): (Vec<_>, Vec<_>) = e0
                        .iter()
                        .zip(m0.iter())
                        .filter(|t| predicate(v, t.0, t.1))
                        .unzip();
                    (*v, (e1, m1))
                })
                .collect::<HashMap<_, _>>(),
        )
    }

    fn initial_matching(&self) -> Matching {
        let mut vertices_done: HashSet<Vertex> = HashSet::new();
        let mut matches = vec![];
        let mut edges: Vec<(Vertex, &Vec<Vertex>)> =
            self.edges.iter().map(|(&v, e)| (v, &e.0)).collect();
        edges.sort_unstable_by_key(|&(_, e)| e.len());
        for (v, ps) in edges {
            if !vertices_done.contains(&v) {
                if let Some(&p) = ps.iter().find(|x| !vertices_done.contains(x)) {
                    matches.push((v, p));
                    vertices_done.insert(v);
                    vertices_done.insert(p);
                }
            }
        }
        Matching::new(&matches[..])
    }

    fn exposed(&self, matching: &Matching) -> Vec<Vertex> {
        let mut matched = matching.vertices();
        matched.sort_unstable();
        self.vertices()
            .iter()
            .filter(|x| matched.binary_search(x).is_err())
            .cloned()
            .collect()
    }

    /// Determines a maximum matching in the current graph.
    ///
    /// Note: the (undirected) edges may be represented in
    /// reverse direction from the initial graph construction.
    ///
    /// # Example
    ///
    /// ```
    /// use blossom::Graph;
    /// let graph: Graph = [
    ///   (0, vec![1, 2, 3]),
    ///   (1, vec![0, 2]),
    ///   (2, vec![0, 1]),
    ///   (3, vec![0])
    /// ].iter().collect();
    /// let matching = graph.maximum_matching();
    /// let matching_edges = matching.edges();
    /// assert!(!matching_edges.contains(&(0, 1)) && !matching_edges.contains(&(1, 0)));
    /// assert!(matching_edges.contains(&(0, 3)) || matching_edges.contains(&(3, 0)));
    /// ```
    pub fn maximum_matching(&self) -> Matching {
        let mut matching = self.initial_matching();
        loop {
            let option = self.find_augmenting_path(&matching);
            if option.is_none() {
                break;
            }
            matching = matching.augment(&option.unwrap());
            if 2 * matching.len() == self.len() {
                break;
            }
        }
        matching
    }

    /// Determines a full matching in the current graph.
    /// If a full matching is not possible, None is returned.
    ///
    /// Note: the (undirected) edges may be represented in
    /// reverse direction from the initial graph construction.
    ///
    /// Note 2: in a graph with `2n+1` vertices,
    /// a matching consisting of `n` edges is considered full.
    pub fn full_matching(&self) -> Option<Matching> {
        if self.len() % 2 == 1 {
            return None;
        }

        if let Some(trees) = self.trees() {
            if trees.iter().any(|tree| tree.len() % 2 == 1) {
                return None;
            }

            let mut full = Matching::new(&[]);
            for tree in &trees {
                if let Some(matching) = tree.full_matching() {
                    full = full.add(&matching);
                } else {
                    return None;
                }
            }
            Some(full)
        } else {
            let matching = self.maximum_matching();
            if 2 * matching.len() == self.len() {
                Some(matching)
            } else {
                None
            }
        }
    }

    fn split(&self, vertices: &HashSet<Vertex>) -> [Self; 2] {
        [
            self.filter_vertices(|v| vertices.contains(v)),
            self.filter_vertices(|v| !vertices.contains(v)),
        ]
    }

    fn find_augmenting_path(&self, m: &Matching) -> Option<Vec<Vertex>> {
        let mut vertices_todo = self.exposed(m);
        let mut f = Forest::from_singletons(&vertices_todo);
        let mut vertices_done = HashSet::new();
        let mut temp = Forest::new();
        while let Some(v) = vertices_todo.pop() {
            let todo: Vec<Vertex> = self
                .vertices_from(v)
                .iter()
                .filter(|&w| !vertices_done.contains(w))
                .cloned()
                .collect();
            for &w in &todo {
                if let Some(wlen) = f.depth(w) {
                    if wlen % 2 == 1 {
                        let root: Vertex;
                        let vleafs: Vec<Vertex>;
                        let wleafs: Vec<Vertex>;
                        let leafs: Vec<Vertex>;
                        {
                            let vpath = f.find_path(v).unwrap();
                            let wpath = f.find_path(w).unwrap();
                            if vpath[0] != wpath[0] {
                                return Some(
                                    vpath.iter().chain(wpath.iter().rev()).cloned().collect(),
                                );
                            }

                            let common_len = vpath
                                .iter()
                                .zip(wpath.iter())
                                .take_while(|&(&v, &w)| v == w)
                                .count();
                            root = vpath[common_len - 1];
                            vleafs = vpath.iter().skip(common_len).cloned().collect();
                            wleafs = wpath.iter().skip(common_len).cloned().collect();
                            leafs = vleafs.iter().chain(wleafs.iter()).cloned().collect();
                        }
                        let gc = self.contract(root, &leafs);
                        let mc = m.contract(&leafs);
                        return gc
                            .find_augmenting_path(&mc)
                            .map(|path| self.lift(path, root, &vleafs, &wleafs));
                    }
                } else {
                    let x = m.partner(w);
                    let mut path: Vec<Vertex> = f.find_path(v).unwrap().to_vec();
                    path.push(w);
                    temp.push(w, path.clone());
                    path.push(x);
                    temp.push(x, path);
                    vertices_todo.push(x);
                }
                f.append(&mut temp);
            }
            vertices_done.insert(v);
        }
        None
    }

    fn lift(
        &self,
        mut path: Vec<Vertex>,
        root: Vertex,
        vleafs: &[Vertex],
        wleafs: &[Vertex],
    ) -> Vec<Vertex> {
        if let Some(root_position) = path.iter().position(|&x| x == root) {
            let after = root_position % 2 == 0;
            let match_position = if after {
                root_position + 1
            } else {
                root_position - 1
            };
            let match_vertex = path[match_position];
            let match_partners = self.vertices_from(match_vertex);
            if match_partners.iter().any(|&w| w == root) {
                return path;
            }
            let mut insert_position = root_position + (if after { 1 } else { 0 });
            for (l, &leafs) in (&[vleafs, wleafs]).iter().enumerate() {
                if let Some(n) = leafs.iter().position(|x| match_partners.contains(x)) {
                    let from_root: Vec<Vertex> = if n % 2 == 0 {
                        (if l == 0 { wleafs } else { vleafs })
                            .iter()
                            .chain(leafs.iter().skip(n).rev())
                            .cloned()
                            .collect()
                    } else {
                        leafs.iter().take(n + 1).cloned().collect()
                    };
                    for &leaf in &from_root {
                        path.insert(insert_position, leaf);
                        if after {
                            insert_position += 1;
                        }
                    }
                    return path;
                }
            }

            panic!(
                "Lift failed; path length = {}, branch lengths = {}/{}",
                path.len(),
                vleafs.len(),
                wleafs.len()
            )
        } else {
            path
        }
    }
}

impl<Annotation> FromIterator<(Vertex, (Vec<Vertex>, Vec<Annotation>))>
for AnnotatedGraph<Annotation>
    where
        Annotation: Copy + Sized,
{
    fn from_iter<I: IntoIterator<Item = (Vertex, (Vec<Vertex>, Vec<Annotation>))>>(
        iter: I,
    ) -> Self {
        Self::new(iter.into_iter().collect())
    }
}

impl<'a, Annotation> FromIterator<&'a (Vertex, (Vec<Vertex>, Vec<Annotation>))>
for AnnotatedGraph<Annotation>
    where
        Annotation: 'a + Copy + Sized,
{
    fn from_iter<I: IntoIterator<Item = &'a (Vertex, (Vec<Vertex>, Vec<Annotation>))>>(
        iter: I,
    ) -> Self {
        Self::new(iter.into_iter().cloned().collect())
    }
}

#[test]
fn contract() {
    let g: Graph = [(0, vec![1, 2]), (1, vec![0, 2]), (2, vec![0, 1])]
        .iter()
        .collect();
    let gc = g.contract(1, &vec![2]);
    assert_eq!(2, gc.vertices().len());
    assert_eq!(1, gc.vertices_from(0).len());
    assert_eq!(1, gc.vertices_from(0)[0]);
    assert_eq!(1, gc.vertices_from(1).len());
    assert_eq!(0, gc.vertices_from(1)[0]);
}

#[test]
fn contract_take_edge_from_leaf() {
    let g: Graph = [(0, vec![1]), (1, vec![0, 2]), (2, vec![1])]
        .iter()
        .collect();
    let gc = g.contract(2, &vec![1]);
    assert_eq!(2, gc.vertices().len());
    assert_eq!(1, gc.vertices_from(0).len());
    assert_eq!(2, gc.vertices_from(0)[0]);
    assert_eq!(1, gc.vertices_from(2).len());
    assert_eq!(0, gc.vertices_from(2)[0]);
}

#[test]
fn find_match_one() {
    let g: Graph = [(0, vec![1]), (1, vec![0])].iter().collect();
    let m = g.maximum_matching();
    verify_matching(&g, &m, 1);
}

#[test]
fn find_match_two() {
    let g: Graph = [(0, vec![1]), (1, vec![0]), (2, vec![3]), (3, vec![2])]
        .iter()
        .collect();
    let m = g.maximum_matching();
    verify_matching(&g, &m, 2);
}

#[test]
fn find_match_four() {
    let g: Graph = [
        (0, vec![1, 4]),
        (1, vec![0, 3]),
        (2, vec![3, 7]),
        (3, vec![1, 2, 5]),
        (4, vec![0, 5]),
        (5, vec![3, 4, 6, 7]),
        (6, vec![5]),
        (7, vec![2, 5]),
    ]
        .iter()
        .collect();
    let m = g.maximum_matching();
    verify_matching(&g, &m, 4);
}

#[cfg(test)]
fn verify_matching(g: &Graph, matching: &Matching, minimum: usize) {
    assert!(matching.len() >= minimum);
    assert!(matching.len() * 2 <= g.len());
    debug_assert!(matching
        .edges()
        .iter()
        .all(|&(a, b)| g.vertices_from(a).iter().any(|&e| e == b)));
}






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


use std::cmp::Ordering::{Equal, Greater, Less};
use std::f64;



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



