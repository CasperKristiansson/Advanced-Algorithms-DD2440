use std::time::{Duration, Instant};
use std::f64;
use std::io;


#[inline]
fn distance(p1: (f64, f64), p2: (f64, f64)) -> f64 {
    let dx = p1.0 - p2.0;
    let dy = p1.1 - p2.1;
    (dx * dx + dy * dy).sqrt()
}

fn initial_tour(points: &[(f64, f64)]) -> Vec<usize> {
    let num_points = points.len();
    let mut visited = vec![false; num_points];
    let mut tour = Vec::with_capacity(num_points);

    tour.push(0);
    visited[0] = true;

    while tour.len() < num_points {
        let last_visited = tour[tour.len() - 1];
        let mut nearest_neighbor = None;
        let mut min_distance = f64::MAX;

        for (i, &point) in points.iter().enumerate() {
            if !visited[i] {
                let dist = distance(points[last_visited], point);
                if dist < min_distance {
                    min_distance = dist;
                    nearest_neighbor = Some(i);
                }
            }
        }

        if let Some(nearest) = nearest_neighbor {
            tour.push(nearest);
            visited[nearest] = true;
        } else {
            break;
        }
    }

    tour
}

fn lin_kernighan_heuristic(points: &[(f64, f64)], tour: &mut Vec<usize>, time_limit: Duration) {
    let start_time = Instant::now();
    let num_points = tour.len();
    let check_interval = 50;  // Check time every 50 iterations

    while start_time.elapsed() < time_limit {
        let mut improvement = false;

        'outer: for i in 0..num_points {
            for j in (i + 2)..num_points {
                // Check the time less frequently to reduce overhead
                if j % check_interval == 0 && start_time.elapsed() >= time_limit {
                    return;
                }

                if two_opt_gain(&points, &tour, i, j) > 0.0 {
                    two_opt_swap(tour, i, j);
                    improvement = true;
                    break 'outer;
                }
            }
        }

        if !improvement {
            break;
        }
    }
}


fn two_opt_gain(points: &[(f64, f64)], tour: &[usize], i: usize, j: usize) -> f64 {
    let num_points = tour.len();
    let a = tour[i];
    let b = tour[(i + 1) % num_points];
    let c = tour[j];
    let d = tour[(j + 1) % num_points];

    let ab = distance(points[a], points[b]);
    let cd = distance(points[c], points[d]);
    let ac = distance(points[a], points[c]);
    let bd = distance(points[b], points[d]);

    (ab + cd) - (ac + bd) // Gain: negative means improvement
}

fn two_opt_swap(tour: &mut Vec<usize>, i: usize, j: usize) {
    if i < j {
        tour[i + 1..=j].reverse();
    } else {
        // Handle wrapping around the tour
        let mut temp = tour[i + 1..].to_vec();
        temp.extend_from_slice(&tour[..=j]);
        temp.reverse();
        let split = tour.len() - i - 1;
        tour[i + 1..].copy_from_slice(&temp[..split]);
        tour[..=j].copy_from_slice(&temp[split..]);
    }
}

fn total_distance(points: &[(f64, f64)], tour: &[usize]) -> f64 {
    let mut total_dist = 0.0;
    for i in 0..tour.len() {
        let curr_city = tour[i];
        let next_city = tour[(i + 1) % tour.len()]; // To wrap around to the first city
        total_dist += distance(points[curr_city], points[next_city]);
    }
    total_dist
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

    let start_time = Instant::now();

    let mut tour = initial_tour(&points);
    lin_kernighan_heuristic(&points, &mut tour, Duration::new(2, 0));

    let elapsed_time = start_time.elapsed();

    for &city in &tour {
        println!("{}", city);
    }
    
    eprintln!("Elapsed time: {}ms", elapsed_time.as_millis());
    eprintln!("Total distance: {}", total_distance(&points, &tour));
}