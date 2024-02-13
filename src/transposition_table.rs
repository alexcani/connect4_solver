#[derive(Default)]
struct Entry {
    data: u64
}

impl Entry {
    const KEY_MASK: u64 = 0xFFFFFFFFFFFFFF;  // 56 bits
    const SCORE_MASK: u64 = 0xFF00000000000000;  // 8 bits

    fn new(key: u64, score: u8) -> Self {
        Self { data: (key & Entry::KEY_MASK) | ((score as u64) << 56)}
    }

    fn key(&self) -> u64 {
        self.data & Entry::KEY_MASK
    }

    fn score(&self) -> u8 {
        ((self.data & Entry::SCORE_MASK) >> 56) as u8
    }
}

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
        i.clear();
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
        self.table[index] = Entry::new(key, score);
    }

    pub fn clear(&mut self) {
        self.table.resize_with(self.table.capacity(), Default::default);
    }
}

impl Default for TranspositionTable {
    fn default() -> Self {
        // 8388617 is the closest prime number that will result in a 64MB table
        Self::new(8388617)
    }
}
