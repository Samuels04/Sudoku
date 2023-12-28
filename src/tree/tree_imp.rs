use std::slice::Iter;

use super::{Trees, ListTreeIterator};


pub struct Tree<T> {
	label: T,
	children: Vec<Tree<T>>,
	parent: Option<Box<Tree<T>>>,
}

impl<T: PartialEq> Tree<T> {
    pub fn new(label: T, trees: Vec<Tree<T>>) -> Self {
        Tree { label: label, children: trees, parent: None }
    }
    pub fn from(t: Tree<T>) -> Self {
        let mut itr = t.iter();

        let s = Tree::from(*itr.next().unwrap());
    }

    pub fn set_parent(&self, to: Tree<T>) {
        self.parent = Some(Box::new(to))
    }
}

impl<T: PartialEq> Trees<T> for Tree<T> {
    fn is_leaf(&self) -> bool {
		if self.children.len() == 0 {true} else {false}
    }

    fn label(&self) -> T {
        self.label
    }

    fn iter(&self) -> ListTreeIterator<T>{
        ListTreeIterator::new(self.children, self)
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



