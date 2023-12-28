use super::{ChildrenIterator, Tree, Trees};

pub struct ListTreeIterator<T> {
	children: Vec<Tree<T>>,
	current: Tree<T>,
}
impl<T:PartialEq> ListTreeIterator<T> {
	pub fn new(children: Vec<Tree<T>>, current: Tree<T>) -> Self {
		ListTreeIterator { children: children, current: current }
	}

}

impl<T:PartialEq> ChildrenIterator <Tree<T>> for ListTreeIterator<T> {
    fn set(&self, t: Tree<T>) {
		let s = Tree::from(t);
		s.set_parent(self.current);
		let lit = self.current.iter();
    }

    fn add(t: Tree<T>) {
        todo!()
    }
}