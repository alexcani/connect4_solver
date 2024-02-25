#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

pub mod board;
pub mod solver;
mod transposition_table;

pub mod prelude {
    //! The prelude of the connect4_solver crate, containing the most commonly used types and functions.
    pub use crate::board::*;
    pub use crate::solver::*;
    pub use crate::transposition_table::TranspositionTable;
}
