pub mod dijkstra_sparse_from_potential {

    //! consider shortest path problem as a linear programming.

    pub fn dijkstra(
        mut g: Vec<Vec<(usize, u64)>>,
        initial_potential: Vec<u64>,
    ) -> Vec<u64> {
        use crate::dijkstra_sparse::dijkstra as f;

        let n = g.len();

        assert_eq!(initial_potential.len(), n);

        g.push(initial_potential.into_iter().enumerate().collect());

        f(&g, n)[..n].to_vec()
    }
}
