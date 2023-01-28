pub struct SWAGQueue {
    st_r: Vec<i64>,
    vr: i64,
    st_l: Vec<i64>,
}

impl SWAGQueue {
    pub fn new() -> Self {
        Self { st_r: vec![], vr: std::i64::MIN, st_l: vec![std::i64::MIN] }
    }

    pub fn size(&self) -> usize { self.st_r.len() + self.st_l.len() - 1 }

    pub fn push(
        &mut self,
        x: i64,
    ) {
        self.vr = self.vr.max(x);

        self.st_r.push(x);
    }

    pub fn pop(&mut self) {
        if self.st_l.len() > 1 {
            self.st_l.pop();

            return;
        }

        assert!(!self.st_r.is_empty());

        while let Some(x) = self.st_r.pop() {
            self.st_l.push(x.max(*self.st_l.last().unwrap()));
        }

        self.vr = std::i64::MIN;

        self.st_l.pop();
    }

    pub fn fold(&self) -> i64 { self.vr.max(*self.st_l.last().unwrap()) }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let mut swag = SWAGQueue::new();

        assert_eq!(swag.fold(), std::i64::MIN);

        swag.push(1);

        assert_eq!(swag.fold(), 1);

        swag.push(2);

        assert_eq!(swag.fold(), 2);

        swag.pop();

        assert_eq!(swag.fold(), 2);

        swag.pop();

        assert_eq!(swag.fold(), std::i64::MIN);
    }
}
