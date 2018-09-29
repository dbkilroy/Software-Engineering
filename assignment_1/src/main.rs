fn main() {
    println!("Hello, world!");
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

    #[test]
    fn another() {
        panic!("Make this test fail");
    }
}
