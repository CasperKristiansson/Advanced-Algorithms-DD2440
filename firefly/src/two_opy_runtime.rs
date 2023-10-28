use std::time::SystemTime;
use std::f64;
use std::io;

struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    const A: u64 = 1664525;
    const C: u64 = 1013904223;
    const M: u64 = 2u64.pow(32);

    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next_u32(&mut self) -> u32 {
        self.state = (Self::A.wrapping_mul(self.state) + Self::C) % Self::M;
        self.state as u32
    }

    fn gen_range(&mut self, start: usize, end: usize) -> usize {
        start + (self.next_u32() as usize) % (end - start)
    }

    fn shuffle<T>(&mut self, slice: &mut [T]) {
        for i in (1..slice.len()).rev() {
            slice.swap(i, self.gen_range(0, i + 1));
        }
    }

    fn next_f64(&mut self) -> f64 {
        self.next_u32() as f64 / u32::MAX as f64
    }
}

#[inline]
fn distance(p1: (f64, f64), p2: (f64, f64)) -> f64 {
    let dx = p1.0 - p2.0;
    let dy = p1.1 - p2.1;
    (dx * dx + dy * dy).sqrt()
}

fn compute_distance_matrix(points: &[(f64, f64)]) -> Vec<Vec<f64>> {
    let mut matrix = vec![vec![0.0; points.len()]; points.len()];
    for i in 0..points.len() {
        for j in i+1..points.len() {
            let dist = distance(points[i], points[j]);
            matrix[i][j] = dist;
            matrix[j][i] = dist;
        }
    }
    matrix
}

#[inline]
fn distance_from_matrix(matrix: &[Vec<f64>], i: usize, j: usize) -> f64 {
    matrix[i][j]
}

#[inline]
fn total_distance_from_matrix(path: &[usize], matrix: &[Vec<f64>]) -> f64 {
    let dist: f64 = path.windows(2)
        .map(|w| distance_from_matrix(&matrix, w[0], w[1]))
        .sum();
    dist + distance_from_matrix(&matrix, path[0], path[path.len() - 1])
}

fn move_firefly(rng: &mut SimpleRng, mut firefly_i: Vec<usize>, firefly_j: &[usize], matrix: &[Vec<f64>], beta0: f64, gamma: f64) -> Vec<usize> {
    let firefly_i_distance = total_distance_from_matrix(&firefly_i, matrix);
    let firefly_j_distance = total_distance_from_matrix(firefly_j, matrix);
    let dist_diff = firefly_i_distance - firefly_j_distance;
    let beta = beta0 * (-gamma * dist_diff).exp();

    for _ in 0..((beta * (firefly_i.len() as f64)) as usize) {
        let (i, k) = {
            let i = rng.gen_range(0, firefly_i.len());
            let k = rng.gen_range(0, firefly_i.len());
            if i < k { (i, k) } else { (k, i) }
        };

        firefly_i[i + 1..=k].reverse();
        let new_dist = total_distance_from_matrix(&firefly_i, matrix);

        if new_dist < firefly_i_distance || rng.next_f64() < (-gamma * (new_dist - firefly_i_distance)).exp() {
            break;
        } else {
            firefly_i[i + 1..=k].reverse();
        }
    }
    firefly_i
}

fn two_opt(path: &mut Vec<usize>, matrix: &[Vec<f64>], max_duration: std::time::Duration) {
    let mut improved = true;
    let start_time = SystemTime::now();

    while improved {
        improved = false;
        for i in 0..path.len() - 1 {
            for j in i + 2..path.len() {
                if j != i && j != i + 1 {
                    let old_dist = distance_from_matrix(&matrix, path[i], path[i + 1]) + 
                                   distance_from_matrix(&matrix, path[j], path[(j + 1) % path.len()]);
                    let new_dist = distance_from_matrix(&matrix, path[i], path[j]) + 
                                   distance_from_matrix(&matrix, path[i + 1], path[(j + 1) % path.len()]);
     
                    if new_dist < old_dist {
                        path[i + 1..=j].reverse();
                        improved = true;
                    }
                }
            }
        }
    
        if start_time.elapsed().unwrap() > max_duration {
            break;
        }
    }
}
   

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();
    stdin.read_line(&mut buffer).unwrap();
    let num_points = buffer.trim().parse::<usize>().unwrap();
    let mut points = Vec::with_capacity(num_points);

    for _ in 0..num_points {
        buffer.clear();
        stdin.read_line(&mut buffer).unwrap();
        let coords: Vec<f64> = buffer.split_whitespace().map(|s| s.parse().unwrap()).collect();
        points.push((coords[0], coords[1]));
    }

    let num_fireflies = 15;
    let mut fireflies: Vec<Vec<usize>> = Vec::with_capacity(num_fireflies);
    let beta0 = 1.0;
    let gamma = 0.1;

    let start_time = SystemTime::now();
    let max_duration = std::time::Duration::new(1, 900_000_000);
    let mut rng = SimpleRng::new(1698508300);

    for _ in 0..num_fireflies {
        let mut firefly: Vec<usize> = (0..num_points).collect();
        rng.shuffle(&mut firefly);
        fireflies.push(firefly);
    }

    let two_opt_time = std::time::Duration::new(0, 800_000_000 / num_fireflies as u32);

    let matrix = compute_distance_matrix(&points);

    for firefly in &mut fireflies {
        two_opt(firefly, &matrix, two_opt_time);
        if start_time.elapsed().unwrap() > max_duration {
            break;
        }
    }

    let mut best_firefly_index = 0;
    let mut best_distance = f64::INFINITY;

    while start_time.elapsed().unwrap() <= max_duration {
        let mut distances: Vec<f64> = fireflies.iter()
            .map(|f| total_distance_from_matrix(f, &matrix))
            .collect();

        for i in 0..num_fireflies {
            for j in 0..num_fireflies {
                if distances[i] > distances[j] {
                    fireflies[i] = move_firefly(&mut rng, fireflies[i].clone(), &fireflies[j], &matrix, beta0, gamma);
                    distances[i] = total_distance_from_matrix(&fireflies[i], &matrix);
                }
            }
        }

        for i in 0..num_fireflies {
            if distances[i] < best_distance {
                best_distance = distances[i];
                best_firefly_index = i;
            }
        }
    }

    let best_path = &fireflies[best_firefly_index];

    for city in best_path {
        println!("{}", city);
    }
}