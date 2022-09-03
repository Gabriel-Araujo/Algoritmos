use rand::prelude::*;
use rand::distributions::{Standard, Distribution};

pub fn generate<T>(size: usize) -> Vec<T> where Standard: Distribution<T> {
    let mut new_vector: Vec<T> = Vec::with_capacity(size);

    for _ in 0..size {
        new_vector.push(thread_rng().gen());
    }

    new_vector
}