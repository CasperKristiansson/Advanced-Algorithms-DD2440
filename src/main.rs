mod utils;

use std::collections::VecDeque;
use std::io::{self};
use crate::utils::Graph;

fn nearest_neighbor_tour(points: &Vec<(f64, f64)>) -> Vec<usize> {
    let n = points.len();
    let mut tour = vec![0; n];
    let mut used = vec![false; n];

    used[0] = true;

    for i in 1..n {
        let mut best = None;
        for j in 0..n {
            if !used[j] && (best.is_none() || utils::euclidean_distance(points[tour[i-1]], points[j]) < utils::euclidean_distance(points[tour[i-1]], points[best.unwrap()])) {
                best = Some(j);
            }
        }
        tour[i] = best.unwrap();
        used[tour[i]] = true;
    }

    tour
}

fn greedy_tour(graph: &Graph) -> Vec<usize> {
    let n = graph.num_nodes;


    for i in 0..n-1 {

    }
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

    // for res in result {
    //     println!("{:?}", res);
    // }
    let graph = utils::Graph::new(&points);

    for i in 0..graph.num_nodes {
        for j in 0..graph.num_nodes {
            println!("Edge between ({:?}) has length {:?}", (i,j), graph.get_edge(i, j));
        }
    }
}
