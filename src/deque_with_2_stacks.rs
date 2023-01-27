use std::mem::swap;

pub struct Deque<T> {
    st_l: Vec<T>,
    st_r: Vec<T>,
}

impl<T> Deque<T> {
    pub fn new() -> Self { Self { st_l: vec![], st_r: vec![] } }

    pub fn size(&self) -> usize { self.st_l.len() + self.st_r.len() }

    fn swap_lr(&mut self) { swap(&mut self.st_l, &mut self.st_r); }

    pub fn push_right(
        &mut self,
        x: T,
    ) {
        self.st_r.push(x);
    }

    pub fn push_left(
        &mut self,
        x: T,
    ) {
        self.st_l.push(x);
    }

    pub fn pop_right(&mut self) -> Option<T> {
        if !self.st_r.is_empty() {
            return self.st_r.pop();
        }

        while self.st_l.len() > self.st_r.len() + 1 {
            self.st_r.push(self.st_l.pop().unwrap());
        }

        self.st_l.reverse();

        self.st_r.reverse();

        self.swap_lr();

        self.st_r.pop()
    }

    pub fn pop_left(&mut self) -> Option<T> {
        self.swap_lr();

        let v = self.pop_right();

        self.swap_lr();

        v
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let mut que = Deque::new();

        que.push_left(0);

        que.push_left(1);

        que.push_left(2);

        que.push_left(3);

        assert_eq!(que.pop_right(), Some(0));

        assert_eq!(que.pop_left(), Some(3));

        que.push_right(4);

        assert_eq!(que.pop_left(), Some(2));

        assert_eq!(que.pop_left(), Some(1));

        assert_eq!(que.pop_left(), Some(4));
    }
}
