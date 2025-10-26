pub mod conditional_dag;
pub mod dag;
pub use dag::{Dag, DagError, DagImpl, NodeId, Result};

pub mod prelude {
    pub use super::{Dag, DagError, DagImpl, NodeId, Result};
}
