pub trait Root {
    fn root(
        &mut self,
        u: usize,
    ) -> usize;
}

#[allow(private_in_public)]

pub trait Size {
    fn size(&self) -> usize;
}

pub trait Unite {
    fn unite(
        &mut self,
        u: usize,
        v: usize,
    );
}

/// size of the component containing the node

pub trait SizeOf {
    fn size_of(
        &mut self,
        u: usize,
    ) -> usize;
}

pub trait Same {
    fn same(
        &mut self,
        u: usize,
        v: usize,
    ) -> bool;
}

impl<U: Root> Same for U {
    fn same(
        &mut self,
        u: usize,
        v: usize,
    ) -> bool {
        self.root(u) == self.root(v)
    }
}

pub trait Labels {
    fn labels(&mut self) -> Vec<usize>;
}

impl<U: Root + Size> Labels for U {
    /// same label -> same component.

    fn labels(&mut self) -> Vec<usize> {
        let n = self.size();

        let mut lb = vec![n; n];

        let mut l = 0;

        for i in 0..n {
            let r = self.root(i);

            if lb[r] == n {
                lb[r] = l;

                l += 1;
            }

            lb[i] = lb[r];
        }

        lb
    }
}
