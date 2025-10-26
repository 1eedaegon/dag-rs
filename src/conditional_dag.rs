use crate::DagImpl;

pub type EdgePredicate = Arc<dyn Fn() -> bool + Sync + Send>;

pub struct ConditionalDag<N, E, D: DagImpl<N, E>> {}
