use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use dag_rs::Dag;

const SIZES: [usize; 3] = [100, 500, 1000];

// Node addition performance measurement
fn bench_add_nodes(c: &mut Criterion) {
    let mut group = c.benchmark_group("add_nodes");
    for size in SIZES.iter() {
        group.bench_with_input(BenchmarkId::new("AdjListDag", size), size, |b, &s| {
            b.iter(|| {
                let mut dag = Dag::<usize, ()>::new();
                for i in 0..s {
                    dag.add_node(i);
                }
            });
        });
    }
    group.finish();
}

// Perf: Edge Addition
// Edge addition performance measurement (linear chain structure)
// Worst-case scenario for cycle detection
fn bench_add_edges_linear(c: &mut Criterion) {
    let mut group = c.benchmark_group("add_edges_linear");
    for size in SIZES.iter() {
        group.bench_with_input(BenchmarkId::new("AdjListDag", size), size, |b, &s| {
            b.iter_with_setup(
                || {
                    let mut dag = Dag::<usize, ()>::new();
                    let nodes: Vec<_> = (0..s).map(|i| dag.add_node(i)).collect();
                    (dag, nodes)
                },
                |(mut dag, nodes)| {
                    for i in 0..(s - 1) {
                        // TODO: 에러는 무시, 성능 측정에 집중
                        let _ = dag.add_edge(nodes[i], nodes[i + 1]);
                    }
                },
            );
        });
    }
    group.finish();
}

// Perf: Topological Sort
fn bench_topological_sort(c: &mut Criterion) {
    let mut group = c.benchmark_group("topological_sort");
    for size in SIZES.iter() {
        let mut dag = Dag::<usize, ()>::new();
        let nodes: Vec<_> = (0..*size).map(|i| dag.add_node(i)).collect();
        for i in 0..(*size - 1) {
            dag.add_edge(nodes[i], nodes[i + 1]).unwrap();
        }

        group.bench_with_input(BenchmarkId::new("AdjListDag", size), &dag, |b, d| {
            b.iter(|| d.topological_sort());
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_add_nodes,
    bench_add_edges_linear,
    bench_topological_sort
);
criterion_main!(benches);
