extern crate petgraph;
use petgraph::Graph;
use petgraph::Outgoing;
use petgraph::graph::NodeIndex;

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
fn lca(mut graph: Graph<&str, &str>, root: NodeIndex, x: &str, y: &str) -> NodeIndex{

        // if root == "" {                 //    if (root == null) { return null; }
        //     return String::from("");
        // }
        //
        // if root == x || root == y {     //      if (root == n1 || root == n2) { return root; }
        //     return String::from(root);
        // }

        graph.neighbors_directed(root, Outgoing);      //pub fn neighbors_directed( &self, a: NodeIndex<Ix>,
                                        //                     dir: Direction) -> Neighbors<E, Ix>
        //let left = lca(graph, root, )   //Node left = LCA(root left, n1, n2);

        return root;
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
        let root = graph.add_node("root");
        lca(graph, root, "1", "2");
    }

    #[test]
    fn test_root_is_lca() {
        use super::*;
        let mut graph = Graph::<&str, &str>::new();
        let root = graph.add_node("root");
        let a = graph.add_node("a");
        let b = graph.add_node("b");
        let c = graph.add_node("c");
        graph.extend_with_edges(&[
             (a, b), (a, c)
        ]);

        assert_eq!(lca(graph, root, "a", "b"), root);
    }

    #[test]
    fn test_left_lca() {
        use super::*;
        let mut graph = Graph::<&str, &str>::new();
        let a = graph.add_node("a");
        let b = graph.add_node("b");
        let c = graph.add_node("c");
        let d = graph.add_node("d");
        let e = graph.add_node("e");

        graph.extend_with_edges(&[
             (a, b), (a, c), (b, d), (b, e)
        ]);

        assert_eq!(lca(graph, a, "d", "e"), b);
    }

    #[test]
    fn test_right_lca() {
        use super::*;
        let mut graph = Graph::<&str, &str>::new();
        let a = graph.add_node("a");
        let b = graph.add_node("b");
        let c = graph.add_node("c");
        let d = graph.add_node("d");
        let e = graph.add_node("e");

        graph.extend_with_edges(&[
             (a, b), (a, c), (c, d), (c, e)
        ]);

        assert_eq!(lca(graph, a, "d", "e"), c);
    }

    #[test]
    fn test_lca_is_a() {
        use super::*;
        let mut graph = Graph::<&str, &str>::new();
        let a = graph.add_node("a");
        let b = graph.add_node("b");
        let c = graph.add_node("c");

        graph.extend_with_edges(&[
             (a, b), (b, c)
        ]);

        assert_eq!(lca(graph, a, "b", "c"), b);
    }

    #[test]
    fn test_lca_is_b() {
        use super::*;
        let mut graph = Graph::<&str, &str>::new();
        let a = graph.add_node("a");
        let b = graph.add_node("b");
        let c = graph.add_node("c");

        graph.extend_with_edges(&[
             (a, b), (c, b)
        ]);

        assert_eq!(lca(graph, a, "b", "c"), c);
    }

}
