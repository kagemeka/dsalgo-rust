use crate::splay_tree_node_with_array_recurse::*;

pub struct Multiset<T>(Option<Box<Node<T>>>);

impl<T: Ord> Multiset<T> {
    pub fn new() -> Self { Self(None) }

    pub fn size(&self) -> usize { Node::size(self.0.as_ref()) }

    pub fn lower_bound(
        &self,
        v: &T,
    ) -> usize {
        Node::binary_search(|x| x >= v, self.0.as_ref())
    }

    pub fn upper_bound(
        &self,
        v: &T,
    ) -> usize {
        Node::binary_search(|x| x > v, self.0.as_ref())
    }

    pub fn count(
        &self,
        v: &T,
    ) -> usize {
        self.upper_bound(v) - self.lower_bound(v)
    }

    pub fn contains(
        &self,
        v: &T,
    ) -> bool {
        self.count(v) > 0
    }

    pub fn insert(
        &mut self,
        v: T,
    ) {
        let i = self.lower_bound(&v);

        self.0 = Node::insert(self.0.take(), i, Some(Node::new(v)));
    }

    pub fn remove(
        &mut self,
        v: &T,
    ) {
        if !self.contains(v) {
            return;
        }

        let i = self.lower_bound(v);

        self.0 = Node::pop(self.0.take().unwrap(), i).1;
    }

    pub fn get(
        &mut self,
        i: usize,
    ) -> &T {
        self.0 = Some(Node::splay(self.0.take().unwrap(), i));

        &self.0.as_ref().unwrap().v
    }
}

#[cfg(test)]

mod tests {

    use super::*;

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
            let mut s = Multiset::new();

            s.insert(0);

            s.insert(l);

            for ((t, x), ans) in q {
                if t == 1 {
                    s.insert(x);
                } else {
                    let i = s.lower_bound(&x);

                    dbg!(i);

                    let hi = *s.get(i);

                    let lo = *s.get(i - 1);

                    assert_eq!(hi - lo, ans);
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
            let mut s = Multiset::new();

            for ((t, x), ans) in q {
                if t == 1 {
                    s.insert(x);
                } else {
                    let v = *s.get(x - 1);

                    assert_eq!(v, ans);

                    s.remove(&v);
                }
            }
        }
    }
}
