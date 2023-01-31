use std::{
    cell::RefCell,
    rc::Rc,
};

pub type Cell<T> = Rc<RefCell<Node<T>>>;

pub struct Node<T> {
    pub next: Option<Cell<T>>,
    pub value: T,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Cell<T> {
        Rc::new(RefCell::new(Self { next: None, value }))
    }

    pub fn add(
        &mut self,
        node: Option<Cell<T>>,
    ) {
        self.next = node;
    }

    pub fn split_off(&mut self) -> Option<Cell<T>> { self.next.take() }
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
