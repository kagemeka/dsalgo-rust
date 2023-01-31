pub struct Stack<T>(Vec<T>);

impl<T> Stack<T> {
    pub fn size(&self) -> usize { self.0.len() }

    pub fn new() -> Self { Self(vec![]) }

    pub fn push(
        &mut self,
        x: T,
    ) {
        self.0.push(x)
    }

    pub fn pop(&mut self) -> T { self.0.pop().unwrap() }

    pub fn top(&mut self) -> &T { self.0.last().unwrap() }

    pub fn is_empty(&self) -> bool { self.size() == 0 }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let mut st = Stack::new();

        st.push(3);

        st.push(2);

        assert_eq!(st.size(), 2);

        assert_eq!(st.top(), &2);

        assert_eq!(st.size(), 2);

        assert_eq!(st.pop(), 2);

        assert_eq!(st.size(), 1);
    }
}
