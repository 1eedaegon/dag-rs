mod adjacency_list_dag;

use crate::adjacency_list_dag::AdjListDag;

pub type Result<T> = std::result::Result<T, DagError>;
pub type NodeId = usize;

pub enum DagError {
    NodeNotFound,
    InvalidNodeId,
    EdgeAlreadyExists,
    CycleDetected,
}

/**
 * Node data: N
 * Edge data: E
 */
pub trait DagImpl<N, E> {
    fn add_node(&mut self, data: N) -> NodeId;
    /**
     * TODO: Implement check if exists for node and edges
     * Node and Edge Validation
     */
    // If cyclic dependency is detected, return Err(DagError::CycleDetected)
    fn add_edge(&mut self, from: NodeId, to: NodeId, data: E) -> Result<()>;
    fn remove_node(&mut self, node_id: NodeId) -> Result<()>;
    fn remove_edge(&mut self, from: NodeId, to: NodeId) -> Result<()>;
    fn has_node(&self, node_id: NodeId) -> bool;
    fn has_edge(&self, from: NodeId, to: NodeId) -> bool;
    /**
     * TODO: Successors and Predecessors
     */
    fn get_successors(&self, node_id: NodeId) -> Result<Vec<NodeId>>;
    fn get_predecessors(&self, node_id: NodeId) -> Result<Vec<NodeId>>;
    /**
     * TODO: Indegree and Outdegree
     */
    fn get_indegree(&self, node_id: NodeId) -> Result<usize>;
    fn get_outdegree(&self, node_id: NodeId) -> Result<usize>;
    /**
     * TODO: Topological Sort
     */
    fn topological_sort(&self) -> Result<Vec<NodeId>>;
    /**
     * TODO: Cyclic Detection
     */
    fn is_cyclic(&self) -> bool;
    /**
     * TODO: Source node and Sink node
     */
    fn get_source_node(&self) -> Result<NodeId>;
    fn get_sink_node(&self) -> Result<NodeId>;
    /**
     * TODO: get All Nodes
     */
    fn get_all_nodes(&self) -> Result<Vec<NodeId>>;
    fn clear(&mut self) -> Result<()>;

    /**
     * TODO: Data
     */
    fn add_node_with_data(&mut self, data: N) -> NodeId;
    fn add_edge_with_data(&mut self, from: NodeId, to: NodeId, data: E) -> Result<()>;
    fn get_node_data(&self, node_id: NodeId) -> Option<&N>;
    fn get_edge_data(&self, from: NodeId, to: NodeId) -> Option<&E>;
}

/**
 * Node data: N
 * Edge data: E
 */
pub struct Dag<N, E> {
    backend: Box<dyn DagImpl<N, E>>,
}
impl<N, E> Dag<N, E>
where
    N: 'static + Clone,
    E: 'static + Clone,
{
    pub fn new() -> Self {
        Self {
            backend: Box::new(AdjListDag::<N, E>::new()),
        }
    }
    pub fn add_node(&mut self, data: N) -> NodeId {
        self.backend.add_node(data)
    }
    /**
     * TODO: Implement check if exists for node and edges
     * Node and Edge Validation
     */
    // If cyclic dependency is detected, return Err(DagError::CycleDetected)
    pub fn add_edge(&mut self, from: NodeId, to: NodeId, data: E) -> Result<()> {
        self.backend.add_edge(from, to, data)
    }
    pub fn remove_node(&mut self, node_id: NodeId) -> Result<()> {
        self.backend.remove_node(node_id)
    }
    pub fn remove_edge(&mut self, from: NodeId, to: NodeId) -> Result<()> {
        self.backend.remove_edge(from, to)
    }
    pub fn has_node(&self, node_id: NodeId) -> bool {
        self.backend.has_node(node_id)
    }
    pub fn has_edge(&self, from: NodeId, to: NodeId) -> bool {
        self.backend.has_edge(from, to)
    }
    /**
     * TODO: Successors and Predecessors
     */
    pub fn get_successors(&self, node_id: NodeId) -> Result<Vec<NodeId>> {
        self.backend.get_successors(node_id)
    }
    pub fn get_predecessors(&self, node_id: NodeId) -> Result<Vec<NodeId>> {
        self.backend.get_predecessors(node_id)
    }
    /**
     * TODO: get All Nodes
     */
    pub fn get_all_nodes(&self) -> Result<Vec<NodeId>> {
        self.backend.get_all_nodes()
    }
    pub fn clear(&mut self) -> Result<()> {
        self.backend.clear()
    }

    pub fn add_node_with_data(&mut self, data: N) -> NodeId {
        self.backend.add_node_with_data(data)
    }
    pub fn add_edge_with_data(&mut self, from: NodeId, to: NodeId, data: E) -> Result<()> {
        self.backend.add_edge_with_data(from, to, data)
    }
    pub fn get_node_data(&self, node_id: NodeId) -> Option<&N> {
        self.backend.get_node_data(node_id)
    }
    pub fn get_edge_data(&self, from: NodeId, to: NodeId) -> Option<&E> {
        self.backend.get_edge_data(from, to)
    }
}

impl<N, E> Default for Dag<N, E>
where
    N: 'static + Clone,
    E: 'static + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}
