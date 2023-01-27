pub struct Deque<T> {
    data: Vec<Option<T>>,
    left: usize,
    right: usize,
    size: usize,
}

impl<T: Clone> Deque<T> {
    pub fn new(buf_size: usize) -> Self {
        Self { data: vec![None; buf_size], left: 0, right: 0, size: 0 }
    }

    pub fn size(&self) -> usize { self.size }

    pub fn buf_size(&self) -> usize { self.data.len() }

    pub fn is_full(&self) -> bool { self.size() == self.buf_size() }

    pub fn is_empty(&self) -> bool { self.size() == 0 }

    pub fn push_right(
        &mut self,
        x: T,
    ) {
        assert!(!self.is_full());

        debug_assert!(self.data[self.right].is_none());

        self.data[self.right] = Some(x);

        self.size += 1;

        self.right += 1;

        if self.right == self.buf_size() {
            self.right = 0;
        }
    }

    pub fn push_left(
        &mut self,
        x: T,
    ) {
        assert!(!self.is_full());

        if self.left == 0 {
            self.left = self.buf_size();
        }

        self.left -= 1;

        debug_assert!(self.data[self.left].is_none());

        self.data[self.left] = Some(x);

        self.size += 1;
    }

    pub fn pop_right(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        if self.right == 0 {
            self.right = self.buf_size();
        }

        self.right -= 1;

        let v = self.data[self.right].take();

        self.size -= 1;

        debug_assert!(v.is_some());

        v
    }

    pub fn pop_left(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let v = self.data[self.left].take();

        self.left += 1;

        if self.left == self.buf_size() {
            self.left = 0;
        }

        self.size -= 1;

        debug_assert!(v.is_some());

        v
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let mut que = Deque::new(10);

        que.push_right(0);

        que.push_left(1);

        que.push_right(2);

        assert_eq!(que.size(), 3);

        assert_eq!(que.buf_size(), 10);

        assert_eq!(que.pop_left(), Some(1));

        assert_eq!(que.pop_left(), Some(0));

        assert_eq!(que.pop_right(), Some(2));

        assert_eq!(que.pop_right(), None);
    }
}
