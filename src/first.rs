use std::mem;

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}


impl List {
    pub fn new() -> Self {
	List { head: Link::Empty }
    }
    pub fn push(&mut self, elem: i32) {
	let new_node = Box::new(Node {
	    elem: elem,
	    next: mem::replace(&mut self.head, Link::Empty),
	});
	self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
	match mem::replace(&mut self.head, Link::Empty) {
	    Link::Empty => None,
	    Link::More(node) => {
		self.head = node.next;
		Some(node.elem)
	    }
	}
    }
}
impl Drop for List {
    fn drop(&mut self) {
	let mut current_link = mem::replace(&mut self.head, Link::Empty);
	while let Link::More(mut boxed_node) = current_link {
	    current_link = mem::replace(&mut boxed_node.next, Link::Empty);
	}
    }
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basic() {
	let mut list = List::new();
	
	assert_eq!(list.pop(), None);

	for i in 1..4 {
	    list.push(i);
	    //println!("{}", i);
	}
	assert_eq!(list.pop(), Some(3));
	assert_eq!(list.pop(), Some(2));

	for i in 4..6 {
	    list.push(i);
	}
	assert_eq!(list.pop(), Some(5));
	assert_eq!(list.pop(), Some(4));

	assert_eq!(list.pop(), Some(1));
	assert_eq!(list.pop(), None);
	assert_eq!(list.pop(), None);
    }
}
