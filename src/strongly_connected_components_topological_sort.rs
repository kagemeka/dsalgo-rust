pub fn toposort(labels: Vec<usize>) -> Vec<usize> {
    let k = *labels.iter().max().unwrap();

    labels.into_iter().map(|l| k - l).collect()
}
