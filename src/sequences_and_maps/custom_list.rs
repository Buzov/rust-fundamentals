struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node { value, next: None }
    }
}

pub struct List<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push_front(&mut self, value: T) {
        let mut new_boxed = Box::new(Node::new(value));
        new_boxed.next = self.head.take();
        self.head = Some(new_boxed);
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().and_then(|node| {
            let next = node.next;
            self.head = next;
            Some(node.value)
        })
    }

    pub fn push_back(&mut self, value: T) {
        let new_boxed = Box::new(Node::new(value));
        if self.head.is_none() {
            self.head = Some(new_boxed);
            return;
        }
        let mut curr = self.head.as_mut().unwrap();
        while curr.next.is_some() {
            curr = curr.next.as_mut().unwrap();
        }
        curr.next = Some(new_boxed);
    }

    pub fn pop_back(&mut self) -> Option<T> {
        // empty list
        if self.head.is_none() {
            None
        } else if self.head.as_ref().unwrap().next.is_none() { // the only element
            Some(self.head.take().unwrap().value)
        } else { // 2+ elements
            let mut curr = self.head.as_mut().unwrap();
            while curr.next.is_some() && curr.next.as_mut().unwrap().next.is_some() {
                curr = curr.next.as_mut().unwrap();
            }
            Some(curr.next.take().unwrap().value)
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_front_pop_front() {
        let mut list = List::new();
        for i in 0..10 {
            list.push_front(i);
        }
        for i in (0..10).rev() {
            let pop = list.pop_front().unwrap();
            println!("popped: {}", pop);
            assert_eq!(pop, i);
            if i > 0 {
                assert!(!list.is_empty());
            }
        }
        assert!(list.is_empty());
    }

    #[test]
    fn push_back_pop_front() {
        let mut list = List::new();
        for i in 0..10 {
            list.push_back(i);
        }
        for i in 0..10 {
            let pop = list.pop_front().unwrap();
            println!("popped: {}", pop);
            assert_eq!(pop, i);
            if i < 9 {
                assert!(!list.is_empty());
            }
        }
        assert!(list.is_empty());
    }

    #[test]
    fn push_front_pop_back() {
        let mut list = List::new();
        for i in 0..10 {
            list.push_front(i);
        }
        for i in 0..10 {
            let pop = list.pop_back().unwrap();
            println!("popped: {}", pop);
            assert_eq!(pop, i);
            if i < 9 {
                assert!(!list.is_empty());
            }
        }
        assert!(list.is_empty());
    }

    #[test]
    fn push_back_pop_back() {
        let mut list = List::new();
        for i in 0..10 {
            list.push_back(i);
        }
        for i in (0..10).rev() {
            let pop = list.pop_back().unwrap();
            println!("popped: {}", pop);
            assert_eq!(pop, i);
            if i > 0 {
                assert!(!list.is_empty());
            }
        }
        assert!(list.is_empty());
    }
}
