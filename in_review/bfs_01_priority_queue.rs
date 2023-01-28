use std::collections::VecDeque;

pub struct BFS01Queue<T>(VecDeque<T>);

impl<T: Ord> BFS01Queue<T> {
    pub fn new() -> Self { Self(VecDeque::new()) }

    pub fn size(&self) -> usize { self.0.len() }

    pub fn push(
        &mut self,
        x: T,
    ) {
        if self.size() == 0 || &x <= self.0.front().unwrap() {
            self.0.push_front(x);
        } else if &x >= self.0.back().unwrap() {
            self.0.push_back(x);
        } else {
            panic!();
        }
    }

    pub fn pop(&mut self) -> Option<T> { self.0.pop_front() }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let mut que = BFS01Queue::new();

        que.push(1);

        que.push(0);

        que.push(2);

        assert_eq!(que.pop(), Some(0));

        que.push(1);

        assert_eq!(que.pop(), Some(1));

        assert_eq!(que.pop(), Some(1));

        assert_eq!(que.pop(), Some(2));

        assert_eq!(que.pop(), None);
    }
}
