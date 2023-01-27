pub struct BinaryMinHeap<T>(Vec<T>);

impl<T> BinaryMinHeap<T> {
    pub fn new() -> Self { Self(vec![]) }

    pub fn size(&self) -> usize { self.0.len() }

    pub fn push(
        &mut self,
        x: T,
    ) where
        T: PartialOrd,
    {
        let mut i = self.size();

        self.0.push(x);

        while i > 0 {
            let j = (i - 1) >> 1;

            if self.0[i] >= self.0[j] {
                break;
            }

            self.0.swap(i, j);

            i = j;
        }
    }

    pub fn top(&self) -> &T { &self.0[0] }

    pub fn pop(&mut self) -> Option<T>
    where
        T: PartialOrd,
    {
        let mut n = self.size();

        if n == 0 {
            return None;
        }

        self.0.swap(0, n - 1);

        let x = self.0.pop();

        n -= 1;

        let mut i = 0;

        loop {
            let mut j = (i << 1) | 1;

            if j >= n {
                break;
            }

            if j < n - 1 && self.0[j + 1] < self.0[j] {
                j += 1;
            }

            if self.0[i] <= self.0[j] {
                break;
            }

            self.0.swap(i, j);

            i = j;
        }

        x
    }
}

impl<T> Default for BinaryMinHeap<T> {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let mut que = BinaryMinHeap::new();

        que.push(0);

        assert_eq!(que.top(), &0);

        que.push(-1);

        assert_eq!(que.top(), &-1);

        que.push(1);

        assert_eq!(que.top(), &-1);

        assert_eq!(que.pop(), Some(-1));

        assert_eq!(que.pop(), Some(0));

        assert_eq!(que.pop(), Some(1));
    }
}
