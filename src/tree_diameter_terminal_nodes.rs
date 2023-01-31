use crate::tree_diameter_path_unweighted::diameter_path;

pub fn diameter_ends(g: &[Vec<usize>]) -> (usize, usize) {
    let path = diameter_path(g);

    (path[0], *path.last().unwrap())
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
