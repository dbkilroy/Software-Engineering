extern crate petgraph;
use petgraph::Graph;

fn main() {
    println!("Hello, world!");
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn lca(mut graph: Graph<&str, &str>, x: &str, y: &str) -> String{
        let a = graph.add_node("x");
        let b = graph.add_node("y");

        return String::from(" ");
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
    #[should_panic]
    #[allow(unused_mut)]
    fn test_empty() {
        use super::*;
        let mut graph = Graph::<&str, &str>::new();
        lca(graph, "1", "2");
    }

    #[test]
    fn test_root_is_lca() {
        use super::*;
        let mut graph = Graph::<&str, &str>::new();
        let a = graph.add_node("a");
        let b = graph.add_node("b");
        let c = graph.add_node("c");

        graph.extend_with_edges(&[
             (a, b), (a, c)
        ]);

        assert_eq!(lca(graph, "b", "c"), "a");
    }


}
