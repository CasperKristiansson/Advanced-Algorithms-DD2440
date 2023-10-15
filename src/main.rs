use std::io;
use std::io::BufRead;

fn euclidean_distance((x1, y1): (f64, f64), (x2, y2): (f64, f64)) -> i64 {
    ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt().round() as i64
}

fn greedy_tour(points: &Vec<(f64, f64)>) -> Vec<usize> {
    let n = points.len();
    let mut tour = vec![0; n];
    let mut used = vec![false; n];

    used[0] = true;

    for i in 1..n {
        let mut best = None;
        for j in 0..n {
            if !used[j] && (best.is_none() || euclidean_distance(points[tour[i-1]], points[j]) < euclidean_distance(points[tour[i-1]], points[best.unwrap()])) {
                best = Some(j);
            }
        }
        tour[i] = best.unwrap();
        used[tour[i]] = true;
    }

    tour
}

fn main() {

    // Vector to hold all 2D points
    let mut points = Vec::new();

    // Get the standard input stream
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    // Read the first line to determine the number of points
    let mut first_line = String::new();
    match handle.read_line(&mut first_line) {
        Ok(_) => {
            // Parse the first line to get the number of points
            let num_points: i32 = first_line.trim().parse().expect("Expected a number on the first line");

            // Read 'num_points' lines from standard input and parse the points
            for _ in 0..num_points {
                let mut line = String::new();
                match handle.read_line(&mut line) {
                    Ok(_) => {
                        // Parse the line as a pair of floating-point numbers
                        let nums: Vec<f64> = line
                            .split_whitespace()
                            .map(|num| num.parse().expect("Expected a floating-point number"))
                            .collect();



                        // Add the tuple to the points vector
                        points.push((nums[0], nums[1]));
                    }
                    Err(err) => panic!("Error reading line: {}", err),
                }
            }
        }
        Err(err) => panic!("Error reading first line: {}", err),
    }
    let result = greedy_tour(&points);
    println!("{:?}", result);
}
