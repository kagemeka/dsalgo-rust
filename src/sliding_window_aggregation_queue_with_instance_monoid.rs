pub trait Monoid {
    type T;

    fn op(
        &self,
        _: Self::T,
        _: Self::T,
    ) -> Self::T;

    fn e(&self) -> Self::T;
}

pub struct SWAGQueue<M: Monoid> {
    m: M,
    st_r: Vec<M::T>, // data
    vr: M::T,
    st_l: Vec<M::T>, // vl
}

impl<M: Monoid> SWAGQueue<M>
where
    M::T: Clone,
{
    pub fn new(m: M) -> Self {
        let vr = m.e();

        let vl = m.e();

        Self { m, st_r: vec![], vr, st_l: vec![vl] }
    }

    pub fn size(&self) -> usize { self.st_r.len() + self.st_l.len() - 1 }

    pub fn push(
        &mut self,
        x: M::T,
    ) {
        self.vr = self.m.op(self.vr.clone(), x.clone());

        self.st_r.push(x);
    }

    pub fn pop(&mut self) {
        if self.st_l.len() > 1 {
            self.st_l.pop();

            return;
        }

        assert!(!self.st_r.is_empty());

        while let Some(x) = self.st_r.pop() {
            self.st_l.push(self.m.op(x, self.st_l.last().unwrap().clone()));
        }

        self.vr = self.m.e();

        self.st_l.pop();
    }

    pub fn fold(&self) -> M::T {
        self.m.op(self.st_l.last().unwrap().clone(), self.vr.clone())
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        struct M;

        impl Monoid for M {
            type T = i64;

            fn op(
                &self,
                l: i64,
                r: i64,
            ) -> i64 {
                l + r
            }

            fn e(&self) -> i64 { 0 }
        }

        let mut swag = SWAGQueue::new(M {});

        assert_eq!(swag.fold(), 0);

        swag.push(1);

        assert_eq!(swag.fold(), 1);

        swag.push(2);

        assert_eq!(swag.fold(), 3);

        swag.pop();

        assert_eq!(swag.fold(), 2);

        swag.pop();

        assert_eq!(swag.fold(), 0);
    }
}
