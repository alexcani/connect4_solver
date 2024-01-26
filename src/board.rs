//! This module contains the board trait and all board implementations

mod bitboard;

pub use bitboard::BitBoard;

use strum_macros::{EnumCount, EnumIter, FromRepr};

pub const WIDTH: usize = 7;
pub const HEIGHT: usize = 6;

#[derive(Copy, Clone, PartialEq, Debug, EnumIter, FromRepr, EnumCount)]
pub enum Column {
    A = 0,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl From<char> for Column {
    fn from(c: char) -> Self {
        match c {
            '1' | 'A' | 'a' => Column::A,
            '2' | 'B' | 'b' => Column::B,
            '3' | 'C' | 'c' => Column::C,
            '4' | 'D' | 'd' => Column::D,
            '5' | 'E' | 'e' => Column::E,
            '6' | 'F' | 'f' => Column::F,
            '7' | 'G' | 'g' => Column::G,
            _ => panic!("Invalid column"),
        }
    }
}

/// A Connect 4 board that can be played on or passed into a solver
pub trait Board: Copy {
    /// Checks if a given column is playable, i.e. if there is still space in the column
    fn is_playable(&self, column: Column) -> bool;

    /// Checks if playing a piece in the given column would result in a win by the current player
    fn is_winning(&self, column: Column) -> bool;

    /// Plays a piece in the given column, if possible
    /// Returns the number of played moves.
    /// One can compare to the previous number of moves to check if the move was valid
    /// A winning move is considered not valid for this function. Before playing, one should check whether the move is winning
    /// by calling [Board::is_winning()]
    fn play(&mut self, column: Column) -> u32;

    /// Returns the number of moves made so far
    fn number_of_moves(&self) -> u32;

    /// Returns if this given board implementation supports hashing and has a key
    fn has_key(&self) -> bool {
        false
    }

    /// Returns the key of the board if it exists, 0 otherwise.
    /// This function should only be called if [Board::has_key()] returns true, since 0 is a valid key.
    fn key(&self) -> u64 {
        0
    }
}
