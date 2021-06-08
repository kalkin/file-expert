#![recursion_limit = "512"]
mod data_structures;
mod expert;
mod heuristic;
mod linguist_extensions;
mod linguist_heuristics;
mod modeline;
mod shebang;
mod linguist_interpreters;
mod linguist_aliases;
mod linguist_filenames;

pub use crate::expert::expert;
pub use crate::expert::ExpertResult;
