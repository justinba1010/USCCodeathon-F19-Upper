#![feature(vec_remove_item)]
#![allow(dead_code)]

use std::fs::File;
use std::io::prelude::*;
use std::ops::{Index, IndexMut};

use clap::{App, Arg};

use rand::random;


fn main() {
    let matches = App::new("biconnected_graph_generator")
        .version("0.1")
        .author("Jeremy Day <jaday@jaday.io>")
        .about("Generates biconnected graphs in adjacency list format.")
        .args(&[
            Arg::from_usage("-t, --tests <INTEGER> 'The number of test cases to generate. The remaining options apply to all test cases.'"),
            Arg::from_usage("-n, --nodes <INTEGER> 'The number of nodes to include in the graph'"),
            Arg::from_usage("-c, --components <INTEGER> 'The number of components to use when constructing the biconnected graph'"),
            Arg::from_usage("-o, --out <FILE> 'The file where the graph will be written'"),
        ])
        .get_matches();

    let t = matches.value_of("tests").unwrap().parse::<usize>().unwrap();
    let n = matches.value_of("nodes").unwrap().parse::<usize>().unwrap();
    let c = matches.value_of("components").unwrap().parse::<usize>().unwrap();
    let out = matches.value_of("out").unwrap();

    let mut file = File::create(out).unwrap();
    write!(file, "{}", t).unwrap();

    for _ in 0..t {
        let now = std::time::Instant::now();
        let (_, g) = make_biconnected_graph_from_components(n, c);
        let ms_create = now.elapsed().as_millis();
        println!("Created graph with {} nodes over {} components in {} ms.", n, c, ms_create);

        let file_bytes = file.metadata().unwrap().len();
        println!("The output file is: {} mb.", file_bytes as f32 / (1<<20) as f32);
        write!(file, "\n{}", g.format_adj_list()).unwrap();

        let now = std::time::Instant::now();
        let num_articulation_pts = g.count_biconnected().len();
        let ms = now.elapsed().as_millis();
        println!("This graph contains {} articulation points, which were calculated in {} ms.", num_articulation_pts, ms);
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NodeIndex(usize);

impl From<usize> for NodeIndex {
    fn from(i: usize) -> Self {
        Self(i)
    }
}

#[derive(Clone, Debug)]
pub struct Node {
    adjacency_list: Vec<NodeIndex>,
}

#[derive(Debug)]
pub struct Graph {
    nodes: Vec<Node>,
}

impl Index<NodeIndex> for Graph {
    type Output = Node;

    fn index(&self, index: NodeIndex) -> &Self::Output {
        &self.nodes[index.0]
    }
}

impl IndexMut<NodeIndex> for Graph {
    fn index_mut(&mut self, index: NodeIndex) -> &mut Self::Output {
        &mut self.nodes[index.0]
    }
}

impl Graph {
    fn empty() -> Self {
        Self {
            nodes: vec![],
        }
    }

    fn unit() -> Self {
        Self {
            nodes: vec![
                Node { adjacency_list: vec![], }
            ],
        }
    }

    fn last_node(&self) -> NodeIndex {
        NodeIndex(self.nodes.len() - 1)
    }

    fn first_node(&self) -> NodeIndex {
        NodeIndex(0)
    }

    fn format_adj_list(&self) -> String {
        let n_str = self.nodes.len().to_string();
        let g_str = self.nodes.iter().enumerate().map(|(i, node)| {
            let idxs = node.adjacency_list.iter().map(|NodeIndex(idx)| idx.to_string()).collect::<Vec<String>>().join(" ");
            format!("{} {}", i, idxs)
        }).collect::<Vec<String>>().join("\n");

        format!("{}\n{}", n_str, g_str)
    }

    fn joined_with(mut self, g2: &Graph, from: NodeIndex, to: NodeIndex) -> Graph {
        // We will offset each node index in the second graph by this amount in
        // order to keep the indexing scheme correct.
        let offset = self.nodes.len();

        let offset_nodes_iter = g2.nodes.iter().map(|node| Node {
            adjacency_list: node.adjacency_list.iter().map(|NodeIndex(idx)| {
                NodeIndex(idx + offset)
            }).collect()
        });

        // Add the nodes from our second graph to our first graph.
        self.nodes.extend(offset_nodes_iter);

        // Connect the two graphs through the specified edge.
        self.with_edge(from, NodeIndex(to.0 + offset))
    }

    fn add_edge(&mut self, from: NodeIndex, to: NodeIndex) {
        self.nodes[from.0].adjacency_list.push(to);
        self.nodes[to.0].adjacency_list.push(from);
    }

    fn remove_edge(&mut self, from: NodeIndex, to: NodeIndex) {
        self.nodes[from.0].adjacency_list.remove_item(&to);
        self.nodes[to.0].adjacency_list.remove_item(&from);
    }

    fn with_edge(mut self, from: NodeIndex, to: NodeIndex) -> Graph {
        self.add_edge(from, to);
        self
    }

    fn without_edge(mut self, from: NodeIndex, to: NodeIndex) -> Graph {
        self.remove_edge(from, to);
        self
    }

    /// Simple linear time connected check. A depth first search is performed
    /// and the number of nodes encountered is compared with the total number of
    /// nodes to see if the check if the search was exhaustive; if it was, the
    /// graph is connected.
    fn is_connected(&self) -> bool {
        if self.nodes.len() == 0 {
            return false;
        }

        // We keep track of which nodes we have visited.
        let mut num_visited = 0;
        let mut visited = vec![false; self.nodes.len()];

        // Maintain our DFS stack.
        let mut stack = vec![0];

        // Initialize our data structures for the first node.
        visited[0] = true;
        num_visited += 1;

        while stack.len() != 0 {
            let curr_idx = stack[stack.len() - 1];
            let mut recursing = false;

            for adj in &self.nodes[curr_idx].adjacency_list {
                // We are recursing another level.
                if !visited[adj.0] {
                    visited[adj.0] = true;
                    num_visited += 1;

                    recursing = true;
                    stack.push(adj.0);
                    break;
                }
            }

            if !recursing {
                stack.pop();
            }
        }

        num_visited == self.nodes.len()
    }

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

fn make_biconnected_graph(node_count: usize) -> (u32, Graph) {
    let mut tries = 1;
    let g = loop {
        let g = make_unconnected_graph(node_count);
        let g = add_random_edges(g, 0.00001 * (tries * tries) as f32);
        if g.is_connected() { break g } else { tries += 1 }
    };

    (tries, g)
}

fn make_biconnected_graph_from_components(node_count: usize, components: usize) -> (u32, Graph) {
    let node_count = node_count / components;

    let (mut tries_avg, mut g) = make_biconnected_graph(node_count);

    for _ in 1..components {
        let (tries, g_prime) = make_biconnected_graph(node_count);
        let from = g.last_node();
        let to = g_prime.first_node();
        g = g.joined_with(&g_prime, from, to);
        tries_avg += tries;
    }

    tries_avg /= components as u32;

    (tries_avg as u32, g)
}

fn make_unconnected_graph(node_count: usize) -> Graph {
    Graph {
        nodes: vec![ Node { adjacency_list: vec![], }; node_count]
    }
}

fn add_random_edges(mut g: Graph, edge_chance: f32) -> Graph {
    let node_count = g.nodes.len();

    for i in 0..node_count {
        for j in 0..node_count {
            if random::<f32>() < edge_chance {
                g.add_edge(i.into(), j.into())
            }
        }
    }

    g
}

#[test]
fn test_join() {
    let g1 = Graph {
        nodes: vec![
            Node {
                adjacency_list: vec![
                    NodeIndex(1),
                    NodeIndex(2),
                ],
            },
            Node {
                adjacency_list: vec![
                    NodeIndex(0),
                    NodeIndex(2),
                ],
            },
            Node {
                adjacency_list: vec![
                    NodeIndex(0),
                    NodeIndex(1),
                ],
            },
        ],
    };

    let articulation_point = Graph {
        nodes: vec![
            Node {
                adjacency_list: vec![],
            }
        ],
    };

    let g2 = Graph {
        nodes: vec![
            Node {
                adjacency_list: vec![
                    NodeIndex(1),
                    NodeIndex(2),
                ],
            },
            Node {
                adjacency_list: vec![
                    NodeIndex(0),
                    NodeIndex(2),
                ],
            },
            Node {
                adjacency_list: vec![
                    NodeIndex(0),
                    NodeIndex(1),
                ],
            },
        ],
    };

    let j1 = g1.joined_with(&articulation_point, NodeIndex(2), NodeIndex(0));
    let j2 = j1.joined_with(&g2, NodeIndex(3), NodeIndex(0));

    println!("{}", j2.format_adj_list());
    println!("{}", j2.is_connected());
    println!("{}", j2.without_edge(NodeIndex(3), NodeIndex(2)).is_connected());
}

#[test]
fn test_make_biconnected() {
    for i in 1..11 {
        let now = std::time::Instant::now();
        let (tries, g) = make_biconnected_graph(1000 * i);
        let ms_create = now.elapsed().as_millis();
        println!("{} nodes\t{} tries\t{} ms", 1000 * i, tries, ms_create);

        let now = std::time::Instant::now();
        let num_articulation_pts = g.count_biconnected().len();
        let ms = now.elapsed().as_millis();
        println!("+-- articulation points: {}\t ms: {}\n", num_articulation_pts, ms);
    }
}
