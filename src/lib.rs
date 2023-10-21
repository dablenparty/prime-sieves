#![warn(clippy::all, clippy::pedantic)]
//! This crate contains various incremental prime sieves, adapted from the
//! Haskell implementations by [Melissa O'Neill](https://www.cs.hmc.edu/~oneill/papers/Sieve-JFP.pdf).

pub mod pq;
pub mod map;
