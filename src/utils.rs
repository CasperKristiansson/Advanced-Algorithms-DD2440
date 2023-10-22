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
}

pub struct SparseGraph {
    num_nodes: i32,
    adjacency_list: Vec<Vec<i32>>
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

    pub(crate) fn contains_circle(&self) -> i32 {
        let mut visited = vec![-1; self.num_nodes as usize];
        let mut stack = Vec::new();
        stack.push((0,0,0));
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
            if stack.is_empty() && visited.iter().any(|&x| x == -1) {
                let unvisited = visited.iter().position(|&x| x == -1).unwrap() as i32;
                stack.push((unvisited, unvisited, 0));
            }

        }
        -1
    }

    pub(crate) fn get_vertex_degree(&self, x: i32) -> i32 {
        self.adjacency_list[x as usize].len() as i32
    }

    pub(crate) fn get_neighbors(&self, x: i32) -> Vec<i32> {
        self.adjacency_list[x as usize].clone()
    }
}

pub fn euclidean_distance((x1, y1): (f64, f64), (x2, y2): (f64, f64)) -> i32 {
        ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt().round() as i32
}
