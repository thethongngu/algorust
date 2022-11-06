// Adjecent list
type Graph = Vec<Vec<usize>>;

fn dfs_helper(
    graph: &Graph,
    u: usize,
    color: &mut Vec<usize>,
    order: &mut Vec<usize>,
    is_dag: &mut bool,
) {
    if !(*is_dag) {
        return;
    }
    if color[u] != 0 {
        return;
    }

    color[u] = 1;
    for v in &graph[u] {
        if color[*v] == 1 {
            // contain cycle
            *is_dag = false;
            return;
        }

        if color[*v] == 0 {
            dfs_helper(graph, *v, color, order, is_dag);
        }
    }
    color[u] = 2;

    order.push(u);
}

/// Get the topological order of graph. If the graph is not DAG then return empty vector
pub fn toposort(graph: &Graph) -> Option<Vec<usize>> {
    let mut order = vec![];
    let num_vertices = graph.len();
    let mut color = vec![0; num_vertices];

    let mut is_dag = true;
    for u in 0..num_vertices {
        dfs_helper(graph, u, &mut color, &mut order, &mut is_dag);
    }

    if is_dag {
        order.reverse();
        Some(order)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{toposort, Graph};

    #[test]
    fn normal() {
        let g: Graph = vec![vec![2], vec![2], vec![], vec![1], vec![0, 1]];
        assert_eq!(vec![4, 3, 1, 0, 2], toposort(&g).unwrap());
    }

    #[test]
    fn empty() {
        let g: Graph = vec![];
        assert_eq!(Vec::<usize>::new(), toposort(&g).unwrap());
    }

    #[test]
    fn cycle() {
        let g: Graph = vec![vec![2], vec![0], vec![1], vec![2]];
        assert_eq!(None, toposort(&g));
    }
}
