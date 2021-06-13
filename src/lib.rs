#![recursion_limit = "512"]
mod data_structures;
mod expert;
mod heuristic;
mod linguist_aliases;
mod linguist_extensions;
mod linguist_filenames;
mod linguist_heuristics;
mod linguist_interpreters;
mod modeline;
mod shebang;

pub use crate::expert::expert;
pub use crate::expert::Guess;
