use std::slice::Iter;

use super::{Node, Tree};
pub trait Trees<T>: PartialEq{
	fn is_leaf(&self) -> bool;

	fn label(&self) -> T;

	fn iter(&self) -> Iter<'_, Tree<T>>;

	fn set_label(&mut self, t: T);

}