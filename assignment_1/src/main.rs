extern crate petgraph;
use petgraph::Graph;
use std::ptr;

fn main() {
    println!("Hello, world!");
}

fn lca(mut graph: Graph<&str, &str>, x: i32, y: i32) {
        let a = graph.add_node("x");
        let b = graph.add_node("y");
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
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
    // #[test]
    // fn larger_can_hold_smaller() {
    //     let larger = Rectangle { length: 8, width: 7 };
    //     let smaller = Rectangle { length: 5, width: 1 };
    //
    //     assert!(larger.can_hold(&smaller));
    // }

    // fn build_graph() {
    //     let mut deps = Graph::<&str, &str>::new();
    //     let pg = deps.add_node("petgraph");
    //     let fb = deps.add_node("fixedbitset");
    //     let qc = deps.add_node("quickcheck");
    //     let rand = deps.add_node("rand");
    //     let libc = deps.add_node("libc");
    //     deps.extend_with_edges(&[
    //         (pg, fb), (pg, qc),
    //         (qc, rand), (rand, libc), (qc, libc),
    //     ]);
    // }
    #[test]
    fn test_empty() {
        use super::*;
        let mut graph = Graph::<&str, &str>::new();
        let a = 1;
        let b = 2;

        assert_eq!(lca(graph, a, b), NULL, "Empty graph should return null as LCA");

        assert!((lca(graph, a, b).is_null());
    }


}
