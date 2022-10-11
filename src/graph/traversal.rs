use std::collections::VecDeque;

type Graph = Vec<Vec<usize>>;

pub fn bfs(graph: &Graph, from: usize, to: usize) -> Vec<usize> {
    let num_vertices = graph.len();
    let mut queue = VecDeque::new();
    let mut visited = vec![false; num_vertices];
    let mut trace = vec![None; num_vertices];
    let mut path = vec![];

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

    let mut u = to;
    path.push(u);
    while let Some(v) = trace[u] {
        path.push(v);
        u = v;
    }

    path.reverse();
    path
}

#[cfg(test)]
mod tests {
    use super::{bfs, Graph};

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
    }
}
