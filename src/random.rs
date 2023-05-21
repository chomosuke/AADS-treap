use lazy_static::lazy_static;
use rand::{distributions::Uniform, prelude::Distribution};

use crate::Key;

lazy_static! {
    static ref DIST: Uniform<Key> = Uniform::from(0..=10_000_000);
}

pub fn random_10_7() -> u32 {
    DIST.sample(&mut rand::thread_rng())
}
