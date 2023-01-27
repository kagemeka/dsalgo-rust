pub struct Node<T> {
    is_red: bool,
    size: usize,
    pub(crate) v: T,
    l: ON<T>,
    r: ON<T>,
}

type N<T> = Box<Node<T>>;

type ON<T> = Option<Box<Node<T>>>;

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
