use std::collections::VecDeque;

pub struct SWAGDeque {
    v: i64,
    que: VecDeque<i64>,
}

impl SWAGDeque {
    pub fn new() -> Self { Self { v: 0, que: VecDeque::new() } }

    pub fn size(&self) -> usize { self.que.len() }

    pub fn push_right(
        &mut self,
        x: i64,
    ) {
        self.v += x;

        self.que.push_back(x);
    }

    pub fn push_left(
        &mut self,
        x: i64,
    ) {
        self.v += x;

        self.que.push_front(x);
    }

    pub fn pop_left(&mut self) { self.v -= self.que.pop_front().unwrap(); }

    pub fn pop_right(&mut self) { self.v -= self.que.pop_back().unwrap(); }

    pub fn fold(&self) -> i64 { self.v }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let mut swag = SWAGDeque::new();

        assert_eq!(swag.fold(), 0);

        swag.push_right(1);

        assert_eq!(swag.fold(), 1);

        swag.push_right(2);

        assert_eq!(swag.fold(), 3);

        swag.pop_left();

        assert_eq!(swag.fold(), 2);

        swag.pop_left();

        assert_eq!(swag.fold(), 0);
    }
}
