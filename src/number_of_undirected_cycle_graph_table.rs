use std::ops::*;

use crate::number_of_undirected_path_graph_table::number_of_path_graph;

pub fn number_of_cycle_graph<T>(size: usize) -> Vec<T>
where
    T: Clone + Mul<Output = T> + From<usize>,
{
    let mut f: Vec<T> = vec![0.into()];

    f.append(&mut number_of_path_graph(size - 1));

    f
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let f = number_of_cycle_graph::<usize>(7);

        assert_eq!(f, [0, 0, 1, 1, 3, 12, 60]);
    }
}
