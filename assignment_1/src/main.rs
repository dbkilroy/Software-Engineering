extern crate petgraph;
extern crate pathfinding;

use petgraph::Graph;
use petgraph::graph::NodeIndex;
use std::collections::LinkedList;
use self::pathfinding::prelude::astar;

fn main() {
    println!("Hello, world!");
}

fn neighbors<N, E>(graph: &Graph<N, E>, n: NodeIndex) -> LinkedList<(NodeIndex, u32)> {
    let mut list: LinkedList<(NodeIndex, u32)> = LinkedList::new();
    let mut neighbors = graph.neighbors(n).collect::<LinkedList<NodeIndex>>();
    for element in neighbors.iter_mut() {
        list.push_back((*element, 1));
    }
    return list;
}

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_mut)]
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
        // if root == "" {                 //    if (root == null) { return null; }
        //     return String::from("");
        // }
        //
        // if root == x || root == y {     //      if (root == n1 || root == n2) { return root; }
        //     return String::from(root);
        // }

        //graph.neighbors_directed(root, Outgoing);      //pub fn neighbors_directed( &self, a: NodeIndex<Ix>,
                                        //                     dir: Direction) -> Neighbors<E, Ix>
        //let left = lca(graph, root, )   //Node left = LCA(root left, n1, n2);

        //return root;
}

//Pseudo-Code for proposed solution
/*
 *  Node LCA(Node root, Node n1, Node n2) {
 *      if (root == null) { return null; }
 *      if (root == n1 || root == n2) { return root; }
 *
 *      Node left = LCA(root left, n1, n2);
 *      Node right = LCA(rott right, n1, n2);
 *      if (left != null && right != null) { return root; }
 *      if (left == null && right == null) { return null; }
 *
 *      return left != null ? left : right;
 */

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

    // #[test]
    // fn test_left_lca() {
    //     use super::*;
    //     let mut graph = Graph::<&str, i32>::new();
    //     let a = graph.add_node("a");
    //     let b = graph.add_node("b");
    //     let c = graph.add_node("c");
    //     let d = graph.add_node("d");
    //     let e = graph.add_node("e");
    //
    //     graph.extend_with_edges(&[
    //          (a, b), (a, c), (b, d), (b, e)
    //     ]);
    //
    //     assert_eq!(lca(graph, a, d, e), b);
    // }
    //
    // #[test]
    // fn test_right_lca() {
    //     use super::*;
    //     let mut graph = Graph::<&str, i32>::new();
    //     let a = graph.add_node("a");
    //     let b = graph.add_node("b");
    //     let c = graph.add_node("c");
    //     let d = graph.add_node("d");
    //     let e = graph.add_node("e");
    //
    //     graph.extend_with_edges(&[
    //          (a, b), (a, c), (c, d), (c, e)
    //     ]);
    //
    //     assert_eq!(lca(graph, a, d, e), c);
    // }
    //
    // #[test]
    // fn test_lca_is_a() {
    //     use super::*;
    //     let mut graph = Graph::<&str, i32>::new();
    //     let a = graph.add_node("a");
    //     let b = graph.add_node("b");
    //     let c = graph.add_node("c");
    //
    //     graph.extend_with_edges(&[
    //          (a, b), (b, c)
    //     ]);
    //
    //     assert_eq!(lca(graph, a, b, c), b);
    // }
    //
    // #[test]
    // fn test_lca_is_b() {
    //     use super::*;
    //     let mut graph = Graph::<&str, i32>::new();
    //     let a = graph.add_node("a");
    //     let b = graph.add_node("b");
    //     let c = graph.add_node("c");
    //
    //     graph.extend_with_edges(&[
    //          (a, b), (c, b)
    //     ]);
    //
    //     assert_eq!(lca(graph, a, b, c), c);
    // }

}
