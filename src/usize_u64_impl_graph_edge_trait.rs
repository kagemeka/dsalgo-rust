type E = (usize, u64);

use crate::graph_edge_trait::{
    To,
    Weight,
};

impl To for E {
    type V = usize;

    fn to(&self) -> &usize { &self.0 }
}

impl Weight for E {
    type T = u64;

    fn weight(&self) -> &u64 { &self.1 }
}
