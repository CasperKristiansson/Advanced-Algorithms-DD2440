use std::collections::VecDeque;

pub struct Graph {
    pub num_nodes: i32,
    // the index of the edge gives the two connected nodes
    // edge (x, y) is represented by edges[x * num_nodes + y - x - 1]
    // the value at that position corresponds to the length of the edge
    pub edges: Vec<Vec<i32>>
}

impl Graph {
    pub(crate) fn new(points: &Vec<(f64, f64)>) -> Graph {
        let num_nodes: i32 = points.len() as i32;
        let mut edges = Vec::new();
        for i in 0..(num_nodes - 1) as usize {
            edges.push(vec![0; (num_nodes - (i as i32 + 1)) as usize]);
            for j in i+1..num_nodes as usize {
                edges[i][j-(i+1)] = euclidean_distance(points[i], points[j]);
            }
        }
        Graph {
            num_nodes,
            edges
        }
    }

    pub(crate) fn get_edge(&self, x: i32, y: i32) -> i32 {
        if x == y {
            panic!("No edge between the same node");
        }
        if x > y {
            return self.edges[y as usize][(x-(y+1)) as usize];
        }
        self.edges[x as usize][(y-(x+1)) as usize]
    }

    pub(crate) fn get_vertices(&self, edge_index: i32) -> (i32, i32) {
        let x = edge_index / self.num_nodes;
        let y = edge_index % self.num_nodes;
        (x, y)
    }

    pub(crate) fn get_edges_sorted(&self) -> VecDeque<(i32, i32)> {
        let mut edges = VecDeque::new();
        for i in 0..self.num_nodes {
            for j in i+1..self.num_nodes {
                edges.push((i, j));
            }
        }
        edges.sort_by(|a, b| self.get_edge(a.0, a.1).cmp(&self.get_edge(b.0, b.1)));
        edges
    }
}

pub fn euclidean_distance((x1, y1): (f64, f64), (x2, y2): (f64, f64)) -> i32 {
        ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt().round() as i32
}
