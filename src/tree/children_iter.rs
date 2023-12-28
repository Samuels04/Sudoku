pub trait ChildrenIterator<T>: Iterator {
	fn set(t: T);
	fn add(t: T);
}