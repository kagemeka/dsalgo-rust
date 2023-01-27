pub struct Node<T> {
    pub next: Option<Box<Node<T>>>,
    pub value: T,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Box<Self> { Box::new(Self { next: None, value }) }

    pub fn add(
        &mut self,
        node: Option<Box<Node<T>>>,
    ) {
        self.next = node;
    }

    pub fn split_off(&mut self) -> Option<Box<Node<T>>> { self.next.take() }
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
