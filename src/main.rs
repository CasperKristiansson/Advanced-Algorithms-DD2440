mod utils;
mod test;

use std::io::{self};
use std::time::Instant;
use crate::utils::{euclidean_distance, Graph};
use crate::utils::SparseGraph;

fn nearest_neighbor_tour(points: &Vec<(f64, f64)>) -> Vec<i32> {
    let n = points.len();
    let mut tour:Vec<i32> = vec![0; n];
    let mut used = vec![false; n];

    used[0] = true;

    for i in 1..n {
        let mut best = None;
        for j in 0..n {
            if !used[j] && (best.is_none() || utils::euclidean_distance(points[tour[i-1] as usize], points[j]) < utils::euclidean_distance(points[tour[i-1] as usize], points[best.unwrap()])) {
                best = Some(j);
            }
        }
        tour[i] = best.unwrap() as i32;
        used[tour[i] as usize] = true;
    }

    tour
}

fn greedy_tour(graph: &Graph) -> Vec<i32> {
    let start_time = Instant::now();
    let n = graph.num_nodes;

    let mut sorted = graph.get_edges_sorted();
    let mut tour = vec![0; n as usize];
    let mut sparse_graph = SparseGraph::new(n);

    while !sorted.is_empty() {
        let (x, y) = sorted.pop().unwrap();
        if sparse_graph.get_vertex_degree(x) < 2 && sparse_graph.get_vertex_degree(y) < 2 {
            sparse_graph.add_edge(x, y);
            let circle_length = sparse_graph.contains_circle();
            if circle_length >= 0 && circle_length < n {
                sparse_graph.remove_edge(x, y);
            }
        }
    }

    // build tour from sparse graph
    tour[0] = 0;
    for i in 1..n as usize {
        let cur = sparse_graph.get_neighbors(tour[i-1]);
        tour[i] = if i > 1 && cur[1] != tour[i-2] {cur[1]} else {cur[0]};
    }

    // two-opt
    // let mut improved = true;
    // let mut iterations = 0;
    // let max_iterations = 5;
    //
    // while improved /* && iterations < max_iterations */ && start_time.elapsed().as_millis() < 1800 {
    //     improved = false;
    //     for i in 0..tour.len() - 1 {
    //         for j in i + 2..tour.len() {
    //             if j != i && j != i + 1 {
    //
    //                 let old_1 = sparse_graph.get_neighbors(tour[i]).iter().find(|&&(a, _)| a == tour[i+1]);
    //                 let old_2 = sparse_graph.get_neighbors(tour[j]).iter().find(|&&(a, _)| a == tour[(j + 1) % tour.len()]);
    //                 let old_dist = old_1.unwrap().1 + old_2.unwrap().1;
    //
    //                 let new_1 = sparse_graph.get_neighbors(tour[i]).iter().find(|&&(a, _)| a == tour[j]);
    //                 let new_2 = sparse_graph.get_neighbors(tour[i+1]).iter().find(|&&(a, _)| a == tour[(j + 1) % tour.len()]);
    //                 let new_dist = new_1.unwrap().1 + new_2.unwrap().1;
    //                 tour[i + 1..=j].reverse();
    //
    //                 if new_dist < old_dist {
    //                     tour[i + 1..=j].reverse();
    //                     improved = true;
    //                 }
    //             }
    //         }
    //     }
    //     iterations += 1;
    // }

    tour
}

fn main() {
    // Vector to hold all 2D points
    let mut points = Vec::new();

    let stdin = io::stdin();
    let mut num_vecs = String::new();
    stdin.read_line(&mut num_vecs).expect("error");
    let num = num_vecs.trim().parse().expect("error parsing int");
    for _ in 0..num {
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        let nums: Vec<f64> = line.split_whitespace()
            .map(|num| num.parse().expect("error"))
            .collect();

        points.push((nums[0], nums[1]));
    }
    // let result = greedy_tour(&points);

    let graph = utils::Graph::new(&points);

    // for i in 0..graph.num_nodes {
    //     for j in 0..graph.num_nodes {
    //         println!("Edge between ({:?}) has length {:?}", (i,j), graph.get_edge(i, j));
    //     }
    // }

    let tour = greedy_tour(&graph);



    // output
    for res in tour {
        println!("{:?}", res);
    }


}
