use std::fmt::Display;

#[derive(PartialEq, Debug)]
pub struct Node<T> {
    pub element: T,
    pub left: Tree<T>,
    pub right: Tree<T>,
}

#[derive(Debug)]
pub enum Tree<T> {
    Empty,
    NonEmpty(Box<Node<T>>),
}

impl<T: PartialEq> PartialEq for Tree<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Tree::Empty, Tree::Empty) => true,
            (Tree::NonEmpty(_), Tree::Empty) => false,
            (Tree::Empty, Tree::NonEmpty(_)) => false,
            (Tree::NonEmpty(ref node1), Tree::NonEmpty(ref node2)) => {
                node1.element == node2.element
                    && node1.left == node2.left
                    && node1.right == node2.right
            }
        }
    }
}

impl<T: Ord + Display> Tree<T> {
    pub fn add(&mut self, elem: T) {
        match *self {
            Tree::Empty => {
                *self = Tree::NonEmpty(Box::new(Node {
                    element: elem,
                    left: Tree::Empty,
                    right: Tree::Empty,
                }))
            }
            Tree::NonEmpty(ref mut node) => {
                if elem <= node.element {
                    node.left.add(elem);
                } else {
                    node.right.add(elem);
                }
            }
        }
    }

    pub fn find(&self, elem: T) -> Option<&Node<T>> {
        match *self {
            Tree::Empty => None,
            Tree::NonEmpty(ref node) if node.element.eq(&elem) => Some(node),
            Tree::NonEmpty(ref node) if node.element.gt(&elem) => node.left.find(elem),
            Tree::NonEmpty(ref node) => node.right.find(elem),
        }
    }

    pub fn in_order_print(&self) -> String {
        match *self {
            Tree::Empty => String::new(),
            Tree::NonEmpty(ref node) => {
                let mut result = String::new();
                result.push_str(&node.left.in_order_print());
                result.push_str(&format!("{}", node.element));
                result.push_str(&node.right.in_order_print());
                result.trim().to_string()
            }
        }
    }

    pub fn pre_order_print(&self) -> String {
        match *self {
            Tree::Empty => String::new(),
            Tree::NonEmpty(ref node) => {
                let mut result = String::new();
                result.push_str(&node.right.pre_order_print());
                result.push_str(&format!("{}", node.element));
                result.push_str(&node.left.pre_order_print());
                result.trim().to_string()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_tree() {
        let mut tree = Tree::Empty;
        tree.add(1);
        tree.add(2);
        tree.add(3);

        assert_element(&tree, 1);
    }

    #[test]
    fn find_in_an_unbalanced_tree() {
        let mut tree = Tree::Empty;
        tree.add(1);
        tree.add(2);
        tree.add(3);
        tree.add(4);
        tree.add(5);

        let node = tree.find(5);
        assert_eq!(node.unwrap().element, 5);
    }

    #[test]
    fn find_in_a_balanced_tree() {
        let mut tree = Tree::Empty;
        tree.add(3);
        tree.add(2);
        tree.add(4);
        tree.add(1);
        tree.add(5);

        let node = tree.find(5);
        assert_eq!(node.unwrap().element, 5);
    }

    #[test]
    fn trees_are_equal() {
        let mut tree1 = Tree::Empty;
        tree1.add(3);
        tree1.add(2);
        tree1.add(4);
        tree1.add(1);

        let mut tree2 = Tree::Empty;
        tree2.add(3);
        tree2.add(2);
        tree2.add(4);
        tree2.add(1);

        assert_eq!(tree1, tree2);
    }

    #[test]
    fn trees_are_not_equal() {
        let mut tree1 = Tree::Empty;
        tree1.add(3);
        tree1.add(2);
        tree1.add(4);
        tree1.add(1);

        let mut tree2 = Tree::Empty;
        tree2.add(3);
        tree2.add(2);
        tree2.add(4);
        tree2.add(1);
        tree2.add(0);

        assert_ne!(tree1, tree2);
    }

    #[test]
    fn in_order_print() {
        let mut tree = Tree::Empty;
        tree.add(3);
        tree.add(2);
        tree.add(4);
        tree.add(1);
        tree.add(5);

        assert_eq!(tree.in_order_print(), "12345");
    }

    #[test]
    fn out_order_print() {
        let mut tree = Tree::Empty;
        tree.add(3);
        tree.add(2);
        tree.add(4);
        tree.add(1);
        tree.add(5);

        assert_eq!(tree.pre_order_print(), "54321");
    }

    fn assert_element<T: std::fmt::Debug + std::cmp::Eq>(tree: &Tree<T>, elem: T) {
        match tree {
            Tree::NonEmpty(ref node) => {
                assert_eq!(node.element, elem);
            }
            Tree::Empty => {
                panic!("Tree should cointain an element");
            }
        }
    }
}
