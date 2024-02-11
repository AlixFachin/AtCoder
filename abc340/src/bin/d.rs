use std::{collections::BinaryHeap, path, vec};
// use pathfinding::directed::dijkstra::dijkstra;

use proconio::input;
const DEBUG: bool = true;

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
struct PathData {
    node: usize,
    cost: usize,
}

impl PartialOrd for PathData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.cost.cmp(&self.cost))
    }
}

impl Ord for PathData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}


fn main() {
    input! {
       n: usize,
       levels: [(usize,usize,usize); n-1]
    }

    // let result = dijkstra(
    //     &0,
    //     |x| if *x == n-1 { vec!{} } else { vec![(*x + 1, levels[*x].0), (levels[*x].2-1, levels[*x].1)] },
    //     |x| *x == n - 1
    // );

    // Setup a clean adjacency list
    let mut adjacency_list = vec![vec![];n];
    for i in 0..(n-1) {
        let (a, b, x) = levels[i];
        adjacency_list[i].push(((i+1), a));
        adjacency_list[i].push(((x-1), b));
    }

    // Implementing Dijkstra's algorithm
    let mut dist = vec![std::usize::MAX; n];
    let mut queue: BinaryHeap<PathData> = std::collections::BinaryHeap::new();
    let mut visited = vec![false; n];
    dist[0] = 0;
    queue.push(PathData { node: 0, cost: 0 });
    while queue.len() > 0 {
        let path_data = queue.pop().unwrap();
        if visited[path_data.node] {
            continue;
        }
        visited[path_data.node] = true;
        for (v, w) in adjacency_list[path_data.node].iter() {
            if dist[*v] > path_data.cost + *w && !visited[*v] {
                dist[*v] = path_data.cost + *w;
                queue.push(PathData { node: *v, cost: dist[*v] });
            }
        }
    }

    println!("{}", dist[n-1]);

}
