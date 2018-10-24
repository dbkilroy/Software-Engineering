//This solution must work for DAG and Binary Tree Graphs
extern crate petgraph;
extern crate pathfinding;

use petgraph::Graph;
use petgraph::graph::NodeIndex;
use std::collections::LinkedList;
use pathfinding::prelude::astar;

fn main() {}

fn neighbors<N, E>(graph: &Graph<N, E>, n: NodeIndex) -> LinkedList<(NodeIndex, u32)> {
    let mut list: LinkedList<(NodeIndex, u32)> = LinkedList::new();
    let mut neighbors = graph.neighbors(n).collect::<LinkedList<NodeIndex>>();
    for element in neighbors.iter_mut() {
        list.push_back((*element, 1));
    }
    return list;
}


//******************************************************************************************
// lca function returns the lowest common ancestor of two nodes in a binary tree recursively
// Parameters:
//      graph - the graph containing the nodes
//      root - root element of the Graph
//      x - first node to calculate lowest common ancestor on
//      y - second node to calculate lowest common ancestor on
//
// Returns:
//      The NodeIndex of the lowest common ancestor or None
//
//******************************************************************************************

pub fn lca<N, E>(graph: &Graph<N, E>, root: NodeIndex, x: NodeIndex, y: NodeIndex) -> Option<NodeIndex>{



        let path1 = astar(&root, |n| neighbors(&graph, *n), |_| 0, |n| *n == x);
        let path2 = astar(&root, |n| neighbors(&graph, *n), |_| 0, |n| *n == y);

        if x != y {
            let reverse1 = astar(&x, |n| neighbors(&graph, *n), |_| 0, |n| *n == root);
            let reverse2 = astar(&y, |n| neighbors(&graph, *n), |_| 0, |n| *n == root);

            if reverse1.is_some() || reverse2.is_some() {
                return None;
            }
        }

        if path1.is_some() && path2.is_some() {
            let path1arr = path1.unwrap().0;
            let path2arr = path2.unwrap().0;

            let len;
            if path1arr.len() < path2arr.len() {
                len = path1arr.len();
            } else {
                len = path2arr.len();
            }

            let mut lca_btree = root;
            for i in 0..len {
                if path1arr[i] == path2arr[i] {
                    lca_btree = path1arr[i]
                } else {
                    break;
                }
            }
            return Some(lca_btree);
        }
    return None;

}

#[cfg(test)]
 mod tests {
    #[test]
    #[allow(unused_mut)]
    fn test_no_connections() {
        use super::*;
        let mut graph = Graph::<&str, i32>::new();
        let root = graph.add_node("root");
        let n1 = graph.add_node("1");
        let n2 = graph.add_node("2");
        assert_eq!(false, lca(&graph, root, n1, n2).is_some());
    }

    #[test]
    fn test_root_is_lca() {
        use super::*;
        let mut graph = Graph::<&str, i32>::new();
        let a = graph.add_node("a");
        let b = graph.add_node("b");
        let c = graph.add_node("c");

        graph.extend_with_edges(&[
             (a, b), (a, c),
        ]);

        assert_eq!(true, lca(&graph, a, b, c).is_some());
        assert_eq!(a, lca(&graph, a, b, c).unwrap());
    }

    #[test]
    fn test_lca_btree() {
        use super::*;
        let mut graph = Graph::<&str, i32>::new();
        let a = graph.add_node("a");
        let b = graph.add_node("b");
        let c = graph.add_node("c");
        let d = graph.add_node("d");
        let e = graph.add_node("e");

        graph.extend_with_edges(&[
             (a, b), (a, c), (b, d), (b, e)
        ]);
        assert_eq!(true, lca(&graph, a, d, e).is_some());
        assert_eq!(b, lca(&graph, a, d, e).unwrap());
    }

    #[test]
    fn test_lca_is_b() {
        use super::*;
        let mut graph = Graph::<&str, i32>::new();
        let a = graph.add_node("a");
        let b = graph.add_node("b");
        let c = graph.add_node("c");

        graph.extend_with_edges(&[
             (a, b), (b, c)
        ]);
        assert_eq!(true, lca(&graph, a, b, c).is_some());
        assert_eq!(b, lca(&graph, a, b, c).unwrap());
    }

    #[test]
    fn dag_tri_branched_root(){
        use super::*;
        let mut graph = Graph::<&str, i32>::new();
        let a = graph.add_node("a");
        let b = graph.add_node("b");
        let c = graph.add_node("c");
        let d = graph.add_node("d");
        let e = graph.add_node("e");
        let f = graph.add_node("f");

        graph.extend_with_edges(&[
             (a, b), (a, c), (a, d), (b, e), (c, e), (c, f)
        ]);

        assert_eq!(true, lca(&graph, a, e, f).is_some());
        assert_eq!(c, lca(&graph, a, e, f).unwrap());
    }

    #[test]
    fn dag_sparce() {
        use super::*;
        let mut graph = Graph::<&str, i32>::new();
        let a = graph.add_node("a");
        let b = graph.add_node("b");
        let c = graph.add_node("c");
        let d = graph.add_node("d");
        let e = graph.add_node("e");

        graph.extend_with_edges(&[
             (a, b), (b, c), (c, d), (b, e), (e, d)
        ]);

        assert_eq!(true, lca(&graph, a, d, e).is_some());
        assert_eq!(e, lca(&graph, a, d, e).unwrap());
    }

    #[test]
    fn dag_dense(){
        use super::*;
        let mut graph = Graph::<&str, i32>::new();
        let a = graph.add_node("a");
        let b = graph.add_node("b");
        let c = graph.add_node("c");
        let d = graph.add_node("d");
        let e = graph.add_node("e");
        let f = graph.add_node("f");
        let g = graph.add_node("g");

        graph.extend_with_edges(&[
             (a, b), (b, c), (c, d), (b, e), (e, f), (d, g), (f, g)
        ]);

        assert_eq!(true, lca(&graph, a, g, e).is_some());
        assert_eq!(e, lca(&graph, a, g, e).unwrap());
    }

}
