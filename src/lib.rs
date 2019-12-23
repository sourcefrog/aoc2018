// Copyright 2018 Google LLC
// 
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// 
//     https://www.apache.org/licenses/LICENSE-2.0
// 
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! mbp AoC2018 solutions - general utilities.

pub mod bisection_search;
pub mod matrix;
mod point;
mod shortest_path;

pub use crate::bisection_search::bisection_search;
pub use crate::matrix::Matrix;
pub use crate::point::{point, Point};
pub use crate::shortest_path::shortest_distance;
