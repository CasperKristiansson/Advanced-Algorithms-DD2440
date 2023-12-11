use std::time::{Duration, Instant};
use std::f64;
use std::io;


#[inline]
fn distance(p1: (f64, f64), p2: (f64, f64)) -> f64 {
    let dx = p1.0 - p2.0;
    let dy = p1.1 - p2.1;
    (dx * dx + dy * dy).sqrt()
}

#[inline]
fn squared_distance(p1: (f64, f64), p2: (f64, f64)) -> f64 {
    let dx = p1.0 - p2.0;
    let dy = p1.1 - p2.1;
    dx * dx + dy * dy
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
    let check_interval = 50;

    while start_time.elapsed() < time_limit {
        let mut improvement = false;

        'outer: for i in 0..num_points {
            for j in (i + 2)..num_points {
                for k in (j + 2)..num_points {
                    if k % check_interval == 0 && start_time.elapsed() >= time_limit {
                        return;
                    }

                    if three_opt_gain(points, tour, i, j, k) > 0.0 {
                        three_opt_swap(tour, i, j, k);
                        improvement = true;
                        break 'outer;
                    }
                }

                if j % check_interval == 0 && start_time.elapsed() >= time_limit {
                    return;
                }
            }
        }

        if !improvement {
            break;
        }
    }
}


fn three_opt_gain(points: &[(f64, f64)], tour: &[usize], i: usize, j: usize, k: usize) -> f64 {
    let num_points = tour.len();
    let a = tour[i];
    let b = tour[(i + 1) % num_points];
    let c = tour[j];
    let d = tour[(j + 1) % num_points];
    let e = tour[k];
    let f = tour[(k + 1) % num_points];

    let ab = squared_distance(points[a], points[b]);
    let cd = squared_distance(points[c], points[d]);
    let ef = squared_distance(points[e], points[f]);
    let ac = squared_distance(points[a], points[c]);
    let be = squared_distance(points[b], points[e]);
    let df = squared_distance(points[d], points[f]);

    (ab + cd + ef) - (ac + be + df)
}

fn three_opt_swap(tour: &mut Vec<usize>, i: usize, j: usize, k: usize) {
    let len = tour.len();

    if i < j && j < k {
        tour[i + 1..=j].reverse();
        tour[j + 1..=k].reverse();
        tour[i + 1..=k].reverse();
    } else {
        let mut new_tour = Vec::with_capacity(len);

        new_tour.extend_from_slice(&tour[0..=i]);
        if k < j {
            new_tour.extend(tour[k + 1..=j].iter().rev());
            new_tour.extend(tour[i + 1..=k].iter().rev());
            new_tour.extend(tour[j + 1..len].iter());
        } else {
            new_tour.extend(tour[j + 1..=k].iter().rev());
            new_tour.extend(tour[i + 1..=j].iter().rev());
            if k < len - 1 {
                new_tour.extend(&tour[k + 1..len]);
            }
        }
        *tour = new_tour;
    }
}

fn total_distance(points: &[(f64, f64)], tour: &[usize]) -> f64 {
    let mut total_dist = 0.0;
    for i in 0..tour.len() {
        let curr_city = tour[i];
        let next_city = tour[(i + 1) % tour.len()];
        total_dist += distance(points[curr_city], points[next_city]);
    }
    total_dist
}

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();

    let start_time = Instant::now();

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

    let remaining_time = Duration::new(2, 0)
        .checked_sub(start_time.elapsed())
        .and_then(|d| d.checked_sub(Duration::from_millis(100)))
        .unwrap_or_else(|| Duration::new(0, 0));

    lin_kernighan_heuristic(&points, &mut tour, remaining_time);

    let elapsed_time = start_time.elapsed();

    for &city in &tour {
        println!("{}", city);
    }
    
    eprintln!("Elapsed time: {}ms", elapsed_time.as_millis());
    eprintln!("Total distance: {}", total_distance(&points, &tour));
}