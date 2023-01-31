pub fn edges_to_0_indexed(
    mut edges: Vec<(usize, usize)>
) -> Vec<(usize, usize)> {
    for e in edges.iter_mut() {
        e.0 = e.0 - 1;

        e.1 = e.1 - 1;
    }

    edges
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
