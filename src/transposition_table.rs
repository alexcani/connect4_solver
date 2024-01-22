use static_assertions as sa;
use modular_bitfield::prelude::*;

#[bitfield]
struct Entry {
    key: B56,
    score: u8,
}

sa::assert_eq_size!(Entry, u64);

/// A transposition table is a cache of previously computed positions.
/// It is used to avoid recomputing the same position multiple times.
/// The table is indexed by a hash of the position, and stores the score of the position for the current player.
/// To create a new table with the default size of 64MB, use [`TranspositionTable::default`], otherwise use [`TranspositionTable::new`] with
/// the desired number of entries (each entry is 8 bytes long). Ideally the table size should be a prime number.
pub struct TranspositionTable {
    table: Vec<Entry>,
}

impl TranspositionTable {
    pub fn new(size: usize) -> Self {
        let mut i = Self {
            table: Vec::with_capacity(size)
        };
        i.table.resize_with(size, Entry::new);
        i
    }

    pub fn get(&self, key: u64) -> Option<u8> {
        let index = key as usize % self.table.len();
        let entry = &self.table[index];
        if entry.key() == key {
            Some(entry.score())
        } else {
            None
        }
    }

    pub fn set(&mut self, key: u64, score: u8) {
        let index = key as usize % self.table.len();
        self.table[index] = Entry::new().with_key(key).with_score(score);
    }
}

impl Default for TranspositionTable {
    fn default() -> Self {
        // 8388617 is the closest prime number that will result in a 64MB table
        Self::new(8388617)
    }
}
