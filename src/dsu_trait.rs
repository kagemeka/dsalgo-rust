pub trait FindRoot {
    fn find_root(&mut self, u: usize) -> usize;
}

pub trait Unite {
    fn unite(&mut self, u: usize, v: usize);
}

/// size of the component containing the node
pub trait SizeOf {
    fn size_of(&mut self, u: usize) -> usize;
}
