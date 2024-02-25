#![allow(private_bounds)]
// Types to help process the transposition table parameters through generic const expressions
trait TableParams {
    type KeyType;
    type ValueType;
    const SIZE: usize;  // The size of the transposition table
}

struct Params<const SIZE_BITS: usize>{}
impl<const SIZE_BITS: usize> TableParams for Params<SIZE_BITS> {
    type KeyType = u32;
    type ValueType = u8;
    const SIZE: usize = calc_next_prime(1 << SIZE_BITS);
}

/// A transposition table is a cache of previously computed positions.
/// It is used to avoid recomputing the same position multiple times.
/// The table is indexed by a hash of the position, and stores the score of the position for the current player.
/// The table has a size of approximately 2^SIZE_BITS entries. For 23 bits, the table is approximately 40MB large.
pub struct TranspositionTable<const SIZE_BITS: usize>
where
    [(); Params::<SIZE_BITS>::SIZE]: Sized,
{
    keys: Box<[<Params<SIZE_BITS> as TableParams>::KeyType; Params::<SIZE_BITS>::SIZE]>,
    scores: Box<[<Params<SIZE_BITS> as TableParams>::ValueType; Params::<SIZE_BITS>::SIZE]>,
}

impl<const SIZE_BITS: usize> TranspositionTable<SIZE_BITS>
where
    [(); Params::<SIZE_BITS>::SIZE]: Sized,
{
    pub fn new() -> Self {
        Self {
            keys: Box::new([0; Params::<SIZE_BITS>::SIZE]),
            scores: Box::new([0; Params::<SIZE_BITS>::SIZE]),
        }
    }

    pub fn get(&self, key: u64) -> Option<u8> {
        let index = key as usize % Params::<SIZE_BITS>::SIZE;
        let entry = self.keys[index];
        if entry == key as <Params<SIZE_BITS> as TableParams>::KeyType {
            Some(self.scores[index])
        } else {
            None
        }
    }

    pub fn set(&mut self, key: u64, score: u8) {
        let index = key as usize % Params::<SIZE_BITS>::SIZE;
        self.keys[index] = key as <Params<SIZE_BITS> as TableParams>::KeyType;
        self.scores[index] = score;
    }

    pub fn clear(&mut self) {
        self.keys.fill(0);
        self.scores.fill(0);
    }
}

impl<const SZ_BITS: usize> Default for TranspositionTable<SZ_BITS>
where
    [(); Params::<SZ_BITS>::SIZE]: Sized,
{
    fn default() -> Self {
        Self::new()
    }
}

// Helper functions to calculate the next prime number close to a given number
const fn calc_next_prime(n: usize) -> usize {
    if has_factor(n, 2, n) {
        calc_next_prime(n + 1)
    } else {
        n
    }
}

const fn has_factor(n: usize, min: usize, max: usize) -> bool {
    if min * min > n {
        false
    } else if min + 1 >= max {
        n % min == 0
    } else {
        has_factor(n, min, med(min, max)) || has_factor(n, med(min, max), max)
    }
}

const fn med(min: usize, max: usize) -> usize {
    (min + max) / 2
}
