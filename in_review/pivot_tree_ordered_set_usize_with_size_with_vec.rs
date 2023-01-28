pub struct PivotTreeSet {
    data: Vec<Option<usize>>,
    size: Vec<usize>,
    max_height: usize,
}

fn left(pivot: usize) -> usize { pivot - (1 << pivot.trailing_zeros() - 1) }

fn right(pivot: usize) -> usize { pivot + (1 << pivot.trailing_zeros() - 1) }

impl PivotTreeSet {
    pub fn new(max_height: usize) -> Self {
        assert!(max_height > 0);

        let n = 1 << max_height;

        Self { data: vec![None; n], size: vec![0; n], max_height }
    }

    fn root_pivot(&self) -> usize { 1 << self.max_height - 1 }

    fn left_size(
        &self,
        p: usize,
    ) -> usize {
        if p & 1 == 1 {
            0
        } else {
            self.size[left(p)]
        }
    }

    fn right_size(
        &self,
        p: usize,
    ) -> usize {
        if p & 1 == 1 {
            0
        } else {
            self.size[right(p)]
        }
    }

    pub fn size(&self) -> usize { self.size[self.root_pivot()] }

    fn update(
        &mut self,
        p: usize,
    ) {
        if self.data[p].is_none() {
            self.size[p] = 0;

            return;
        }

        self.size[p] = self.left_size(p) + self.right_size(p) + 1;
    }

    pub fn _insert(
        &mut self,
        p: usize,
        mut v: usize,
    ) {
        use std::mem::swap;

        let value = self.data[p];

        if value.is_none() {
            debug_assert!(self.size[p] == 0);

            self.data[p] = Some(v);

            self.size[p] = 1;

            return;
        }

        let mut value = value.unwrap();

        if v == value {
            return;
        }

        let d = 1 << p.trailing_zeros();

        assert!(p - d < v && v < p + d);

        if value.min(v) < p {
            if value < v {
                swap(&mut value, &mut v);
            }

            self.data[p] = Some(value);

            self._insert(left(p), v);
        } else {
            if value > v {
                swap(&mut value, &mut v);
            }

            self.data[p] = Some(value);

            self._insert(right(p), v);
        }

        self.update(p);
    }

    fn _remove(
        &mut self,
        p: usize,
        i: usize,
    ) {
        assert!(i < self.size[p]);

        let lsize = self.left_size(p);

        if i < lsize {
            self._remove(left(p), i);
        } else if i > lsize {
            self._remove(right(p), i - lsize - 1);
        } else {
            if self.right_size(p) > 0 {
                let rp = right(p);

                self.data[p] = Some(self.kth_value(rp, 0));

                self._remove(rp, 0);
            } else if lsize > 0 {
                let lp = left(p);

                self.data[p] = Some(self.kth_value(lp, lsize - 1));

                self._remove(lp, lsize - 1);
            } else {
                self.data[p] = None;
            }
        }

        self.update(p);
    }

    fn kth_value(
        &self,
        p: usize,
        k: usize,
    ) -> usize {
        assert!(k < self.size[p]);

        let lsize = self.left_size(p);

        if k < lsize {
            self.kth_value(left(p), k)
        } else if k == lsize {
            self.data[p].unwrap()
        } else {
            self.kth_value(right(p), k - lsize - 1)
        }
    }

    fn binary_search<F>(
        &self,
        f: F,
        p: usize,
    ) -> usize
    where
        F: Fn(usize) -> bool,
    {
        let v = self.data[p];

        if v.is_none() {
            return 0;
        }

        let v = v.unwrap();

        if f(v) {
            if p & 1 == 1 {
                0
            } else {
                self.binary_search(f, left(p))
            }
        } else {
            let i = self.left_size(p) + 1;

            i + if p & 1 == 1 { 0 } else { self.binary_search(f, right(p)) }
        }
    }

    pub fn get(
        &self,
        i: usize,
    ) -> usize {
        self.kth_value(self.root_pivot(), i) - 1
    }

    pub fn lower_bound(
        &self,
        x: usize,
    ) -> usize {
        self.binary_search(|v| v >= x + 1, self.root_pivot())
    }

    pub fn upper_bound(
        &self,
        x: usize,
    ) -> usize {
        self.binary_search(|v| v > x + 1, self.root_pivot())
    }

    pub fn count(
        &self,
        x: usize,
    ) -> usize {
        self.upper_bound(x) - self.lower_bound(x)
    }

    pub fn contains(
        &self,
        x: usize,
    ) -> bool {
        self.count(x) > 0
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

        self._insert(self.root_pivot(), x);
    }

    pub fn remove(
        &mut self,
        x: usize,
    ) {
        if !self.contains(x) {
            return;
        }

        let i = self.lower_bound(x);

        self._remove(self.root_pivot(), i);
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let h = 20;

        let mut s = PivotTreeSet::new(h);

        s.insert(1);

        assert_eq!(s.size(), 1);

        s.insert(0);

        assert_eq!(s.size(), 2);

        s.insert(1 << (h - 1));

        assert_eq!(s.size(), 3);

        assert_eq!(s.get(2), 1 << (h - 1));

        assert_eq!(s.get(1), 1);

        assert_eq!(s.get(0), 0);

        assert!(s.contains(0));

        s.remove(0);

        assert!(!s.contains(0));
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
            let mut s = PivotTreeSet::new(20);

            s.insert(0);

            s.insert(l);

            for ((t, x), ans) in q {
                if t == 1 {
                    s.insert(x);
                } else {
                    let i = s.lower_bound(x);

                    dbg!(i);

                    assert_eq!(s.get(i) - s.get(i - 1), ans);
                }
            }
        }
    }

    #[test]

    fn test_arc033_3() {
        let cases = vec![
            vec![
                ((1, 11), 0),
                ((1, 29), 0),
                ((1, 89), 0),
                ((2, 2), 29),
                ((2, 2), 89),
            ],
            vec![
                ((1, 8932), 0),
                ((1, 183450), 0),
                ((1, 34323), 0),
                ((1, 81486), 0),
                ((1, 127874), 0),
                ((1, 114850), 0),
                ((1, 55277), 0),
                ((1, 112706), 0),
                ((2, 3), 55277),
                ((1, 39456), 0),
                ((1, 52403), 0),
                ((2, 4), 52403),
            ],
        ];

        for q in cases {
            let mut s = PivotTreeSet::new(18);

            for ((t, x), ans) in q {
                if t == 1 {
                    s.insert(x);
                } else {
                    let v = s.get(x - 1);

                    assert_eq!(v, ans);

                    s.remove(v);
                }
            }
        }
    }
}
