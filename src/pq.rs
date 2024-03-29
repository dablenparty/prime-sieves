use std::{cmp::Reverse, collections::BinaryHeap};

/// Type alias for `BinaryHeap<Reverse<(u64, u64)>>`
pub type MinPriorityQueue<T> = BinaryHeap<Reverse<(T, T)>>;

/// An iterator that yields prime numbers.
///
/// This implementation uses a [`MinPriorityQueue`] under the hood to
/// track and generate composite multiples just-in-time.
///
/// # Examples
///
/// Basic usage:
/// ```rust
/// # use rust_primes::pq::QueuePrimesIter;
/// let primes: Vec<_> = QueuePrimesIter::default().take(5).collect();
/// let expected = vec![2, 3, 5, 7, 11];
/// assert_eq!(primes, expected);
/// ```
pub struct QueuePrimesIter {
    candidate: u64,
    /// Format: `(composite, step)`
    next_composite: (u64, u64),
    pq: MinPriorityQueue<u64>,
}

impl Iterator for QueuePrimesIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        const CANDIDATE_STEP: u64 = 2;

        match self.candidate {
            0 | 1 => unreachable!(),
            2 => {
                self.candidate += 1;
                return Some(2);
            }
            3 => {
                self.candidate += CANDIDATE_STEP;
                self.pq_push((9, 6));
                return Some(3);
            }
            _ => {}
        }

        // this should absolutely never happen
        assert_ne!(
            self.candidate % 2,
            0,
            "x must be odd but x = {}",
            self.candidate
        );

        loop {
            if self.candidate < self.next_composite.0 {
                let x = self.candidate;
                self.pq_push((x * x, x * CANDIDATE_STEP));
                self.candidate += CANDIDATE_STEP;
                return Some(x);
            }

            while self.candidate == self.next_composite.0 {
                let (composite, step) = self.next_composite;
                self.pq_push((composite + step, step));
                self.next_composite = self.pq.pop().unwrap().0;
            }
            self.candidate += CANDIDATE_STEP;
        }
    }
}

impl QueuePrimesIter {
    #[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
    fn new() -> Self {
        Self {
            candidate: 2,
            next_composite: (9, 6),
            pq: MinPriorityQueue::default(),
        }
    }

    /// Utility function to make for cleaner code.
    fn pq_push(&mut self, t: (u64, u64)) {
        self.pq.push(Reverse(t));
    }
}

impl Default for QueuePrimesIter {
    fn default() -> Self {
        Self::new()
    }
}
