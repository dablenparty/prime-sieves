#![warn(clippy::all, clippy::pedantic)]

use std::collections::{hash_map::Entry, HashMap};

/// Adapted from the Haskell implementation by
/// [Melissa O'Neill](https://www.cs.hmc.edu/~oneill/papers/Sieve-JFP.pdf)
struct Primes {
    x: u64,
    table: HashMap<u64, Vec<u64>>,
}

impl Iterator for Primes {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(facts) = self.table.remove(&self.x) {
            // this could be done with iter chains, but that requires
            // unnecessary cloning
            for (k, v) in facts.iter().map(|&fact| (self.x + fact, vec![fact])) {
                match self.table.entry(k) {
                    Entry::Occupied(mut e) => {
                        e.get_mut().extend(v);
                    }
                    Entry::Vacant(e) => {
                        e.insert(v);
                    }
                }
            }
            self.x += 1;
            self.next()
        } else {
            let x = self.x;
            self.table.insert(x * x, vec![x]);
            self.x += 1;
            Some(x)
        }
    }
}

impl Primes {
    fn new() -> Self {
        Self {
            x: 2,
            table: HashMap::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_five_primes() {
        let expected = vec![2, 3, 5, 7, 11];
        let actual: Vec<_> = Primes::new().take(5).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_first_100_primes() {
        let expected = vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179,
            181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271,
            277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379,
            383, 389, 397, 401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479,
            487, 491, 499, 503, 509, 521, 523, 541,
        ];
        let actual: Vec<_> = Primes::new().take(100).collect();
        assert_eq!(expected, actual);
    }
}
