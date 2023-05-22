use std::time::Instant;

use data_generator::DataGenerator;
use treap::Treap;

use crate::{data_generator::Action, dynamic_array::DynamicArray};

mod data_generator;
mod dynamic_array;
mod random;
mod treap;

type ID = u64;
type Key = u32;
type Element = (ID, Key);

fn main() {
    let now = Instant::now();
    experiment_0();
    println!("Experiment 0 took: {:?}", now.elapsed());

    let now = Instant::now();
    experiment_1();
    println!("Experiment 1 took: {:?}", now.elapsed());
}

fn experiment_0() {
    for _ in 0..100 {
        let mut treap = Treap::new();
        for i in 1..=1024 {
            treap.insert((i as u64, i));
        }
        let depths = treap.get_depths();
        println!("{}", depths.iter().sum::<usize>() as f64 / depths.len() as f64);
    }
}

fn experiment_1() {
    let mut data_generator = DataGenerator::new();
    for num_ins in [100_000, 200_000, 500_000, 800_000, 1_000_000] {
        let mut es = Vec::with_capacity(num_ins);
        for _ in 0..num_ins {
            match data_generator.gen_insertion() {
                Action::Insertion(e) => es.push(e),
                _ => panic!(),
            }
        }

        let now = Instant::now();
        let mut treap = Treap::new();
        for &e in &es {
            treap.insert(e);
        }
        println!("{num_ins} insertions into treap took: {:?}", now.elapsed());

        let now = Instant::now();
        let mut dynamic_array = DynamicArray::new();
        for &e in &es {
            dynamic_array.insert(e);
        }
        println!("{num_ins} insertions into dynamic array took: {:?}", now.elapsed());
    }
}
