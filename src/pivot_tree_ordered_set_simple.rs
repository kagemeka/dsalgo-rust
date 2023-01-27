use crate::pivot_tree_node_usize_recurse::*;

pub struct PivotSet {
    root: Option<Box<Node>>,
    max_height: usize,
    size: usize,
}

impl PivotSet {
    /// deal with 0 <= x < 2^max_height - 1
    /// without duplicates.

    pub fn new(max_height: usize) -> Self {
        Self { root: None, max_height, size: 0 }
    }

    pub fn size(&self) -> usize { self.size }

    pub fn min_ge(
        &self,
        x: usize,
    ) -> Option<usize> {
        let v = Node::min_ok(|v| v >= x + 1, self.root.as_ref())?;

        Some(v - 1)
    }

    pub fn max_le(
        &self,
        x: usize,
    ) -> Option<usize> {
        let v = Node::max_ok(|v| v <= x + 1, self.root.as_ref())?;

        Some(v - 1)
    }

    pub fn contains(
        &self,
        x: usize,
    ) -> bool {
        if let Some(v) = self.min_ge(x) {
            v == x
        } else {
            false
        }
    }

    pub fn insert(
        &mut self,
        mut x: usize,
    ) {
        assert!(x < (1 << self.max_height) - 1);

        if self.contains(x) {
            return;
        }

        x += 1;

        if let Some(root) = self.root.as_mut() {
            root.insert(x);
        } else {
            self.root = Node::new(self.max_height, x)
        }

        self.size += 1;
    }

    pub fn remove(
        &mut self,
        x: usize,
    ) {
        if !self.contains(x + 1) {
            return;
        }

        self.root = Node::remove(self.root.take(), x + 1);

        self.size -= 1;
    }

    pub fn iter<'a>(&'a self) -> std::vec::IntoIter<&'a usize> {
        self.root.as_ref().unwrap().iter()
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let h = 31;

        let mut s = PivotSet::new(h);

        s.insert(1);

        assert_eq!(s.size(), 1);

        s.insert(0);

        assert_eq!(s.size(), 2);

        s.insert(1 << (h - 1));

        assert_eq!(s.size(), 3);

        assert_eq!(s.min_ge(2), Some(1 << (h - 1)));

        assert_eq!(s.min_ge(1), Some(1));

        assert_eq!(s.min_ge(0), Some(0));

        assert_eq!(s.max_le(2), Some(1));

        assert_eq!(s.max_le(1), Some(1));

        assert_eq!(s.max_le(0), Some(0));

        assert!(s.contains(0));

        s.remove(0);

        assert!(!s.contains(0));

        for x in s.iter() {
            dbg!(x);
        }
    }

    #[test]

    fn test_abc217() {
        let cases = vec![
            (5, vec![((2, 2), 5), ((1, 3), 0), ((2, 2), 3)]),
            (5, vec![((1, 2), 0), ((1, 4), 0), ((2, 3), 2)]),
            (
                100,
                vec![
                    ((1, 31), 0),
                    ((2, 41), 69),
                    ((1, 59), 0),
                    ((2, 26), 31),
                    ((1, 53), 0),
                    ((2, 58), 6),
                    ((1, 97), 0),
                    ((2, 93), 38),
                    ((1, 23), 0),
                    ((2, 84), 38),
                ],
            ),
        ];

        for (l, q) in cases {
            let mut s = PivotSet::new(30);

            s.insert(0);

            s.insert(l);

            for ((t, x), ans) in q {
                if t == 1 {
                    s.insert(x);
                } else {
                    let hi = s.min_ge(x).unwrap();

                    let lo = s.max_le(x).unwrap();

                    assert_eq!(hi - lo, ans);
                }
            }
        }
    }
}
