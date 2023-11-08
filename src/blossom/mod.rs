pub mod connect;
mod forest;
pub mod graph;
pub mod matching;
pub mod unweighted;
pub mod weighted;

pub type Vertex = usize;
pub type Edge = (Vertex, Vertex);

pub use self::matching::Matching;
pub use self::unweighted::Graph;
pub use self::weighted::WeightedGraph;
