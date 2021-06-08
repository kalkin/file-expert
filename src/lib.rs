#![recursion_limit = "512"]
mod data_structures;
mod expert;
mod heuristic;
mod linguist_heuristics;
mod modeline;
mod shebang;

pub use crate::expert::expert;
pub use crate::expert::ExpertResult;
