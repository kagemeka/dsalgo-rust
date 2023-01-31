use crate::dynamic_sqrt_bucket::SqrtBucket;

pub struct Multiset<T>(SqrtBucket<T>);

impl<T: Ord> Multiset<T> {
    pub fn new() -> Self { Self(SqrtBucket::new()) }

    pub fn size(&self) -> usize { self.0.size() }

    pub fn lower_bound(
        &self,
        x: &T,
    ) -> usize {
        self.0.binary_search(|v| v >= x)
    }

    pub fn upper_bound(
        &self,
        x: &T,
    ) -> usize {
        self.0.binary_search(|v| v > x)
    }

    pub fn count(
        &self,
        x: &T,
    ) -> usize {
        self.upper_bound(x) - self.lower_bound(x)
    }

    pub fn insert(
        &mut self,
        x: T,
    ) {
        let i = self.lower_bound(&x);

        self.0.insert(i, x);
    }

    pub fn remove(
        &mut self,
        x: &T,
    ) {
        assert!(self.count(x) > 0);

        let i = self.lower_bound(&x);

        self.0.remove(i);
    }
}

use std::ops::*;

impl<T> Index<usize> for Multiset<T> {
    type Output = T;

    fn index(
        &self,
        i: usize,
    ) -> &Self::Output {
        &self.0[i]
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {}

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
            let mut s = Multiset::new();

            for ((t, x), ans) in q {
                if t == 1 {
                    s.insert(x);
                } else {
                    let v = s[x - 1];

                    assert_eq!(v, ans);

                    s.remove(&v);
                }
            }
        }
    }
}
