use std::iter::FromIterator;

// Helps count the length of a linked list
// Tail recursive, although not sure if Rust can tell.
fn count_this_node_and_maybe_continue<U>(count: usize, node: &Box<Node<U>>) -> usize {
    match &node.next {
        None => return count + 1,
        Some(n) => count_this_node_and_maybe_continue(count + 1, &n)
    }
}

pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>
}

impl<T> Node<T> {
    fn new(data: T, next: Option<Box<Node<T>>>) -> Node<T> {
        Node { data, next }
    }
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    pub fn is_empty(&self) -> bool {
        match self.head {
            None => true,
            _ => false
        }
    }

    pub fn len(&self) -> usize {
        match &self.head {
            None => 0,
            Some(n) => count_this_node_and_maybe_continue(0, &n)
        }
    }

    pub fn push(&mut self, element: T) {
        self.head = Some(Box::new(Node { data: element, next: self.head.take() }));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|n| {
            self.head = n.next;
            n.data
        })
    }

    pub fn peek(&self) -> Option<&T> {
        match &self.head {
            None => None,
            Some(n) => Some(&n.data)
        }
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut v = Vec::from(self);
        let mut sll = SimpleLinkedList::new();
        loop {
            match v.pop() {
                None => return sll,
                Some(item) => sll.push(item),
            }
        }
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut head: Option<Box<Node<T>>> = None;
        for item in _iter {
            head = Some(Box::new(Node::new(item, head)));
        }
        let mut sll = SimpleLinkedList::new();
        sll.head = head;
        sll
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut v = vec![];
        let mut next = _linked_list.head;
        loop {
            match next {
                None => {
                    v.reverse();
                    return v
                },
                Some(bn) => {
                    next = bn.next;
                    v.push(bn.data);
                }
            }
        }
    }
}
