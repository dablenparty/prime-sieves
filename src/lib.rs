#![warn(clippy::all, clippy::pedantic)]

use std::{cmp::Reverse, collections::BinaryHeap};

pub type MinPriorityQueue = BinaryHeap<Reverse<(u64, u64)>>;

/// Adapted from the Haskell implementation by
/// [Melissa O'Neill](https://www.cs.hmc.edu/~oneill/papers/Sieve-JFP.pdf)
pub struct PrimesIter {
    x: u64,
    candidate: (u64, u64),
    table: MinPriorityQueue,
}

impl Iterator for PrimesIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x == 2 {
            self.x += 1;
            return Some(2);
        } else if self.x == 3 {
            self.x += 2;
            self.table.push(Reverse((9, 6)));
            return Some(3);
        }

        assert!(self.x % 2 != 0, "x must be odd");

        loop {
            if self.x < self.candidate.0 {
                let x = self.x;
                self.table.push(Reverse((x * x, x * 2)));
                self.x += 2;
                return Some(x);
            }
            while self.x == self.candidate.0 {
                self.table.push(Reverse((
                    self.candidate.0 + self.candidate.1,
                    self.candidate.1,
                )));
                self.candidate = self.table.pop().unwrap().0;
            }
            self.x += 2;
        }
    }
}

impl PrimesIter {
    #[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
    pub fn new() -> Self {
        Self {
            x: 2,
            candidate: (9, 6),
            table: MinPriorityQueue::default(),
        }
    }
}

impl Default for PrimesIter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_five_primes() {
        let expected = vec![2, 3, 5, 7, 11];
        let actual: Vec<_> = PrimesIter::default().take(5).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_next_100_primes() {
        // Skips the first 100 primes and takes the next 100. This is more
        // of a sanity check to make sure that it isn't too slow.
        let expected = vec![
            547, 557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619, 631, 641, 643,
            647, 653, 659, 661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743, 751, 757,
            761, 769, 773, 787, 797, 809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863, 877,
            881, 883, 887, 907, 911, 919, 929, 937, 941, 947, 953, 967, 971, 977, 983, 991, 997,
            1009, 1013, 1019, 1021, 1031, 1033, 1039, 1049, 1051, 1061, 1063, 1069, 1087, 1091,
            1093, 1097, 1103, 1109, 1117, 1123, 1129, 1151, 1153, 1163, 1171, 1181, 1187, 1193,
            1201, 1213, 1217, 1223,
        ];
        let actual: Vec<_> = PrimesIter::default().skip(100).take(100).collect();
        assert_eq!(expected, actual);
    }
}
