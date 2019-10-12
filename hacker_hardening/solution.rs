use std::io::prelude::*;
use std::io::BufReader;
use std::io;

fn main() {
    let stdin = io::stdin();
    let buffered_stdin = BufReader::new(stdin);
    // Unwrap, assuming input is always in a valid form.
    let mut lines_iter = buffered_stdin.lines().map(|l| l.unwrap());

    // Unwrap, assuming input is always in a valid form.
    let t = lines_iter.next().unwrap().parse::<usize>().unwrap();

    // For each test case...
    for _ in 0..t {
        // The number of nodes in the graph; also the number of lines in this
        // test case.
        let n = lines_iter.next().unwrap().parse::<usize>().unwrap();
        let graph = graph_from_iter(lines_iter.by_ref().take(n));
        let mut articulation_points = graph.count_biconnected();
        articulation_points.sort();
        for (idx, pt) in articulation_points.iter().enumerate() {
            if idx != 0 { print!(", ") };
            print!("{}", pt);
        }
        println!();
    }
}


use crate::graph::*;
mod graph {
    pub fn graph_from_iter(i: impl Iterator<Item=String>) -> Graph {
        let nodes = i.map(|l| {
            let adjacency_list = l
                .split_whitespace()
                .skip(1)
                .map(|idx| NodeIndex(idx.parse::<usize>().unwrap()))
                .collect();
            Node { adjacency_list }
        }).collect();

        Graph { nodes }
    }

    #[derive(Debug)]
    pub struct NodeIndex(usize);

    #[derive(Debug)]
    pub struct Node {
        adjacency_list: Vec<NodeIndex>,
    }

    #[derive(Debug)]
    pub struct Graph {
        nodes: Vec<Node>,
    }

    impl Graph {
        pub fn count_biconnected(&self) -> Vec<usize> {
            let mut depths  = vec![-1; self.nodes.len()];
            let mut low_pts  = vec![-1; self.nodes.len()];
            let mut visited = vec![false; self.nodes.len()];
            let mut idx_stack = vec![0];
            let mut depth = 1;
            let mut articulation_pts = Vec::new();
            let mut children = vec![Vec::new(); self.nodes.len()];

            // Initialize for our first point.
            visited[0] = true;
            depths[0] = depth;
            low_pts[0] = depth;

            while idx_stack.len() > 0 {
                // Get the top node of the stack.
                let curr_idx = idx_stack[idx_stack.len() - 1];

                let adj = &self.nodes[curr_idx].adjacency_list;
                // Find the next node in the DFS tree that hasn't been visited.
                let next_idx = adj.iter().find(|n| !visited[n.0]);

                // If we found a node, we continue DFS further into the tree. If
                // we did not find a node, we pop our node off the stack and
                // work our way back up.
                if let Some(&NodeIndex(idx)) = next_idx {
                    // In preparation for the next level, we increase the depth.
                    depth += 1;

                    // We have now visited this node, so we mark it as such.
                    visited[idx] = true;

                    // We also know the depth of this node.
                    depths[idx] = depth;

                    // We initialize the low point for this node to be the
                    // depth.
                    low_pts[idx] = depth;

                    // We also track that the next node is an explicit child
                    // node (not just a neighbor) of the current node.
                    children[curr_idx].push(idx);

                    idx_stack.push(idx);
                    continue;
                } else {
                    if idx_stack.len() == 1 {
                        // We are finished and must handle the root node
                        // separately.
                        if children[idx_stack[0]].len() >= 2 {
                            articulation_pts.push(idx_stack[0]);
                        }
                        idx_stack.pop();
                        continue
                    }
                    let parent_idx = idx_stack[idx_stack.len() - 2];

                    // min of is the next low point:
                    //   depths[curr_idx]
                    //   depths[neighbors except parent]
                    //   lowpts[children in tree]

                    let curr_depth = depths[curr_idx];
                    let mut new_low = curr_depth;

                    let neighbors_iter = adj
                        .iter()
                        .filter(|&&NodeIndex(idx)| idx != parent_idx);
                    for neighbor_idx in neighbors_iter {
                        new_low = new_low.min(depths[neighbor_idx.0]);
                    }

                    let lowest_child_lowpt = children[curr_idx]
                        .iter()
                        .map(|&idx| low_pts[idx])
                        .min();

                    if let Some(val) = lowest_child_lowpt {
                        new_low = new_low.min(val);
                        if val >= curr_depth {
                            articulation_pts.push(curr_idx);
                        }
                    }

                    low_pts[curr_idx] = new_low;

                    // Now we pop the current index from the tree.
                    idx_stack.pop();

                    depth -= 1;
                }
            }

            articulation_pts
        }
    }
}
