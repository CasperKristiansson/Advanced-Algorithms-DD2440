mod lin_kernighan_opt {
    use std::mem::swap;
    use crate::utils::Graph;

    pub struct Lin<'a> {
        pub stack_2_opt_moves: Vec<(i32,i32)>,
        pub tour: Vec<i32>,
        pub graph: &'a Graph,
    }

    impl Lin {
        pub fn new(tour: Vec<i32>, graph: &Graph) -> Lin {
            Lin {
                stack_2_opt_moves: Vec::new(),
                tour,
                graph
            }
        }


        pub fn execute() {

        }

        pub fn predecessor(&self, node: i32) -> i32 {
            let index = self.tour.iter().position(|&r| r == node).unwrap();

            return if index == 0 {self.tour[self.tour.len() - 1]} else {self.tour[index - 1]};
        }

        pub fn successor(&self, node:i32)->i32 {
            let index = self.tour.iter().position(|&r| r == node).unwrap();

            return if index == self.tour.len() - 1 {self.tour[0]} else {self.tour[index + 1]};
        }

        pub fn between(&self, node: i32, before: i32, after: i32, direction: bool) -> bool {
            false
        }

        pub fn has_positive_gain(&self, (o1,o2): (i32, i32), (n1, n2): (i32, i32)) -> bool {
            self.graph.get_edge(o1,o2) - self.graph.get_edge(n1,n2) > 0
        }

        pub fn move_5_opt() {
            // todo max 5 2-opt moves
        }

        pub fn move_4_opt() {
            // todo max 3 2-opt moves
        }

        pub fn move_3_opt() {
            // todo max 3 2-opt moves
        }

        pub fn swap(self, i: i32, j:i32) {
            // todo max 1 2-opt move
        }


        pub fn undo(&mut self) {
            while let (x,y) = self.stack_2_opt_moves.pop().unwrap() {
                self.swap(x,y);
            }
        }

    }

}