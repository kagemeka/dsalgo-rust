pub fn labels_to_groups(labels: &[usize]) -> Vec<Vec<usize>> {
    let n = labels.len();

    let k = *labels.iter().max().unwrap() + 1;

    let mut groups = vec![vec![]; k];

    for i in 0..n {
        groups[labels[i]].push(i);
    }

    groups
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let labels = vec![0, 2, 1, 0];

        assert_eq!(
            labels_to_groups(&labels),
            vec![vec![0, 3], vec![2], vec![1]]
        );
    }
}
