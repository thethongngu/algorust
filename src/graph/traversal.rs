use std::collections::VecDeque;

// Adjacent list
type Graph = Vec<Vec<usize>>;

fn get_path(to: usize, trace: &Vec<Option<usize>>) -> Vec<usize> {
    let mut path = Vec::new();
    let mut u = to;
    path.push(u);
    while let Some(v) = trace[u] {
        path.push(v);
        u = v;
    }

    path.reverse();
    path
}

fn dfs_helper(graph: &Graph, u: usize, visited: &mut Vec<bool>, trace: &mut Vec<Option<usize>>) {
    visited[u] = true;
    for v in &graph[u] {
        if !visited[*v] {
            trace[*v] = Some(u);
            dfs_helper(graph, *v, visited, trace);
        }
    }
}

pub fn dfs(graph: &Graph, from: usize, to: usize) -> Vec<usize> {
    if from >= graph.len() || to >= graph.len() {
        return vec![];
    }

    let num_vertices = graph.len();
    let mut visited = vec![false; num_vertices];
    let mut trace = vec![None; num_vertices];

    dfs_helper(graph, from, &mut visited, &mut trace);
    get_path(to, &trace)
}

pub fn bfs(graph: &Graph, from: usize, to: usize) -> Vec<usize> {
    if from >= graph.len() || to >= graph.len() {
        return vec![];
    }

    let num_vertices = graph.len();
    let mut queue = VecDeque::new();
    let mut visited = vec![false; num_vertices];
    let mut trace = vec![None; num_vertices];

    for _ in 0..graph.len() {
        visited.push(false);
    }
    queue.push_back(from);
    visited[from] = true;

    while !queue.is_empty() {
        let u = queue.pop_front().unwrap();

        if u == to {
            break;
        }

        for v in &graph[u] {
            if !visited[*v] {
                queue.push_back(*v);
                visited[*v] = true;
                trace[*v] = Some(u);
            }
        }
    }

    get_path(to, &trace)
}

#[cfg(test)]
mod tests {
    use super::{bfs, dfs, Graph};

    #[test]
    fn normal() {
        let g: Graph = vec![
            vec![1],
            vec![0, 2],
            vec![1, 5, 4],
            vec![4, 6],
            vec![2, 3],
            vec![2, 6],
            vec![3, 5],
        ];

        assert_eq!(vec![0, 1, 2, 5, 6], bfs(&g, 0, 6));
        assert_eq!(vec![0, 1, 2, 5, 6], dfs(&g, 0, 6));
    }

    #[test]
    fn empty() {
        let g: Graph = vec![];

        assert_eq!(Vec::<usize>::new(), bfs(&g, 0, 6));
        assert_eq!(Vec::<usize>::new(), dfs(&g, 0, 6));
    }

    // TOOD: add test
    // - graph with cycle
    // - graph with no path
}
