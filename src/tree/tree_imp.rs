use std::slice::Iter;

use super::Trees;


pub struct Tree<T> {
	label: T,
	children: Vec<Tree<T>>,
	parent: Box<Tree<T>>,
}

impl<T> Tree<T>

impl<T: PartialEq> Trees<T> for Tree<T> {
    fn is_leaf(&self) -> bool {
		if self.children.len() == 0 {true} else {false}
    }

    fn label(&self) -> T {
        self.label
    }

    fn iter(&self) -> Iter<'_, Tree<T>> {
        self.children.iter()
    }

    fn set_label(&mut self, t: T) {
        self.label = t
    }
}

impl<T: PartialEq> PartialEq for Tree<T> {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label && self.children == other.children && self.parent == other.parent
    }
}

