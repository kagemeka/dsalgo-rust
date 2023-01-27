use std::collections::BinaryHeap;

pub struct LazyBinaryHeap<T> {
    que: BinaryHeap<T>,
    remove_que: BinaryHeap<T>,
}

impl<T: Ord> LazyBinaryHeap<T> {
    pub fn new() -> Self {
        Self { que: BinaryHeap::new(), remove_que: BinaryHeap::new() }
    }

    pub fn insert(
        &mut self,
        x: T,
    ) {
        self.que.push(x);
    }

    pub fn remove(
        &mut self,
        x: T,
    ) {
        self.remove_que.push(x);
    }

    fn lazy_discard_false_peek(&mut self) {
        while let Some(x) = self.que.peek() {
            while let Some(y) = self.remove_que.peek() {
                if y <= x {
                    break;
                }

                self.remove_que.pop();
            }

            if let Some(y) = self.remove_que.peek() {
                if y != x {
                    return;
                }

                self.que.pop();

                self.remove_que.pop();
            } else {
                return;
            }
        }
    }

    pub fn peek(&mut self) -> Option<&T> {
        self.lazy_discard_false_peek();

        self.que.peek()
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let mut que = LazyBinaryHeap::new();

        que.insert(1);

        que.insert(2);

        assert_eq!(que.peek(), Some(&2));

        que.remove(1);

        assert_eq!(que.peek(), Some(&2));

        que.remove(2);

        assert_eq!(que.peek(), None);
    }
}
