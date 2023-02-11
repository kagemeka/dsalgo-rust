pub fn group_by_id(ids: Vec<usize>) -> Vec<Vec<usize>> {
    let k = ids.iter().max().unwrap() + 1;

    let mut comp = vec![vec![]; k];

    for (u, id) in ids.into_iter().enumerate() {
        comp[id].push(u);
    }

    comp
}
