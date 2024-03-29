use std::time::Instant;

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

    pub(crate) fn get_edges_sorted(&self) -> Vec<(i32, i32)> {
        let mut edges_vec: Vec<(i32, i32)> = Vec::new();
        for i in 0..self.num_nodes {
            for j in i+1..self.num_nodes {
                edges_vec.push((i, j));
            }
        }
        edges_vec.sort_by(|a, b| self.get_edge(a.0, a.1).cmp(&self.get_edge(b.0, b.1)));
        edges_vec.reverse();
        edges_vec
    }

    // Kruskal's algorithm
    pub(crate) fn get_min_spanning_tree(&self) -> SparseGraph {
        let mut sorted = self.get_edges_sorted();
        let mut sparse_graph = SparseGraph::new(self.num_nodes);

        let mut edge_count = 0;
        while !sorted.is_empty() && edge_count < self.num_nodes - 1 {
            let (x, y) = sorted.pop().unwrap();
            sparse_graph.add_edge(x, y);
            edge_count += 1;
            let circle_length = sparse_graph.get_circle(x);
            if circle_length >= 0 {
                sparse_graph.remove_edge(x, y);
                edge_count -= 1;
            }
        }
        sparse_graph
    }
}

pub struct SparseGraph {
    num_nodes: i32,
    pub(crate) adjacency_list: Vec<Vec<i32>>
}

impl SparseGraph {
    pub(crate) fn new(num_nodes: i32) -> SparseGraph {
        SparseGraph {
            num_nodes,
            adjacency_list: vec![Vec::new(); num_nodes as usize]
        }
    }

    pub(crate) fn add_edge(&mut self, x: i32, y: i32) {
        self.adjacency_list[x as usize].push(y);
        self.adjacency_list[y as usize].push(x);
    }

    pub(crate) fn remove_edge(&mut self, x: i32, y:i32) {
        let index = self.adjacency_list[x as usize].iter().position(|&r| r == y).unwrap();
        self.adjacency_list[x as usize].swap_remove(index);
        let index = self.adjacency_list[y as usize].iter().position(|&r| r == x).unwrap();
        self.adjacency_list[y as usize].swap_remove(index);
    }

    pub(crate) fn get_circle(&self, start_vertex: i32) -> i32 {
        let mut visited = vec![-1; self.num_nodes as usize];
        let mut stack = Vec::new();
        stack.push((start_vertex,start_vertex,0));
        while !stack.is_empty() {
            let (node, parent, index) = stack.pop().unwrap();
            // return circle length if node was already visited
            if visited[node as usize] >= 0 {
                return index - visited[node as usize];
            }
            visited[node as usize] = index;
            // push the neighbors of the current node to the stack
            if self.adjacency_list[node as usize].len() != 0 {
                for neighbor in &self.adjacency_list[node as usize] {
                    if *neighbor == parent {
                        continue;
                    } else {
                        stack.push((*neighbor, node, index + 1));
                    }
                }
            }

            // if the stack gets empty check if there is a node that has not been visited
            /*if stack.is_empty() && visited.iter().any(|&x| x == -1) {
                let unvisited = visited.iter().position(|&x| x == -1).unwrap() as i32;
                stack.push((unvisited, unvisited, 0));
            }*/

        }
        -1
    }

    pub(crate) fn get_vertex_degree(&self, x: i32) -> i32 {
        self.adjacency_list[x as usize].len() as i32
    }

    pub(crate) fn get_neighbors(&self, x: i32) -> Vec<i32> {
        self.adjacency_list[x as usize].clone()
    }

    pub(crate) fn get_edge(&self) -> Option<(i32, i32)> {
        for i in 0..self.num_nodes as usize {
            if self.adjacency_list[i].len() != 0 {
                return Some((i as i32, self.adjacency_list[i][0]));
            }
        }
        None
    }
}

pub fn euclidean_distance((x1, y1): (f64, f64), (x2, y2): (f64, f64)) -> i32 {
        ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt().round() as i32
}

pub fn two_opt(graph: &Graph, mut tour:Vec<i32>, start_time:Instant, max_processing_ms: u128) -> Vec<i32> {
    let mut improved = true;

    while improved &&  start_time.elapsed().as_millis() < max_processing_ms {
        improved = false;
        for i in 0..tour.len() - 1 {
            for j in i + 2..tour.len() - 1 {
                if j != i && j != i + 1 {

                    let old_1 = graph.get_edge(tour[i], tour[i+1]);
                    let old_2 = graph.get_edge(tour[j], tour[(j + 1) % tour.len()]);
                    let old_dist = old_1 + old_2;

                    let new_1 = graph.get_edge(tour[i], tour[j]);
                    let new_2 = graph.get_edge(tour[i+1], tour[(j + 1) % tour.len()]);
                    let new_dist = new_1 + new_2;
                    // tour[i + 1..=j].reverse();

                    if new_dist < old_dist {
                        tour[i + 1..=j].reverse();
                        improved = true;
                    }
                }
            }
        }
    }

    tour
}

pub fn three_opt(graph: &Graph, mut tour:Vec<i32>, start_time:Instant, max_processing_ms: u128) -> Vec<i32> {
    let mut improved = true;

    while improved &&  start_time.elapsed().as_millis() < max_processing_ms {
        improved = false;
        for i in 0..tour.len() - 1 {
            for j in i + 2..tour.len() - 1 {
                for k in j + 2..tour.len() - 1 {
                    if j != i && j != i + 1 && k != i && k != i + 1 && k != j && k != j + 1 {

                        let old_1 = graph.get_edge(tour[i], tour[i+1]);
                        let old_2 = graph.get_edge(tour[j], tour[(j + 1) % tour.len()]);
                        let old_3 = graph.get_edge(tour[k], tour[(k + 1) % tour.len()]);
                        let old_dist = old_1 + old_2 + old_3;

                        let mut best_dist = old_dist;
                        let mut flip:Vec<i32> = Vec::new();

                        // one-flip options
                        let new_1 = graph.get_edge(tour[i], tour[j]);
                        let new_2 = graph.get_edge(tour[i+1], tour[(j + 1) % tour.len()]);
                        let new_3 = graph.get_edge(tour[k], tour[(k + 1) % tour.len()]);
                        if (new_1 + new_2 + new_3) < best_dist {
                            best_dist = new_1 + new_2 + new_3;
                            flip = vec![0];
                        }

                        let new_1 = graph.get_edge(tour[i], tour[i+1]);
                        let new_2 = graph.get_edge(tour[j], tour[k]);
                        let new_3 = graph.get_edge(tour[(j + 1) % tour.len()], tour[(k + 1) % tour.len()]);
                        if (new_1 + new_2 + new_3) < best_dist {
                            best_dist = new_1 + new_2 + new_3;
                            flip = vec![1];
                        }

                        let new_1 = graph.get_edge(tour[i], tour[k]);
                        let new_2 = graph.get_edge(tour[(i + 1) % tour.len()], tour[(k + 1) % tour.len()]);
                        let new_3 = graph.get_edge(tour[j], tour[(j + 1) % tour.len()]);
                        if (new_1 + new_2 + new_3) < best_dist {
                            best_dist = new_1 + new_2 + new_3;
                            flip = vec![2];
                        }

                        // two-flip options
                        let new_1 = graph.get_edge(tour[i], tour[(j+1) % tour.len()]);
                        let new_2 = graph.get_edge(tour[(i + 1) % tour.len()], tour[(k + 1) % tour.len()]);
                        let new_3 = graph.get_edge(tour[k], tour[j]);
                        if (new_1 + new_2 + new_3) < best_dist {
                            best_dist = new_1 + new_2 + new_3;
                            flip = vec![1, 2];
                        }

                        let new_1 = graph.get_edge(tour[i], tour[k]);
                        let new_2 = graph.get_edge(tour[(i + 1) % tour.len()], tour[(j+1) % tour.len()]);
                        let new_3 = graph.get_edge(tour[j], tour[(k + 1) % tour.len()]);
                        if (new_1 + new_2 + new_3) < best_dist {
                            best_dist = new_1 + new_2 + new_3;
                            flip = vec![0, 2];
                        }

                        let new_1 = graph.get_edge(tour[i], tour[j]);
                        let new_2 = graph.get_edge(tour[(i + 1) % tour.len()], tour[k]);
                        let new_3 = graph.get_edge(tour[(j + 1) % tour.len()], tour[(k + 1) % tour.len()]);
                        if (new_1 + new_2 + new_3) < best_dist {
                            best_dist = new_1 + new_2 + new_3;
                            flip = vec![0, 1];
                        }

                        // three-flip options
                        let new_1 = graph.get_edge(tour[i], tour[(j+1) % tour.len()]);
                        let new_2 = graph.get_edge(tour[(i + 1) % tour.len()], tour[k]);
                        let new_3 = graph.get_edge(tour[j], tour[(k + 1) % tour.len()]);
                        if (new_1 + new_2 + new_3) < best_dist {
                            best_dist = new_1 + new_2 + new_3;
                            flip = vec![0, 1, 2];
                        }


                        // choose best and apply
                        if (best_dist <old_dist) {
                            for f in flip {
                                match f {
                                    0 => tour[i + 1..=j].reverse(),
                                    1 => tour[j + 1..=k].reverse(),
                                    2 => tour[i + 1..=k].reverse(),
                                    _ => panic!("Invalid flip")
                                }
                            }
                            improved = true;
                        }

                        if (start_time.elapsed().as_millis() >= max_processing_ms) {
                            return tour;
                        }


                    }
                }
            }
        }
    }

    tour
}

pub struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    const A: u64 = 1664525;
    const C: u64 = 1013904223;
    const M: u64 = 2u64.pow(32);

    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    pub fn next_u32(&mut self) -> u32 {
        self.state = (Self::A.wrapping_mul(self.state) + Self::C) % Self::M;
        self.state as u32
    }

    pub fn gen_range(&mut self, start: usize, end: usize) -> usize {
        start + (self.next_u32() as usize) % (end - start)
    }

    pub fn shuffle<T>(&mut self, slice: &mut [T]) {
        for i in (1..slice.len()).rev() {
            slice.swap(i, self.gen_range(0, i + 1));
        }
    }

    pub fn next_f64(&mut self) -> f64 {
        self.next_u32() as f64 / u32::MAX as f64
    }
}
