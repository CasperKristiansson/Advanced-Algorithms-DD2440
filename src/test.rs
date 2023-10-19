
#[cfg(test)]
mod tests {
    use log::info;
    use crate::{greedy_tour, nearest_neighbor_tour, utils};

    #[test]
    fn test_main() {
        env_logger::builder()
            .filter_level(log::LevelFilter::Info)
            .init();
        let input = vec![vec![(95.0129,61.5432),(23.1139,79.1937),(60.6843,92.1813),(48.5982,73.8207),(89.1299,17.6266),(76.2097,40.5706),(45.6468,93.5470),(1.8504,91.6904),(82.1407,41.0270),(44.4703,89.3650)]];
        let expected = vec![vec![0, 8, 5, 4, 3, 9, 6, 2, 1, 7]];

        for i in 0..input.len() {


            let graph = utils::Graph::new(&input[i]);

            let compare = nearest_neighbor_tour(&input[i]);

            let result = greedy_tour(&graph);
            let nn_distance = calculate_distance(&input[i], &compare);
            let greedy_distance = calculate_distance(&input[i], &result);
            assert!(greedy_distance <= nn_distance);
            info!("{:?}: Greedy: {:?} < NN: {:?}",i, greedy_distance, nn_distance);
        }
    }

    fn calculate_distance(points: &Vec<(f64, f64)>, tour: &Vec<i32>) -> i32 {
        let mut dist = 0;
        for i in 0..tour.len() - 1 {
            dist += utils::euclidean_distance(points[tour[i]as usize], points[tour[i + 1] as usize]);
        }
        dist + utils::euclidean_distance(points[tour[0usize] as usize], points[tour[(tour.len() - 1)] as usize])
    }
}