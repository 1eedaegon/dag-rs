use dag_rs::Dag;

#[test]
fn test_add_and_has_node() {
    let mut dag = Dag::<&str, ()>::new();
    let n0 = dag.add_node("A");
    let n1 = dag.add_node("B");

    assert!(dag.has_node(n0));
    assert!(dag.has_node(n1));
    assert!(!dag.has_node(99));
    assert_eq!(dag.get_all_nodes().len(), 2);
}

#[test]
fn test_add_and_has_edge() {
    let mut dag = Dag::<(), &str>::new();
    let n0 = dag.add_node(());
    let n1 = dag.add_node(());
    let n2 = dag.add_node(());

    assert!(dag.add_edge(n0, n1, "n0->n1").is_ok());
    assert!(dag.add_edge(n1, n2, "n1->n2").is_ok());

    assert!(dag.has_edge(n0, n1));
    assert!(!dag.has_edge(n1, n0));
    assert_eq!(dag.get_indegree(n1).unwrap(), 1);
    assert_eq!(dag.get_outdegree(n1).unwrap(), 1);
    assert_eq!(dag.get_successors(n0).unwrap(), vec![n1]);
    assert_eq!(dag.get_predecessors(n2).unwrap(), vec![n1]);
}

#[test]
fn test_cycle_detection() {
    let mut dag = Dag::<(), ()>::new();
    let n0 = dag.add_node(());
    let n1 = dag.add_node(());
    let n2 = dag.add_node(());

    dag.add_edge(n0, n1, ()).unwrap();
    dag.add_edge(n1, n2, ()).unwrap();

    // n2 -> n0 간선은 n0 -> n1 -> n2 경로와 사이클을 형성합니다.
    assert_eq!(dag.add_edge(n2, n0, ()), Err(DagError::CycleDetected));
    // 자기 자신을 가리키는 간선도 사이클입니다.
    assert_eq!(dag.add_edge(n0, n0, ()), Err(DagError::CycleDetected));
}

#[test]
fn test_topological_sort() {
    let mut dag = Dag::<(), ()>::new();
    let n0 = dag.add_node(()); // Sources
    let n1 = dag.add_node(());
    let n2 = dag.add_node(());
    let n3 = dag.add_node(()); // Sinks

    //  n0 --> n1 --> n3
    //   \-> n2 --/
    dag.add_edge(n0, n1, ()).unwrap();
    dag.add_edge(n1, n3, ()).unwrap();
    dag.add_edge(n0, n2, ()).unwrap();
    dag.add_edge(n2, n3, ()).unwrap();

    let sorted = dag.topological_sort().unwrap();

    // 가능한 정렬: [0, 1, 2, 3] 또는 [0, 2, 1, 3]
    assert_eq!(sorted.len(), 4);
    assert_eq!(sorted[0], n0);
    assert_eq!(sorted[3], n3);
    assert!((sorted[1] == n1 && sorted[2] == n2) || (sorted[1] == n2 && sorted[2] == n1));
}

#[test]
fn test_remove_edge() {
    let mut dag = Dag::<(), ()>::new();
    let n0 = dag.add_node(());
    let n1 = dag.add_node(());

    dag.add_edge(n0, n1, ()).unwrap();
    assert!(dag.has_edge(n0, n1));
    assert_eq!(dag.get_indegree(n1).unwrap(), 1);

    assert!(dag.remove_edge(n0, n1).is_ok());
    assert!(!dag.has_edge(n0, n1));
    assert_eq!(dag.get_indegree(n1).unwrap(), 0);

    // 존재하지 않는 간선 삭제 시도
    assert_eq!(dag.remove_edge(n0, n1), Err(DagError::EdgeNotFound));
}

#[test]
fn test_remove_node() {
    let mut dag = Dag::<(), ()>::new();
    let n0 = dag.add_node(());
    let n1 = dag.add_node(());
    let n2 = dag.add_node(());

    // n0 -> n1 -> n2
    dag.add_edge(n0, n1, ()).unwrap();
    dag.add_edge(n1, n2, ()).unwrap();

    // n1을 삭제하면 n0 -> n1, n1 -> n2 간선이 모두 사라져야 합니다.
    assert!(dag.remove_node(n1).is_ok());

    assert!(!dag.has_node(n1));
    assert_eq!(dag.get_all_nodes().len(), 2);
    assert!(!dag.has_edge(n0, n1));
    assert!(!dag.has_edge(n1, n2));
    assert_eq!(dag.get_outdegree(n0).unwrap(), 0); // n0의 successor였던 n1이 사라짐
    assert_eq!(dag.get_indegree(n2).unwrap(), 0); // n2의 predecessor였던 n1이 사라짐

    // 삭제된 노드 ID를 재사용하는지 확인
    let n3 = dag.add_node(());
    assert_eq!(n3, n1); // 이전에 삭제된 노드 ID인 1을 재사용
    assert!(dag.has_node(n3));
    assert_eq!(dag.get_all_nodes().len(), 3);
}

#[test]
fn test_clear() {
    let mut dag = Dag::<(), ()>::new();
    let n0 = dag.add_node(());
    let n1 = dag.add_node(());
    dag.add_edge(n0, n1, ()).unwrap();

    dag.clear();
    assert_eq!(dag.get_all_nodes().len(), 0);
    assert!(!dag.has_node(n0));
    assert!(!dag.has_node(n1));
}
