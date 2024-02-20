/// A transposition table is a cache of previously computed positions.
/// It is used to avoid recomputing the same position multiple times.
/// The table is indexed by a hash of the position, and stores the score of the position for the current player.
/// The table has a fixed size of 2^23 entries, amounting to 40MB of memory.
pub struct TranspositionTable {
    keys: Box<[u32; Self::SIZE]>,
    scores: Box<[u8; Self::SIZE]>,
}

impl TranspositionTable {
    const SIZE: usize = 8388617; // 1 << 23 + 9

    pub fn new() -> Self {
        Self {
            keys: Box::new([0; Self::SIZE]),
            scores: Box::new([0; Self::SIZE]),
        }
    }

    pub fn get(&self, key: u64) -> Option<u8> {
        let index = key as usize % Self::SIZE;
        let entry = self.keys[index];
        if entry == key as u32 {
            Some(self.scores[index])
        } else {
            None
        }
    }

    pub fn set(&mut self, key: u64, score: u8) {
        let index = key as usize % Self::SIZE;
        self.keys[index] = key as u32;
        self.scores[index] = score;
    }

    pub fn clear(&mut self) {
        self.keys.fill(0);
        self.scores.fill(0);
    }
}

impl Default for TranspositionTable {
    fn default() -> Self {
        Self::new()
    }
}
