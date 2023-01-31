use crate::singly_linked_list_node_with_box::*;

pub struct Stack<T> {
    top: Option<Box<Node<T>>>,
    size: usize,
}

impl<T> Stack<T> {
    pub fn new() -> Self { Self { top: None, size: 0 } }

    pub fn size(&self) -> usize { self.size }

    pub fn is_empty(&self) -> bool { self.size == 0 }

    pub fn top(&self) -> Option<&T> {
        if self.size == 0 {
            None
        } else {
            Some(&self.top.as_ref().unwrap().value)
        }
    }

    pub fn push(
        &mut self,
        x: T,
    ) {
        let mut x = Node::new(x);

        x.next = self.top.take();

        self.top = Some(x);

        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T>
    where
        T: Clone + Copy,
    {
        if self.size == 0 {
            return None;
        }

        let mut node = self.top.take().unwrap();

        self.top = node.next.take();

        self.size -= 1;

        Some(node.value.clone())
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let mut st = Stack::new();

        st.push(2);

        st.push(1);

        assert_eq!(st.pop(), Some(1));

        st.push(3);

        assert_eq!(st.pop(), Some(3));

        assert_eq!(st.pop(), Some(2));

        assert_eq!(st.pop(), None);
    }
}
