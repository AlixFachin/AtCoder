use itertools::Itertools;
use proconio::input;

struct Node {
    id: usize,
    neighbours: Vec<usize>,
}

impl Node {
    fn new(id: usize) -> Node {
        return Node {
            id: id,
            neighbours: vec![],
        };
    }

    fn add_neighbour(&mut self, c: usize) {
        self.neighbours.push(c);
    }

    fn get_subtree_size(&self, all_nodes: &Vec<Node>, origin_node_id: usize) -> usize {
        let down_neighbours: Vec<usize> = self.neighbours
            .iter()
            .map(|x| (*x).clone())
            .filter(|x| *x != origin_node_id)
            .collect();
        if down_neighbours.len() == 0 {
            return 1;
        }

        let subtrees_sizes: Vec<usize> = down_neighbours
            .iter()
            .map(|x| all_nodes[*x].get_subtree_size(all_nodes, self.id))
            .collect_vec();

        return subtrees_sizes.iter().sum::<usize>() + 1;
    }
}

fn main() {
    input! {
       n: usize,
       edges: [(usize,usize);n-1],
    }

    let mut all_nodes: Vec<Node> = vec![];
    for i in 0..=n {
        all_nodes.push(Node::new(i));
    }

    for (a, b) in edges {
        all_nodes[a].add_neighbour(b);
        all_nodes[b].add_neighbour(a);
    }

    let first_node = &all_nodes[1];
    if first_node.neighbours.len() == 1 {
        print!("1");
        return;
    }

    let sizes_list: Vec<usize> = first_node.neighbours
        .iter()
        .map(|x| all_nodes[*x].get_subtree_size(&all_nodes, 1))
        .collect_vec();
    // println!("Sizes list: {:?}", sizes_list);
    let max_subtree_size = sizes_list.iter().max().unwrap();
    println!("{}", n - *max_subtree_size);
}
