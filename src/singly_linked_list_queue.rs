use std::{
    cell::RefCell,
    rc::{
        Rc,
        Weak,
    },
};

use crate::singly_linked_list_node_with_rc_refcell::*;

type Ref<T> = Weak<RefCell<Node<T>>>;

pub struct Queue<T> {
    front: Option<Cell<T>>,
    back: Option<Ref<T>>,
    size: usize,
}

impl<T> Queue<T> {
    pub fn new() -> Self { Self { front: None, back: None, size: 0 } }

    pub fn size(&self) -> usize { self.size }

    pub fn is_empty(&self) -> bool { self.size() == 0 }

    pub fn push(
        &mut self,
        x: T,
    ) {
        let x = Node::new(x);

        if self.size == 0 {
            self.front = Some(x.clone());
        } else {
            self.back
                .take()
                .unwrap()
                .upgrade()
                .unwrap()
                .borrow_mut()
                .add(Some(x.clone()));
        }

        self.back = Some(Rc::downgrade(&x));

        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T>
    where
        T: Clone,
    {
        if self.size == 0 {
            return None;
        }

        let node = self.front.take().unwrap();

        let v = node.borrow_mut().value.clone();

        self.front = node.borrow_mut().next.take();

        self.size -= 1;

        if self.size == 0 {
            self.back = None;
        }

        Some(v)
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let mut que = Queue::new();

        que.push(2);

        que.push(1);

        assert_eq!(que.pop(), Some(2));

        que.push(3);

        assert_eq!(que.pop(), Some(1));

        assert_eq!(que.pop(), Some(3));

        assert_eq!(que.pop(), None);
    }
}
