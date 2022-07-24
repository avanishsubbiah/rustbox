use std::mem;

// Defined with pub to be useable outside of module
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

// Implementing linked list
// impl List {
// }

impl List {
    pub fn new() -> Self {
        List {head: Link::Empty}
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
        // Replacing self.head with empty and moving to cur_link
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);

        // Iterating through list (while let <obj> -> do while pattern matches obj)
        while let Link::More(mut boxed_node) = cur_link {
            // Replacing node next ptr with Link::Empty then letting it go out of scope
            // This prevents unbounded recursion
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        // Initialize list
        let mut list = List::new();

        // Check that popping empty list yields None
        assert_eq!(list.pop(), None);

        // Populate List
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push more values
        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check list emptying
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}