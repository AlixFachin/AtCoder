use std::collections::{ HashSet, HashMap, VecDeque };
use proconio::input;

const DEBUG: bool = false;

/**
 * This is a DFS algorithm that will count the length of the longest path between two nodes
 */
fn _count_max_nodes(
    adjacency: &Vec<Vec<usize>>,
    from: usize,
    to: usize,
    visited: &mut HashSet<usize>
) -> usize {
    if from == to {
        return visited.len() + 1;
    }
    visited.insert(from);
    let mut max_distance = 0;
    for node in adjacency[from].iter() {
        if !visited.contains(&node) {
            let res = _count_max_nodes(adjacency, *node, to, visited);
            max_distance = max_distance.max(res);
        }
    }
    visited.remove(&from);
    return max_distance;
}

fn count_bfs_max_nodes(adjacency: &Vec<Vec<usize>>, from: usize, to: usize) -> usize {
    let mut max_dist_to_node: HashMap<usize, usize> = HashMap::new();
    let mut queue: VecDeque<usize> = VecDeque::new();
    let mut visited: HashSet<usize> = HashSet::new();
    max_dist_to_node.insert(from, 1);
    queue.push_back(from);
    while let Some(node) = queue.pop_front() {
        let &current_maximum_node = max_dist_to_node.get(&node).unwrap();
        for next_node in adjacency[node].iter() {
            if !max_dist_to_node.contains_key(next_node) {
                max_dist_to_node.insert(*next_node, current_maximum_node + 1);
            } else {
                let &previous_maximum_node = max_dist_to_node.get(&next_node).unwrap();
                max_dist_to_node.insert(
                    *next_node,
                    previous_maximum_node.max(current_maximum_node + 1)
                );
            }
            if !visited.contains(next_node) {
                queue.push_back(*next_node);
                visited.insert(*next_node);
            }
            if DEBUG {
                println!("Node: {}, max_dist: {}", next_node, max_dist_to_node.get(&node).unwrap());
                println!("Queue: {:?}", queue);
                println!("Max_dist_to_node: {:?}", max_dist_to_node);
            }
        }
        visited.insert(node);
    }
    if !max_dist_to_node.contains_key(&to) {
        return 0;
    }
    return max_dist_to_node[&to];
}

#[test]
fn test_dfs_max_node() {
    let adjacency_list = vec![vec![1, 2], vec![4], vec![3, 4], vec![4], vec![]];

    let mut visited: HashSet<usize> = HashSet::new();

    assert_eq!(_count_max_nodes(&adjacency_list, 0, 4, &mut visited), 4);
    assert_eq!(count_bfs_max_nodes(&adjacency_list, 0, 4), 4);
}

// Let's implement a topological sort algorithm
fn get_topological_sort(adjacency_list: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut visited = vec![false; adjacency_list.len()];
    let mut topological_sort = vec![];

    fn _topological_sort(
        adjacency_list: &Vec<Vec<usize>>,
        node: usize,
        visited: &mut Vec<bool>,
        topological_sort: &mut Vec<usize>
    ) {

        visited[node] = true;
        for next_node in &adjacency_list[node] {
            if !visited[*next_node] {
                _topological_sort(adjacency_list, *next_node, visited, topological_sort);
            }
        }

        topological_sort.push(node);
    }

    for i in 0..adjacency_list.len() {
        if !visited[i] {
            _topological_sort(adjacency_list, i, &mut visited, &mut topological_sort);
        }
    }
    topological_sort.reverse();
    return topological_sort;
}

#[test]
fn test_topological_sort() {
    let adjacency_list = vec![vec![1, 2], vec![4, 5], vec![3], vec![6], vec![5], vec![6], vec![]];
    assert_eq!(get_topological_sort(&adjacency_list), vec![0, 2, 3, 1, 4, 5, 6]);
}

fn main() {
    input! {
       n: usize,
       m: usize,
       weights: [usize; n],
       edges: [(usize, usize); m],
    }

    let mut connections: Vec<Vec<usize>> = vec![vec![]; n];
    // We will "clean up" the graph by removing the edges that are not increasing
    // And by merging all the nodes with a similar weight
    for (a, b) in &edges {
        // We will keep only the increasing (or equal) weights edges
        if weights[a - 1] <= weights[b - 1] {
            connections[a - 1].push(b - 1);
        }
        if weights[b - 1] <= weights[a - 1] {
            connections[b - 1].push(a - 1);
        }
    }

    if DEBUG {
        println!("Adjacency list before merge: {:?}", connections);
    }

    // Now that we built our adjacency list, we need to merge the nodes with similar weights
    // For this we use a visited list and do a DFS on each node to see which connected nodes have similar weights
    // We will store each root node into a root table
    let mut visited = vec![false; n];
    let mut root = vec![];
    for i in 0..n {
        root.push(i);
    }
    for i in 0..n {
        if !visited[i] {
            let mut stack = vec![i];
            while let Some(node) = stack.pop() {
                visited[node] = true;
                for next_node in &connections[node] {
                    if !visited[*next_node] && weights[node] == weights[*next_node] {
                        if DEBUG {
                            println!("Merging {} and {} through {}", i, *next_node, node);
                        }
                        stack.push(*next_node);
                        root[*next_node] = i;
                    }
                }
            }
        }
    }

    if DEBUG {
        println!("Root list: {:?}", root);
    }

    // Now we will replace in the adjacency list all the nodes by their roots
    let mut adjacency_list = vec![vec![]; n];
    for i in 0..n {
        for next_node in &connections[i] {
            if DEBUG {
                println!("{} -> {} becomes {} -> {}", i, next_node, root[i], root[*next_node]);
            }
            if root[i] != root[*next_node] && !adjacency_list[root[i]].contains(&root[*next_node]) {
                adjacency_list[root[i]].push(root[*next_node]);
            }
        }
    }

    if DEBUG {
        println!("Adjacency list after merge: {:?}", adjacency_list);
    }

    let top_order = get_topological_sort(&adjacency_list);
    if DEBUG {
        println!("Topological sort: {:?}", top_order);
    }

    let mut max_dist_to_node: Vec<i32> = vec![0; n];
    max_dist_to_node[0] = 1;
    for node in top_order {
        if DEBUG {
            println!("Checking distance with Node: {}", node);
        }
        for next_node in &adjacency_list[node] {
            if DEBUG {
                println!("Checking distance with Node: {} (MD={}), next_node: {} (MD={})", node, max_dist_to_node[node],  next_node, max_dist_to_node[*next_node]);
            }
            if max_dist_to_node[node] + 1 > max_dist_to_node[*next_node] {
                max_dist_to_node[*next_node] = max_dist_to_node[node] + 1 ;
                if DEBUG {
                    println!("Updated distance  of next_node: {} (MD={})",next_node, max_dist_to_node[*next_node]);
                }
            }
        }
    }

    println!("{}", max_dist_to_node[n - 1] );
}
