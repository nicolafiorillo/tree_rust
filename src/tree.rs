#[derive(PartialEq, Debug)]
pub struct Node<T> {
    pub element: T,
    pub left: Tree<T>,
    pub right: Tree<T>,
}

#[derive(PartialEq, Debug)]
pub enum Tree<T> {
    Empty,
    NonEmpty(Box<Node<T>>),
}

impl<T: Ord> Tree<T> {
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

    pub fn find(&self, elem: T) -> &Box<Node<T>> {
        // TODO
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
    fn find_in_a_tree() {
        let mut tree = Tree::Empty;
        tree.add(1);
        tree.add(2);
        tree.add(3);
        tree.add(4);
        tree.add(5);

        let node = tree.find(5);

        assert_element(&node, 5);
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
