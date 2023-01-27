pub struct Queue<T> {
    st_front: Vec<T>,
    st_back: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self { Self { st_front: vec![], st_back: vec![] } }

    pub fn size(&self) -> usize { self.st_front.len() + self.st_back.len() }

    pub fn push(
        &mut self,
        x: T,
    ) {
        self.st_back.push(x);
    }

    pub fn pop(&mut self) -> Option<T> {
        if !self.st_front.is_empty() {
            return self.st_front.pop();
        }

        while let Some(x) = self.st_back.pop() {
            self.st_front.push(x);
        }

        self.st_front.pop()
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
