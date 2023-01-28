pub fn bellman_ford_abstract<E, F>(
    edges: &[E],
    mut f: F,
) where
    F: FnMut(&E) -> bool,
{
    let mut updated;

    loop {
        updated = false;

        for e in edges {
            updated |= f(e);
        }

        if !updated {
            break;
        }
    }
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
