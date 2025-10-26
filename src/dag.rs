use std::marker::PhantomData;

pub type Result<T> = std::result::Result<T, DagError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DagError {
    NodeNotFound,
    EdgeNotFound,
    InvalidNodeId,
    EdgeAlreadyExists,
    CycleDetected, // If cyclic dependency is detected, return Err(DagError::CycleDetected)
}

impl std::fmt::Display for DagError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DagError::NodeNotFound => write!(f, "Node Not Found"),
            DagError::EdgeNotFound => write!(f, "Edge Not Found"),
            DagError::InvalidNodeId => write!(f, "Invalid Node ID"),
            DagError::EdgeAlreadyExists => write!(f, "Edge Already Exists"),
            DagError::CycleDetected => write!(f, "Cycle Detected"),
        }
    }
}

impl std::error::Error for DagError {}
/**
* Iterators
* type NodeIter: Iterator<Item = NodeId>;
* type SuccessorIter: Iterator<Item = NodeId>;
* type PredecessorIter: Iterator<Item = NodeId>;
* type EdgeIter: Iterator<Item = (NodeId, NodeId)>;
*/

pub type NodeId = usize;

/**
 * Dag implementation trait
 * Node data: N
 * Edge data: E (default: unit type)
 */
pub trait DagImpl<N, E = ()> {
    // Node Operations
    fn add_node(&mut self, data: N) -> NodeId;
    fn remove_node(&mut self, node_id: NodeId) -> Result<()>;
    fn has_node(&self, node_id: NodeId) -> bool;
    fn node_count(&self) -> usize;

    // Edge Operations
    fn add_edge(&mut self, from: NodeId, to: NodeId) -> Result<()>;
    fn remove_edge(&mut self, from: NodeId, to: NodeId) -> Result<()>;
    fn has_edge(&self, from: NodeId, to: NodeId) -> bool;
    fn edge_count(&self) -> usize;

    // Data
    fn node_data(&self, node_id: NodeId) -> Result<&N>;
    fn node_data_mut(&mut self, node_id: NodeId) -> Result<&mut N>;
    fn edge_data(&self, from: NodeId, to: NodeId) -> Result<&E>;
    fn edge_data_mut(&mut self, from: NodeId, to: NodeId) -> Result<&mut E>;

    // Successor & Presdecessors
    fn successors(&self, node_id: NodeId) -> Result<Vec<NodeId>>;
    fn predecessors(&self, node_id: NodeId) -> Result<Vec<NodeId>>;

    // Degree
    fn indegree(&self, node_id: NodeId) -> Result<usize>;
    fn outdegree(&self, node_id: NodeId) -> Result<usize>;

    // Search and Sort
    fn topological_sort(&self) -> Result<Vec<NodeId>>;
    fn is_cyclic(&self) -> bool;
    fn source_nodes(&self) -> Vec<NodeId>;
    fn sink_nodes(&self) -> Vec<NodeId>;

    // Iterators
    type NodeIter: Iterator<Item = NodeId>;
    fn iter_nodes(&self) -> Self::NodeIter;

    type EdgeIter: Iterator<Item = (NodeId, NodeId)>;
    fn iter_edges(&self) -> Self::EdgeIter;

    type SuccessorIter: Iterator<Item = NodeId>;
    fn iter_successor(&self, node_id: NodeId) -> Result<Self::SuccessorIter>;

    type PredecessorIter: Iterator<Item = NodeId>;
    fn iter_predecessor(&self, node_id: NodeId) -> Result<Self::PredecessorIter>;

    // Aggregates
    fn all_nodes(&self) -> Vec<NodeId>;

    fn clear(&mut self);
}

/**
 * Node data: N
 */
pub struct Dag<N, E, B: DagImpl<N, E>> {
    backend: B,
    _phantom: PhantomData<(N, E)>,
}

impl<N, E, B: DagImpl<N, E> + Default> Dag<N, E, B> {
    pub fn new() -> Self {
        Self {
            backend: B::default(),
            _phantom: PhantomData,
        }
    }
}

impl<N, E, B: DagImpl<N, E> + Default> Default for Dag<N, E, B> {
    fn default() -> Self {
        Self::new()
    }
}

impl<N, E, B: DagImpl<N, E>> DagImpl<N, E> for Dag<N, E, B> {
    fn add_node(&mut self, data: N) -> NodeId {
        self.backend.add_node(data)
    }
    // If cyclic dependency is detected, return Err(DagError::CycleDetected)
    fn add_edge(&mut self, from: NodeId, to: NodeId) -> Result<()> {
        self.backend.add_edge(from, to)
    }
    fn remove_node(&mut self, node_id: NodeId) -> Result<()> {
        self.backend.remove_node(node_id)
    }
    fn remove_edge(&mut self, from: NodeId, to: NodeId) -> Result<()> {
        self.backend.remove_edge(from, to)
    }
    fn has_node(&self, node_id: NodeId) -> bool {
        self.backend.has_node(node_id)
    }
    fn has_edge(&self, from: NodeId, to: NodeId) -> bool {
        self.backend.has_edge(from, to)
    }

    fn successors(&self, node_id: NodeId) -> Result<Vec<NodeId>> {
        self.backend.successors(node_id)
    }
    fn predecessors(&self, node_id: NodeId) -> Result<Vec<NodeId>> {
        self.backend.predecessors(node_id)
    }

    fn all_nodes(&self) -> Vec<NodeId> {
        self.backend.all_nodes()
    }
    fn clear(&mut self) {
        self.backend.clear()
    }

    fn node_data(&self, node_id: NodeId) -> Result<&N> {
        self.backend.node_data(node_id)
    }

    fn indegree(&self, node_id: NodeId) -> Result<usize> {
        self.backend.indegree(node_id)
    }

    fn outdegree(&self, node_id: NodeId) -> Result<usize> {
        self.backend.outdegree(node_id)
    }

    fn topological_sort(&self) -> Result<Vec<NodeId>> {
        self.backend.topological_sort()
    }

    fn is_cyclic(&self) -> bool {
        self.backend.is_cyclic()
    }

    fn source_nodes(&self) -> Vec<NodeId> {
        self.backend.source_nodes()
    }

    fn sink_nodes(&self) -> Vec<NodeId> {
        self.backend.sink_nodes()
    }

    fn node_count(&self) -> usize {
        self.backend.node_count()
    }

    fn edge_count(&self) -> usize {
        self.backend.edge_count()
    }

    fn node_data_mut(&mut self, node_id: NodeId) -> Result<&mut N> {
        self.backend.node_data_mut(node_id)
    }

    fn edge_data(&self, from: NodeId, to: NodeId) -> Result<&E> {
        self.backend.edge_data(from, to)
    }

    fn edge_data_mut(&mut self, from: NodeId, to: NodeId) -> Result<&mut E> {
        self.backend.edge_data_mut(from, to)
    }

    type NodeIter = B::NodeIter;
    fn iter_nodes(&self) -> Self::NodeIter {
        self.backend.iter_nodes()
    }

    type EdgeIter = B::EdgeIter;
    fn iter_edges(&self) -> Self::EdgeIter {
        self.backend.iter_edges()
    }

    type SuccessorIter = B::SuccessorIter;
    fn iter_successor(&self, node_id: NodeId) -> Result<Self::SuccessorIter> {
        self.backend.iter_successor(node_id)
    }

    type PredecessorIter = B::PredecessorIter;
    fn iter_predecessor(&self, node_id: NodeId) -> Result<Self::PredecessorIter> {
        self.backend.iter_predecessor(node_id)
    }
}
