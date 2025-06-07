pub enum DagError {
    NodeNotFound,
    InvalidNodeId,
    EdgeAlreadyExists,
    EdgeNotFound,
    CycleDetected,
}

pub type Result<T> = std::result::Result<T, DagError>;
pub type NodeId = usize;

/**
 * Node data: N
 * Edge data: E
 */
pub trait Dag<N, E> {
    fn new(&mut self) -> Self
    where
        Self: Sized;
    fn add_node(&mut self) -> NodeId;
    /**
     * TODO: Implement check if exists for node and edges
     * Node and Edge Validation
     */
    fn add_edge(&mut self, from: NodeId, to: NodeId) -> Result<()>; // If cyclic dependency is detected, return Err(DagError::CycleDetected)
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
    fn add_edge_with_data(&mut self, from: NodeId, to: NodeId, data: E) -> Result<(), DagError>;
    fn get_node_data(&self, node_id: NodeId) -> Option<&N>;
    fn get_edge_data(&self, from: NodeId, to: NodeId) -> Option<&E>;
}
