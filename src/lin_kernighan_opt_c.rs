use crate::utils::SimpleRng;
use crate::utils::Graph;
use std::time::{Duration, Instant};

pub struct Lin<'a> {
    pub tour: Vec<i32>,
    pub graph: &'a Graph,
    tabu_list: Vec<(i32, i32)>,
    max_tabu_size: usize,
}

impl<'a> Lin<'a> {
    pub fn new(tour: Vec<i32>, graph: &Graph) -> Lin {
        let num_nodes = graph.num_nodes as usize;
        let max_tabu_size = std::cmp::max(5, num_nodes / 10);
        Lin {
            tour,
            graph,
            tabu_list: Vec::new(),
            max_tabu_size,
        }
    }

    pub fn execute(&mut self) {
        if self.tour.is_empty() {
            let mut rng = SimpleRng::new(1698508300);
            self.initialize_random_tour(&mut rng);
        }

        let duration = std::time::Duration::from_millis(1900);
        let start_time = Instant::now();
        let mut iteration = 0;

        while start_time.elapsed() < duration {
            self.two_opt(iteration, &start_time, &duration);
            self.three_opt(iteration, &start_time, &duration);
            iteration += 1;
        }
    }

    pub fn initialize_random_tour(&mut self, rng: &mut SimpleRng) {
        let num_nodes = self.graph.num_nodes;
        self.tour = (0..num_nodes).collect();
        rng.shuffle(&mut self.tour);
    }

    pub fn two_opt(&mut self, iteration: usize, start_time: &Instant, duration: &Duration) {
        let max_duration = self.max_duration_per_operation(iteration);
        let mut improvement = true;

        while improvement && start_time.elapsed() < *duration {
            improvement = false;
            for i in 0..self.tour.len() - 1 {
                for j in i + 2..self.tour.len() {
                    if start_time.elapsed() >= *duration {
                        return;
                    }
                    if j == self.tour.len() - 1 && i == 0 {
                        continue;
                    }
                    if self.two_opt_swap(i, j) {
                        improvement = true;
                    }
                }
            }
        }
    }

    fn two_opt_swap(&mut self, i: usize, k: usize) -> bool {
        if self.tabu_list.contains(&(self.tour[i], self.tour[k])) {
            return false;
        }

        let edge_before = self.graph.get_edge(self.tour[i], self.tour[i + 1]) + self.graph.get_edge(self.tour[k], self.tour[(k + 1) % self.tour.len()]);
        let edge_after = self.graph.get_edge(self.tour[i], self.tour[k]) + self.graph.get_edge(self.tour[i + 1], self.tour[(k + 1) % self.tour.len()]);

        if edge_after >= edge_before {
            return false;
        }

        let tour_len_before = self.calculate_tour_length();
        self.tour[i + 1..=k].reverse();
        let tour_len_after = self.calculate_tour_length();

        if tour_len_after < tour_len_before {
            self.tabu_list.push((self.tour[i], self.tour[k]));
            if self.tabu_list.len() > self.max_tabu_size {
                self.tabu_list.remove(0);
            }
            true
        } else {
            false
        }
    }

    pub fn three_opt(&mut self, iteration: usize, start_time: &Instant, duration: &Duration) {
        let max_duration = self.max_duration_per_operation(iteration);
        let mut improvement = true;

        while improvement && start_time.elapsed() < *duration {
            improvement = false;
            for i in 0..self.tour.len() - 2 {
                for j in i + 2..self.tour.len() - 1 {
                    for k in j + 2..self.tour.len() {
                        if start_time.elapsed() >= *duration {
                            return;
                        }

                        if k == self.tour.len() - 1 && i == 0 {
                            continue;
                        }
                        if self.three_opt_swap(i, j, k) {
                            improvement = true;
                        }
                    }
                }
            }
        }
    }

    fn three_opt_swap(&mut self, i: usize, j: usize, k: usize) -> bool {
        if self.tabu_list.contains(&(self.tour[i], self.tour[j])) || self.tabu_list.contains(&(self.tour[j], self.tour[k])) {
            return false;
        }

        let edges_before = self.graph.get_edge(self.tour[i], self.tour[i + 1]) + self.graph.get_edge(self.tour[j], self.tour[j + 1]) + self.graph.get_edge(self.tour[k], self.tour[(k + 1) % self.tour.len()]);
        let edges_after = self.graph.get_edge(self.tour[i], self.tour[j]) + self.graph.get_edge(self.tour[j + 1], self.tour[k]) + self.graph.get_edge(self.tour[i + 1], self.tour[(k + 1) % self.tour.len()]);

        if edges_after >= edges_before {
            return false;
        }

        let tour_len_before = self.calculate_tour_length();

        let mut best_tour = self.tour.clone();
        let mut best_length = tour_len_before;

        self.try_reconnect(&mut best_tour, &mut best_length, i, j, k, 1);
        self.try_reconnect(&mut best_tour, &mut best_length, i, j, k, 2);
        self.try_reconnect(&mut best_tour, &mut best_length, i, j, k, 3);
        self.try_reconnect(&mut best_tour, &mut best_length, i, j, k, 4);
        self.try_reconnect(&mut best_tour, &mut best_length, i, j, k, 5);
        self.try_reconnect(&mut best_tour, &mut best_length, i, j, k, 6);
        self.try_reconnect(&mut best_tour, &mut best_length, i, j, k, 7);

        if best_length < tour_len_before {
            self.tabu_list.push((self.tour[i], self.tour[j]));
            self.tabu_list.push((self.tour[j], self.tour[k]));
            while self.tabu_list.len() > self.max_tabu_size {
                self.tabu_list.remove(0);
            }
            true
        } else {
            false
        }
    }

    fn max_duration_per_operation(&self, iteration: usize) -> Duration {
        let base_duration = Duration::from_millis(5);
        let factor = self.graph.num_nodes as u32;
        base_duration * (iteration as u32 + 1)
    }

    fn try_reconnect(&mut self, best_tour: &mut Vec<i32>, best_length: &mut i32, i: usize, j: usize, k: usize, case: u8) {
        let mut new_tour = self.tour.clone();
    
        match case {
            1 => {
                // No change, original tour
            },
            2 => {
                // Reverse the segment between i and j
                new_tour[i + 1..=j].reverse();
            },
            3 => {
                // Reverse the segment between j and k
                new_tour[j + 1..=k].reverse();
            },
            4 => {
                // Reverse both segments
                new_tour[i + 1..=j].reverse();
                new_tour[j + 1..=k].reverse();
            },
            5 => {
                // Swap segments
                new_tour[i + 1..=k].rotate_left(j - i);
            },
            6 => {
                // Reverse segment between i and j, then swap
                new_tour[i + 1..=j].reverse();
                new_tour[i + 1..=k].rotate_left(j - i);
            },
            7 => {
                // Reverse segment between j and k, then swap
                new_tour[j + 1..=k].reverse();
                new_tour[i + 1..=k].rotate_left(j - i);
            },
            _ => {}
        }
    
        let new_length = self.calculate_tour_length_for(&new_tour);
    
        if new_length < *best_length {
            *best_length = new_length;
            *best_tour = new_tour;
        }
    }

    fn calculate_tour_length(&self) -> i32 {
        let mut length = 0;
        for i in 0..self.tour.len() - 1 {
            length += self.graph.get_edge(self.tour[i], self.tour[i + 1]);
        }
        length += self.graph.get_edge(self.tour[self.tour.len() - 1], self.tour[0]);
        length
    }

    fn calculate_tour_length_for(&self, tour: &Vec<i32>) -> i32 {
        let mut length = 0;
        for i in 0..tour.len() - 1 {
            length += self.graph.get_edge(tour[i], tour[i + 1]);
        }
        length += self.graph.get_edge(tour[tour.len() - 1], tour[0]);
        length
    }
}