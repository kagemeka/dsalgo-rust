pub fn euler_tour_nodes(tour_edges: &[isize]) -> Vec<usize> {
    let size = tour_edges.len() - 1;

    let mut tour = vec![0; size];

    let mut st = vec![!tour_edges[size] as usize];

    for (i, &u) in tour_edges[..size].iter().enumerate().rev() {
        if u < 0 {
            let u = !u as usize;

            tour[i] = *st.last().unwrap();

            st.push(u);
        } else {
            tour[i] = u as usize;

            st.pop();
        }
    }

    tour
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        use crate::euler_tour_edges::euler_tour_edges;

        let g = vec![vec![1, 2], vec![0, 3], vec![0], vec![1]];

        let tour_edges = euler_tour_edges(&g, 0);

        assert_eq!(euler_tour_nodes(&tour_edges), [0, 1, 3, 1, 0, 2, 0]);
    }
}
