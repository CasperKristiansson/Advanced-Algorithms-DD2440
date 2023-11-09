mod utils;
//mod blossom;
mod mwmatching;
// mod lin_kernighan_opt;
// TODO always comment out before uploading solution
//mod test;


use std::collections::HashMap;
use std::io::{self};
use std::time::Instant;
use mwmatching::Matching;
// use log::info;
//use blossom::{Vertex, WeightedGraph, AnnotatedGraph};
use crate::utils::{Graph, three_opt};
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

fn greedy_tour(graph: &Graph, optimize: bool) -> Vec<i32> {
    let start_time = Instant::now();
    let n = graph.num_nodes;

    let mut sorted = graph.get_edges_sorted();
    let mut tour = vec![0; n as usize];
    let mut sparse_graph = SparseGraph::new(n);

    while !sorted.is_empty() {
        let (x, y) = sorted.pop().unwrap();
        if sparse_graph.get_vertex_degree(x) < 2 && sparse_graph.get_vertex_degree(y) < 2 {
            sparse_graph.add_edge(x, y);
            let circle_length = sparse_graph.get_circle(x);
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
    if !optimize {
        return tour.clone();
    }
    three_opt(graph, tour, start_time, 1950)
}

fn christofidis(graph: &Graph, optimize: bool) -> Vec<i32> {
    let start_time= Instant::now();
    if graph.num_nodes == 1 {
        return vec![0];
    }
    let mut prev_time = Instant::now();
    let mut spanning_tree: SparseGraph = graph.get_min_spanning_tree();
    // info!("Spanning tree: {:?}", Instant::now() - prev_time);

    prev_time = Instant::now();
    let odd_degree_nodes = spanning_tree.adjacency_list.iter().
        enumerate().filter(|(_, v)| v.len() % 2 == 1).map(|(i, _)| i).collect::<Vec<usize>>();


    let max = *graph.edges.iter().flat_map(|row| row.iter()).max().unwrap();
    let mut edges = Vec::new();
    for i in 0..odd_degree_nodes.len() {
        for j in i+1..odd_degree_nodes.len() {
            let length = graph.get_edge(odd_degree_nodes[i] as i32, odd_degree_nodes[j] as i32);
            edges.push((i,j,-length + max))
        }
    }

    let mates = Matching::new(edges).solve();

    for i in 0..mates.len() {
        if mates[i] < usize::MAX && mates[i] > i {
            spanning_tree.add_edge(odd_degree_nodes[i] as i32, odd_degree_nodes[mates[i]] as i32);
        }
    }

    prev_time = Instant::now();
    // for (x,y) in matching_edges {
    //     spanning_tree.add_edge(odd_degree_nodes[x] as i32,odd_degree_nodes[y] as i32);
    // }

    // Euler Tour
    let mut visited = vec![0; graph.num_nodes as usize];
    let mut tour:Vec<i32> = Vec::new();
    tour.push(0);
    while spanning_tree.get_edge().is_some() {
        let mut start = *tour.iter().find(|&&ele| spanning_tree.adjacency_list[ele as usize].len() != 0).unwrap();
        let mut y = spanning_tree.adjacency_list[start as usize][0];
        // if !tour.contains(&start) {
        //     let copy = start;
        //     start = y;
        //     y = copy;
        // }
        let mut counter = 1;
        let mut position = tour.iter().position(|&ele| ele == start).unwrap();
        let mut x = start;
        spanning_tree.remove_edge(x, y);
        while y != start {
            if visited[y as usize] != 1 {
                tour.insert(position + counter, y);
                visited[y as usize] = 1;
                counter += 1;
            }
            if spanning_tree.adjacency_list[y as usize].len() != 0 {
                (x,y) = (y, spanning_tree.adjacency_list[y as usize][0]);
                spanning_tree.remove_edge(x, y);
            }
        }
    }
    // info!("Euler tour: {:?}", Instant::now() - prev_time);
    if !optimize {
        return tour.clone();
    }
    three_opt(graph, tour, start_time, 1980)
}

fn main() {
    // Vector to hold all 2D points
    let mut points = Vec::new();

    let stdin = io::stdin();
    let mut num_vecs = String::new();
    stdin.read_line(&mut num_vecs).expect("error");
    let num = num_vecs.trim().parse().expect("error parsing number");
    for _ in 0..num {
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        let nums: Vec<f64> = line.split_whitespace()
            .map(|num| num.parse().expect("error"))
            .collect();

        points.push((nums[0], nums[1]));
    }
    // let result = greedy_tour(&points);

    let graph = Graph::new(&points);

    // for i in 0..graph.num_nodes {
    //     for j in 0..graph.num_nodes {
    //         println!("Edge between ({:?}) has length {:?}", (i,j), graph.get_edge(i, j));
    //     }
    // }


    // output
    for res in christofidis(&graph, true) {
        println!("{:?}", res);
    }
}
