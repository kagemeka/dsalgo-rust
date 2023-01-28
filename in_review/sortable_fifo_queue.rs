use std::{
    cmp::Reverse,
    collections::{
        BinaryHeap,
        VecDeque,
    },
};

pub struct SortableQueue<T> {
    que: VecDeque<T>,
    hq: BinaryHeap<Reverse<T>>,
}

impl<T: Ord> SortableQueue<T> {
    pub fn new() -> Self {
        Self { que: VecDeque::new(), hq: BinaryHeap::new() }
    }

    pub fn size(&self) -> usize { self.que.len() + self.hq.len() }

    pub fn push(
        &mut self,
        x: T,
    ) {
        self.que.push_back(x);
    }

    pub fn sort(&mut self) {
        while let Some(x) = self.que.pop_front() {
            self.hq.push(Reverse(x));
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.size() == 0 {
            return None;
        }

        Some(if self.hq.is_empty() {
            self.que.pop_front().unwrap()
        } else {
            self.hq.pop().unwrap().0
        })
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![
            vec![
                ((1, 4), -1),
                ((1, 3), -1),
                ((1, 2), -1),
                ((1, 1), -1),
                ((3, -1), -1),
                ((2, -1), 1),
                ((1, 0), -1),
                ((2, -1), 2),
            ],
            vec![
                ((1, 5), -1),
                ((1, 5), -1),
                ((1, 3), -1),
                ((2, -1), 5),
                ((3, -1), -1),
                ((2, -1), 3),
                ((1, 6), -1),
                ((3, -1), -1),
                ((2, -1), 5),
            ],
        ];

        for queries in cases {
            let mut que = SortableQueue::new();

            for ((t, x), ans) in queries {
                if t == 1 {
                    que.push(x);
                } else if t == 2 {
                    assert_eq!(que.pop().unwrap(), ans);
                } else {
                    que.sort();
                }
            }
        }
    }
}
