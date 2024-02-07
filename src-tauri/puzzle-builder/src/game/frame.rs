use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct GameFrame {
    pub(crate) x: usize,
    pub(crate) y: usize,
}
