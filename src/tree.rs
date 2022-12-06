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
    pub fn add(&mut self, value: T) {
        match *self {
            Tree::Empty => {
                *self = Tree::NonEmpty(Box::new(Node {
                    element: value,
                    left: Tree::Empty,
                    right: Tree::Empty,
                }))
            }
            Tree::NonEmpty(ref mut node) => {
                if value <= node.element {
                    node.left.add(value);
                } else {
                    node.right.add(value);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut tree = Tree::Empty;
        tree.add(1);
        tree.add(2);
        tree.add(3);

        assert_element(&tree, 1);
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
