use std::collections::BTreeMap;

pub struct MinMaxQueue<T> {
    map: BTreeMap<T, usize>,
    size: usize,
}

impl<T: Ord> MinMaxQueue<T> {
    pub fn new() -> Self { Self { map: BTreeMap::new(), size: 0 } }

    pub fn size(&self) -> usize { self.size }

    pub fn count(
        &self,
        x: &T,
    ) -> usize {
        *self.map.get(x).or_else(|| Some(&0)).unwrap()
    }

    pub fn insert(
        &mut self,
        x: T,
        count: usize,
    ) {
        *self.map.entry(x).or_insert(0) += count;

        self.size += count;
    }

    pub fn remove(
        &mut self,
        x: &T,
        count: usize,
    ) {
        let c = self.map.get_mut(x).unwrap();

        *c -= count;

        if *c == 0 {
            self.map.remove(x);
        }

        self.size -= count;
    }

    pub fn min(&self) -> Option<&T> {
        if let Some((x, _)) = self.map.iter().next() {
            Some(x)
        } else {
            None
        }
    }

    pub fn max(&self) -> Option<&T> {
        if let Some((x, _)) = self.map.iter().next_back() {
            Some(x)
        } else {
            None
        }
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let mut que = MinMaxQueue::new();

        que.insert(1, 5);

        assert_eq!(que.min(), Some(&1));

        que.insert(-1, 1);

        assert_eq!(que.min(), Some(&-1));

        assert_eq!(que.max(), Some(&1));

        que.remove(&1, 1);

        que.remove(&1, 3);

        que.remove(&-1, 1);

        assert_eq!(que.count(&-1), 0);
    }
}
