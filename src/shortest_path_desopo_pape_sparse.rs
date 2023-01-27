pub fn desopo_pape<T>(
    adj_list: &[Vec<(usize, T)>],
    src: usize,
) -> Vec<Option<T>>
where
    T: Ord + std::ops::Add<Output = T> + Copy + Default,
{
    use std::collections::VecDeque;

    let g = adj_list;

    let n = g.len();

    let mut dist = vec![None; n];

    dist[src] = Some(T::default());

    let mut q = VecDeque::new();

    q.push_back(src);

    #[derive(PartialEq, Clone, Copy)]

    enum State {
        OnStack,
        Popped,
        None,
    }

    let mut state = vec![State::None; n];

    while let Some(u) = q.pop_front() {
        state[u] = State::Popped;

        debug_assert!(dist[u].is_some());

        for &(v, w) in g[u].iter() {
            let dv = dist[u].unwrap() + w;

            if dist[v].is_some() && Some(dv) >= dist[v] {
                continue;
            }

            dist[v] = Some(dv);

            match state[v] {
                State::OnStack => continue,
                State::Popped => {
                    q.push_back(v);

                    state[v] = State::OnStack
                }
                State::None => {
                    q.push_front(v);

                    state[v] = State::OnStack;
                }
            }
        }
    }

    dist
}
