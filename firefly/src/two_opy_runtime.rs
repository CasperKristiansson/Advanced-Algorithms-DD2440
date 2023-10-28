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

fn distance(p1: (f64, f64), p2: (f64, f64)) -> f64 {
    let dx = p1.0 - p2.0;
    let dy = p1.1 - p2.1;
    (dx * dx + dy * dy).sqrt()
}

fn total_distance(path: &[usize], points: &[(f64, f64)]) -> f64 {
    let dist: f64 = path.windows(2)
        .map(|w| distance(points[w[0]], points[w[1]]))
        .sum();
    dist + distance(points[path[0]], points[path[path.len() - 1]])
}

fn two_opt_swap(route: &[usize], i: usize, k: usize) -> Vec<usize> {
    let mut new_route = route[0..i].to_vec();
    new_route.extend(route[i..=k].iter().rev());
    new_route.extend(&route[k+1..]);
    new_route
}

fn move_firefly(rng: &mut SimpleRng, firefly_i: Vec<usize>, firefly_j: &[usize], points: &[(f64, f64)], beta0: f64, gamma: f64) -> Vec<usize> {
    let mut new_firefly_i = firefly_i.clone();
    let dist_diff = total_distance(&firefly_i, points) - total_distance(firefly_j, points);
    let beta = beta0 * (-gamma * dist_diff).exp();

    for _ in 0..((beta * (firefly_i.len() as f64)) as usize) {
        let (i, k) = {
            let i = rng.gen_range(0, firefly_i.len());
            let k = rng.gen_range(0, firefly_i.len());
            if i < k { (i, k) } else { (k, i) }
        };
        new_firefly_i = two_opt_swap(&new_firefly_i, i, k);
        let new_dist = total_distance(&new_firefly_i, points);
        let old_dist = total_distance(&firefly_i, points);

        if new_dist < old_dist || rng.next_f64() < (-gamma * (new_dist - old_dist)).exp() {
            return new_firefly_i;
        }
    }
    firefly_i
}

fn two_opt(path: &mut Vec<usize>, points: &[(f64, f64)], max_duration: std::time::Duration) {
    let mut improved = true;
    let start_time = SystemTime::now();

    while improved {
        improved = false;
        for i in 0..path.len() - 1 {
            for j in i + 2..path.len() {
                if j != i && j != i + 1 {
                    let old_dist = distance(points[path[i]], points[path[i + 1]]) + 
                                   distance(points[path[j]], points[path[(j + 1) % path.len()]]);
                    let new_dist = distance(points[path[i]], points[path[j]]) + 
                                   distance(points[path[i + 1]], points[path[(j + 1) % path.len()]]);
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

    let num_fireflies = 10;
    let mut fireflies: Vec<Vec<usize>> = Vec::with_capacity(num_fireflies);
    let beta0 = 1.0;
    let gamma = 0.1;

    let start_time = SystemTime::now();
    let max_duration = std::time::Duration::new(1, 900_000_000);
    let mut rng = SimpleRng::new(1698506886);

    for _ in 0..num_fireflies {
        let mut firefly: Vec<usize> = (0..num_points).collect();
        rng.shuffle(&mut firefly);
        fireflies.push(firefly);
    }

    let two_opt_time = std::time::Duration::new(0, 900_000_000 / num_fireflies as u32);

    for firefly in &mut fireflies {
        two_opt(firefly, &points, two_opt_time);
        if start_time.elapsed().unwrap() > max_duration {
            break;
        }
    }

    let mut best_firefly_index = 0;
    let mut best_distance = f64::INFINITY;

    while start_time.elapsed().unwrap() <= max_duration {
        let mut distances: Vec<f64> = fireflies.iter()
            .map(|f| total_distance(f, &points))
            .collect();

        for i in 0..num_fireflies {
            for j in 0..num_fireflies {
                if distances[i] > distances[j] {
                    fireflies[i] = move_firefly(&mut rng, fireflies[i].clone(), &fireflies[j], &points, beta0, gamma);
                    distances[i] = total_distance(&fireflies[i], &points);
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