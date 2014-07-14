//! Miscelaneous, unsorted generic geometric utilities.

pub use utils::center::center;
pub use utils::triangulate::triangulate;
pub use utils::project_homogeneous::{project_homogeneous, project_homogeneous_to};
pub use utils::triangle_utils::{circumcircle, is_affinely_dependent_triangle};
pub use utils::cleanup::remove_unused_points;
pub use utils::derivatives::{dcos, dsin, binom};
pub use utils::optimization::{maximize_with_newton, newton, minimize_with_bfgs, bfgs};


pub mod symbolic;
mod center;
mod triangulate;
mod project_homogeneous;
mod triangle_utils;
mod cleanup;
mod derivatives;
mod optimization;