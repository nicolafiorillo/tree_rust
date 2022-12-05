#[derive(Debug)]
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
    }
}
