
#[cfg(test)]
mod tests {
    use log::info;
    use rand::{Rng, SeedableRng};
    use rand::rngs::StdRng;
    use crate::{christofidis, greedy_tour, nearest_neighbor_tour, utils};

    #[test]
    fn test_simple_graph () {
        let input = vec![(95.0129,61.5432),(23.1139,79.1937),(60.6843,92.1813),(48.5982,73.8207),(89.1299,17.6266),(76.2097,40.5706),(45.6468,93.5470),(1.8504,91.6904),(82.1407,41.0270),(44.4703,89.3650)];
        let graph = utils::Graph::new(&input);
        let displayGraph = utils::Graph::new(&input);
        for edges in displayGraph.edges {
            println!("{}", edges.iter().map(|&n| n.to_string()).collect::<Vec<String>>().join(","));
        }
        let result = christofidis(&graph);
    }
    #[test]
    fn test_small_graphs() {
        env_logger::builder()
            .filter_level(log::LevelFilter::Info)
            .init();
        let input = vec![vec![(95.0129,61.5432),(23.1139,79.1937),(60.6843,92.1813),(48.5982,73.8207),(89.1299,17.6266),(76.2097,40.5706),(45.6468,93.5470),(1.8504,91.6904),(82.1407,41.0270),(44.4703,89.3650)],
                         vec![(41.0, 49.0), (35.0, 17.0), (55.0, 45.0), (55.0, 20.0), (15.0, 30.0), (25.0, 30.0)],
                         vec![(0.0, 0.0), (0.0, 1.0), (1.0, 1.0), (1.0, 0.0)],
                         vec![(0.0, 0.0), (0.0, 1.0), (1.0, 1.0), (1.0, 0.0), (0.5, 0.5)],
                         vec![(0.0, 0.0), (0.0, 1.0), (1.0, 1.0), (1.0, 0.0), (0.5, 0.5), (0.5, 0.0)],
                         vec![(0.0, 0.0), (0.0, 1.0), (1.0, 1.0), (1.0, 0.0), (0.5, 0.5), (0.5, 0.0), (0.0, 0.5)],
                         vec![(0.0, 0.0), (0.0, 1.0), (1.0, 1.0), (1.0, 0.0), (0.5, 0.5), (0.5, 0.0), (0.0, 0.5), (1.0, 0.5)],
                         vec![(0.0, 0.0), (0.0, 1.0), (1.0, 1.0), (1.0, 0.0), (0.5, 0.5), (0.5, 0.0), (0.0, 0.5), (1.0, 0.5), (0.25, 0.25)],
                         vec![(0.0, 0.0), (0.0, 1.0), (1.0,1.0)]
            ];

        for i in 0..input.len() {
            let graph = utils::Graph::new(&input[i]);

            let compare = nearest_neighbor_tour(&input[i]);

            let result = christofidis(&graph);
            assert_eq!(result.len(), input[i].len());
            assert!(!has_duplicates(&result));

            let nn_distance = calculate_distance(&input[i], &compare);
            let greedy_distance = calculate_distance(&input[i], &result);
            // assert!(greedy_distance <= nn_distance);
            info!("{:?}: Greedy: {:?} < NN: {:?}",i, greedy_distance, nn_distance);
        }
    }

    #[test]
    fn test_big_graphs() {
        env_logger::builder()
            .filter_level(log::LevelFilter::Info)
            .init();
        info!("test");
        let input = vec![
            graph_builder(10),
            graph_builder(50),
            graph_builder(100),
            graph_builder(500),
            graph_builder(1000)
        ];

        for i in 0..input.len() {
            let graph = utils::Graph::new(&input[i]);

            let compare = nearest_neighbor_tour(&input[i]);
            info!("Graph size: {:?}", graph.num_nodes);
            let result = christofidis(&graph);
            assert_eq!(result.len(), input[i].len());
            assert!(!has_duplicates(&result));
            let nn_distance = calculate_distance(&input[i], &compare);
            let greedy_distance = calculate_distance(&input[i], &result);
            // assert!(greedy_distance <= nn_distance);
        }
    }

    fn calculate_distance(points: &Vec<(f64, f64)>, tour: &Vec<i32>) -> i32 {
        let mut dist = 0;
        for i in 0..tour.len() - 1 {
            dist += utils::euclidean_distance(points[tour[i]as usize], points[tour[i + 1] as usize]);
        }
        dist + utils::euclidean_distance(points[tour[0usize] as usize], points[tour[(tour.len() - 1)] as usize])
    }

    fn graph_builder(num_points: usize) -> Vec<(f64, f64)> {
        let mut rng = StdRng::seed_from_u64(201);
        let mut points = Vec::with_capacity(num_points);

        for _ in 0..num_points {
            let x = rng.gen_range(-1000000.0..1000000.0); // Adjust the range as needed
            let y = rng.gen_range(-1000000.0..1000000.0); // Adjust the range as needed
            let point = (x, y);
            points.push(point);
        }

        points
    }

    fn has_duplicates(tour: &Vec<i32>) -> bool {
        let mut used = vec![false; tour.len()];
        for i in tour {
            if used[*i as usize] {
                return true;
            }
            used[*i as usize] = true;
        }
        false
    }
}