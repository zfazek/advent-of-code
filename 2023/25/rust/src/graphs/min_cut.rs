#![allow(dead_code)]

use super::*;
use hashbag::HashBag;
use rand::{thread_rng, Rng};
use std::{
    collections::{hash_map, HashMap},
    ops::Div,
};

// ANCHOR: graphs_min_cut_super_edges
#[derive(Debug)]
pub struct SuperEdges {
    list: HashMap<Node, HashBag<NodeType>>,
    length: usize,
}

impl SuperEdges {
    pub fn get_random_edge(&self) -> Edge {
        let mut idx = thread_rng().gen_range(0..self.length);

        let mut iter = self.list.iter();
        if let Some((idx, &node, edges)) = loop {
            if let Some((node, edges)) = iter.next() {
                if idx < edges.len() {
                    break Some((idx, node, edges));
                }
                idx -= edges.len();
            } else {
                break None;
            }
        } {
            Edge(
                node,
                edges.iter().nth(idx).copied().unwrap_or_else(|| {
                    panic!(
                        "get_random_edge(): cannot get dst node at position({idx} from src({node})"
                    )
                }),
            )
        } else {
            panic!("get_random_edge(): Couldn't pick a random edge with index({idx}) from src")
        }
    }

    pub fn remove_edge(&mut self, src: Node, dst: Node) {
        // print!("remove_edge(): {:?} IN:{:?} -> Out:", edge, self);
        let edges = self.list.get_mut(&src).unwrap_or_else(|| {
            panic!("remove_edge(): Node({src}) cannot be found within SuperEdges")
        });
        self.length -= edges.contains(&dst.into());
        while edges.remove(&dst.into()) != 0 {}
        // println!("{:?}",self);
    }

    pub fn move_edges(&mut self, old: Node, new: Node) {
        // Fix direction OLD -> *
        let old_edges = self
            .list
            .remove(&old)
            .unwrap_or_else(|| panic!("move_edges(): cannot remove old node({old})"));
        // print!("move_edges(): {old}:{:?}, {new}:{:?}", old_edges,self.list[&new]);
        self.list
            .get_mut(&new)
            .unwrap_or_else(|| {
                panic!(
                    "move_edges(): failed to extend({new}) with {:?} from({new})",
                    old_edges
                )
            })
            .extend(old_edges.into_iter());

        // Fix Direction * -> OLD
        self.list
            .values_mut()
            .filter_map(|e| {
                let count = e.contains(&old.into());
                if count > 0 {
                    Some((count, e))
                } else {
                    None
                }
            })
            .for_each(|(mut count, edges)| {
                while edges.remove(&old.into()) != 0 {}
                while count != 0 {
                    edges.insert(new.into());
                    count -= 1;
                }
            });
        // println!(" -> {:?}",self.list[&new]);
    }
}
// ANCHOR_END: graphs_min_cut_super_edges
// ANCHOR: graphs_min_cut_super_nodes
#[derive(Debug)]
/// Helper Structure that holds the `set` of merged nodes under a super node `key`
/// The HashMap therefore is used as [Key:Super Node, Value: Set of Merged Nodes]
/// A super node's set is a `Graph Component` in itself, that is, you can visit a Node from any other Node within the set
pub struct SuperNodes {
    super_nodes: HashMap<Node, HashSet<Node>>,
}
impl Clone for SuperNodes {
    fn clone(&self) -> Self {
        SuperNodes {
            super_nodes: self.super_nodes.clone(),
        }
    }
}
impl SuperNodes {
    /// Total size of `Graph Components`, that is, super nodes
    pub fn len(&self) -> usize {
        self.super_nodes.len()
    }
    /// Given an Graph node, the function returns the Super Node that it belongs
    /// for example given the super node [Key:1, Set:{1,2,3,4,5}]
    /// querying for node `3` will return `1` as its super node
    pub fn find_supernode(&self, node: &Node) -> Node {
        // is this a super node ?
        if self.contains_supernode(node) {
            // if yes, just return it
            *node
        } else {
            // otherwise find its super node and return it
            // get an Iterator for searching each super node
            let mut sets = self.super_nodes.iter();
            loop {
                // If next returns [Super Node, Node Set] proceed otherwise exist with None
                let Some((&src, set)) = sets.next() else {
                    break None;
                };
                // Is the queried Node in the set ?
                if set.contains(node) {
                    // yes, return the super node
                    break Some(src);
                }
            }
            .unwrap_or_else(|| {
                panic!("find_supernode(): Unexpected error, cannot find super node for {node}")
            })
        }
    }
    /// Returns the graph component, aka `set` of nodes, for a given super node `key`,
    /// otherwise `None` if it doesn't exist
    pub fn contains_supernode(&self, node: &Node) -> bool {
        self.super_nodes.contains_key(node)
    }
    /// The function takes two super nodes and merges them into one
    /// The `dst` super node is merged onto the `src` super node
    pub fn merge_nodes(&mut self, src: Node, dst: Node) -> &mut HashSet<Node> {
        // remove both nodes that form the random edge and
        // hold onto the incoming/outgoing edges
        let super_src = self.super_nodes.remove(&src).unwrap();
        let super_dst = self.super_nodes.remove(&dst).unwrap();

        // combine the incoming/outgoing edges for attaching onto the new super-node
        let super_node = super_src
            .union(&super_dst)
            .copied()
            .collect::<HashSet<Node>>();
        // re-insert the src node as the new super-node and attach the resulting union
        self.super_nodes.entry(src).or_insert(super_node)
    }
    /// Provides an iterator that yields the Node Set of each super node
    pub fn iter(&self) -> SuperNodeIter {
        SuperNodeIter {
            iter: self.super_nodes.iter(),
        }
    }
}
/// Ability for SuperNode struct to use indexing for search
/// e.g super_node[3] will return the HashSet corresponding to key `3`
impl Index<Node> for SuperNodes {
    type Output = HashSet<Node>;
    fn index(&self, index: Node) -> &Self::Output {
        &self.super_nodes[&index]
    }
}

/// HashNode Iterator structure
pub struct SuperNodeIter<'a> {
    iter: hash_map::Iter<'a, Node, HashSet<Node>>,
}

/// HashNode Iterator implementation yields a HashSet at a time
impl<'a> Iterator for SuperNodeIter<'a> {
    type Item = &'a HashSet<Node>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(super_node) = self.iter.next() {
            Some(super_node.1)
        } else {
            None
        }
    }
}
// ANCHOR_END: graphs_min_cut_super_nodes
// ANCHOR: graphs_min_cut_super_edges_graph
/// Helper Graph functions
impl Graph {
    /// SuperEdges Constructor
    pub fn get_super_edges(&self) -> SuperEdges {
        let mut length = 0;
        let list = self
            .edges
            .iter()
            .map(|(&n, e)| (n, e.iter().copied().collect::<HashBag<NodeType>>()))
            .inspect(|(_, c)| length += c.len())
            .collect();
        // println!("get_super_edges(): [{length}]{:?}",list);
        SuperEdges { list, length }
    }
    /// SuperNodes Constructor
    pub fn get_super_nodes(&self) -> SuperNodes {
        SuperNodes {
            super_nodes: self
                .nodes
                .iter()
                .map(|&node| (node, HashSet::<Node>::new()))
                .map(|(node, mut map)| {
                    map.insert(node);
                    (node, map)
                })
                .collect::<HashMap<Node, HashSet<Node>>>(),
        }
    }
}
// ANCHOR_END: graphs_min_cut_super_edges_graph

pub trait MinimumCut {
    fn minimum_cut(&self) -> Option<Graph>;
    fn contract_graph(&self) -> Option<Graph>;
    fn get_crossing_edges(&self, src_set: &HashSet<Node>, dst_set: &HashSet<Node>) -> Graph;
}

impl MinimumCut for Graph {
    //noinspection RsExternalLinter
    // ANCHOR: graphs_min_cut
    fn minimum_cut(&self) -> Option<Graph> {
        // calculate the number of iterations as N*log(N)
        let nodes = self.nodes.len();
        let mut iterations = 1 * nodes as u32 * nodes.ilog2();
        println!("Run Iterations: {iterations}");

        // initialise min-cut min value and output as Option
        let mut min_cut = usize::MAX;
        let mut result = None;
        let repetitions = iterations as f32;

        // iterate N*log(N) time or exit if min-cut found has only 2 edges
        let mut f = f32::MAX;
        while iterations != 0 && f > 0.0 {
            // contract the graph
            if let Some(graph) = self.contract_graph() {
                // extract the number of edges
                let edges = graph.export_edges();
                // count the edges
                let edges = edges.len();

                // if number of edges returned is smaller than current
                // then store the min-cut returned from this iteration
                if edges < min_cut {
                    min_cut = edges;
                    result = Some(graph);
                    f = (min_cut as f32).div(repetitions);
                    println!("({iterations})({f:.3}) Min Cut !! => {:?}", edges);
                }
                if min_cut == 6 {
                    break;
                }
            }
            iterations -= 1;
        }
        result
    }
    // ANCHOR_END: graphs_min_cut

    // ANCHOR: graphs_contraction
    fn contract_graph(&self) -> Option<Graph> {
        if self.edges.is_empty() {
            return None;
        }

        // STEP 1: INITIALISE temporary super node and super edge structures
        let mut super_edges = self.get_super_edges();
        let mut super_nodes = self.get_super_nodes();

        // STEP 2: CONTRACT the graph, until 2 super nodes are left
        while super_nodes.len() > 2 {
            // STEP A: select a random edge
            // get a copy rather a reference so we don't upset the borrow checker
            // while we deconstruct the edge into src and dst nodes
            let Edge(src, dst) = super_edges.get_random_edge();
            // println!("While: E({src},{dst}):{:?}",super_edges.list);

            // STEP B : Contract the edge by merging the edge's nodes
            // remove both nodes that form the random edge and
            // hold onto the incoming/outgoing edges
            // combine the incoming/outgoing edges for attaching onto the new super-node
            // re-insert the src node as the new super-node and attach the resulting union
            super_nodes.merge_nodes(src, dst.into());

            // STEP C : Collapse/Remove newly formed edge loops since src & dst is the new super node
            super_edges.remove_edge(src, dst.into());
            super_edges.remove_edge(dst.into(), src);

            // STEP D : Identify all edges that still point to the dst removed as part of collapsing src and dst nodes
            // STEP E : Repoint all affected edges to the new super node src
            super_edges.move_edges(dst.into(), src);
        }

        // STEP 3 : find the edges between the two super node sets
        let mut snode_iter = super_nodes.iter();
        Some(self.get_crossing_edges(
            snode_iter.next().expect("There is no src super node"),
            snode_iter.next().expect("There is no dst super node"),
        ))
    }
    // ANCHOR_END: graphs_contraction

    // ANCHOR: graphs_crossing
    /// Given two Super Node sets the function returns the crossing edges as a new Graph structure
    fn get_crossing_edges(&self, src_set: &HashSet<Node>, dst_set: &HashSet<Node>) -> Graph {
        src_set
            .iter()
            .map(|src| {
                (
                    src,
                    // get src_node's edges from the original graph
                    self.edges
                        .get(src)
                        .unwrap_or_else(|| {
                            panic!("get_crossing_edges(): cannot extract edges for node({src}")
                        })
                        .iter()
                        .map(|&ntype| ntype.into())
                        .collect::<HashSet<Node>>(),
                )
            })
            .map(|(src, set)|
                // Keep only the edges nodes found in the dst_set (intersection)
                // we need to clone the reference before we push them
                // into the output graph structure
                (src, set.intersection(dst_set).copied().collect::<HashSet<Node>>()))
            .filter(|(_, edges)| !edges.is_empty())
            .fold(Graph::new(), |mut out, (&src, edges)| {
                // println!("Node: {node} -> {:?}",edges);
                // add edges: direction dst -> src
                edges.iter().for_each(|&dst| {
                    out.nodes.insert(dst);
                    out.edges.entry(dst).or_default().insert(src.into());
                });
                // add edges: direction src -> dst
                out.nodes.insert(src);
                out.edges
                    .insert(src, edges.into_iter().map(|edge| edge.into()).collect());
                out
            })
        // println!("Crossing graph: {:?}", output);
    }
    // ANCHOR_END: graphs_crossing
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::graphs::Graph;

    #[test]
    fn test_min_cut() {
        // test dataset: Array[ (input_graph, minimum expected edges) ]
        let adj_list: Vec<(Vec<Vec<Node>>, usize)> = vec![
            (
                vec![
                    vec![1, 2, 4, 3],
                    vec![2, 3, 1, 4, 5],
                    vec![3, 4, 2, 8, 1],
                    vec![4, 1, 3, 2],
                    vec![5, 6, 8, 7, 2],
                    vec![6, 7, 5, 8],
                    vec![7, 8, 6, 5],
                    vec![8, 5, 3, 7, 6],
                ],
                4,
            ),
            (
                vec![
                    vec![1, 2, 3, 4, 7],
                    vec![2, 1, 3, 4],
                    vec![3, 1, 2, 4],
                    vec![4, 1, 2, 3, 5],
                    vec![5, 4, 6, 7, 8],
                    vec![6, 5, 7, 8],
                    vec![7, 1, 5, 6, 8],
                    vec![8, 5, 6, 7],
                ],
                4,
            ),
            (
                vec![
                    vec![1, 2, 4],
                    vec![2, 3, 1, 4],
                    vec![3, 4, 2],
                    vec![4, 1, 3, 2],
                ],
                4,
            ),
        ];

        for (input, output) in adj_list {
            let g = Graph::import_edges(&input).expect("Error: Couldn't load input edges");
            let mc = g.minimum_cut();
            assert!(mc.is_some());
            let edges = mc.unwrap().export_edges();
            assert_eq!(edges.len(), output);
            println!("------------");
        }
    }
}
