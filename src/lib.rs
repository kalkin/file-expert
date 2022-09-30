//
// Copyright (c) 2018-2020 Bahtiar `kalkin-` Gadimov.
//
// This file is part of file-expert
// (see https://github.com/kalkin/file-expert).
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.
//
#![recursion_limit = "512"]
#![allow(missing_docs)]
/// Api for file-expert
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

/// Guess the file type
pub use crate::expert::guess;
/// A file type guess result
pub use crate::expert::Guess;
