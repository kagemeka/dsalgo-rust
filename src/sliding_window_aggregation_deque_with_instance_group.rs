use std::collections::VecDeque;

pub trait Group {
    type T;

    fn op(
        &self,
        _: Self::T,
        _: Self::T,
    ) -> Self::T;

    fn e(&self) -> Self::T;

    fn inv(
        &self,
        _: Self::T,
    ) -> Self::T;
}

pub struct SWAGDeque<G: Group> {
    g: G,
    v: G::T,
    que: VecDeque<G::T>,
}

impl<G: Group> SWAGDeque<G>
where
    G::T: Clone,
{
    pub fn new(g: G) -> Self {
        let v = g.e();

        Self { g, v, que: VecDeque::new() }
    }

    pub fn size(&self) -> usize { self.que.len() }

    pub fn push_right(
        &mut self,
        x: G::T,
    ) {
        self.v = self.g.op(self.v.clone(), x.clone());

        self.que.push_back(x);
    }

    pub fn push_left(
        &mut self,
        x: G::T,
    ) {
        self.v = self.g.op(x.clone(), self.v.clone());

        self.que.push_front(x);
    }

    pub fn pop_left(&mut self) {
        let inv = self.g.inv(self.que.pop_front().unwrap());

        self.v = self.g.op(inv, self.v.clone());
    }

    pub fn pop_right(&mut self) {
        let inv = self.g.inv(self.que.pop_back().unwrap());

        self.v = self.g.op(self.v.clone(), inv);
    }

    pub fn fold(&self) -> G::T { self.v.clone() }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        struct G;

        impl Group for G {
            type T = i64;

            fn op(
                &self,
                l: i64,
                r: i64,
            ) -> i64 {
                l + r
            }

            fn e(&self) -> i64 { 0 }

            fn inv(
                &self,
                x: i64,
            ) -> i64 {
                -x
            }
        }

        let mut swag = SWAGDeque::new(G {});

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
