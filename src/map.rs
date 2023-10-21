use std::collections::{hash_map::Entry, HashMap};

/// An iterator that yields prime numbers.
///
/// This implementation uses a [`HashMap`] under the hood to track
/// composite multiples.
///
/// # Examples
///
/// Basic usage:
/// ```rust
/// # use rust_primes::map::MapPrimesIter;
/// let primes: Vec<_> = MapPrimesIter::default().take(5).collect();
/// let expected = vec![2, 3, 5, 7, 11];
/// assert_eq!(primes, expected);
#[allow(clippy::module_name_repetitions)]
pub struct MapPrimesIter {
    factors_table: HashMap<u64, Vec<u64>>,
    candidate: u64,
}

impl Iterator for MapPrimesIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(factors) = self.factors_table.remove(&self.candidate) {
            // Using iterators and the Entry API would be very clean,
            // but it requires cloning the factors vector which isn't
            // necessary.
            for (k, v) in factors.into_iter().map(|f| (self.candidate + f, vec![f])) {
                match self.factors_table.entry(k) {
                    Entry::Occupied(mut e) => e.get_mut().extend(v),
                    Entry::Vacant(e) => {
                        e.insert(v);
                    }
                }
            }
            self.candidate += 1;
            self.next()
        } else {
            let sq = self.candidate * self.candidate;
            self.factors_table.insert(sq, vec![self.candidate]);
            let prime = self.candidate;
            self.candidate += 1;
            Some(prime)
        }
    }
}

impl MapPrimesIter {
    #[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
    pub fn new() -> Self {
        Self {
            factors_table: HashMap::new(),
            candidate: 2,
        }
    }
}

impl Default for MapPrimesIter {
    fn default() -> Self {
        Self::new()
    }
}
