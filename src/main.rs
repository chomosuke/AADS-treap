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

    let now = Instant::now();
    experiment_2();
    println!("Experiment 2 took: {:?}", now.elapsed());

    let now = Instant::now();
    experiment_3();
    println!("Experiment 3 took: {:?}", now.elapsed());

    let now = Instant::now();
    experiment_4();
    println!("Experiment 4 took: {:?}", now.elapsed());
}

fn experiment_0() {
    for _ in 0..100 {
        let mut treap = Treap::new();
        for i in 1..=1024 {
            treap.insert((i as u64, i));
        }
        let depths = treap.get_depths();
        println!(
            "{}",
            depths.iter().sum::<usize>() as f64 / depths.len() as f64
        );
    }
}

fn experiment_1() {
    let mut data_generator = DataGenerator::new();
    for num_ins in [100_000, 200_000, 500_000, 800_000, 1_000_000] {
        let mut es = Vec::with_capacity(num_ins);
        for _ in 0..num_ins {
            match data_generator.gen_insertion() {
                Action::Insertion(e) => es.push(e),
                _ => unreachable!(),
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
        println!(
            "{num_ins} insertions into dynamic array took: {:?}",
            now.elapsed()
        );
    }
}

fn experiment_2() {
    let mut data_generator = DataGenerator::new();
    for p_del in [0.001, 0.005, 0.01, 0.05, 0.1] {
        let mut es = Vec::with_capacity(1_000_000);
        for _ in 0..1_000_000 {
            let a = if rand::random::<f32>() < p_del {
                data_generator.gen_deletion()
            } else {
                data_generator.gen_insertion()
            };
            es.push(a);
        }

        let now = Instant::now();
        let mut treap = Treap::new();
        for &e in &es {
            match e {
                Action::Insertion(e) => treap.insert(e),
                Action::Deletion(k) => treap.delete(k),
                Action::Search(_) => unreachable!(),
            }
        }
        println!(
            "{}% deletions for treap took: {:?}",
            p_del * 100.0,
            now.elapsed()
        );

        let now = Instant::now();
        let mut dynamic_array = DynamicArray::new();
        for &e in &es {
            match e {
                Action::Insertion(e) => dynamic_array.insert(e),
                Action::Deletion(k) => dynamic_array.delete(k),
                Action::Search(_) => unreachable!(),
            }
        }
        println!(
            "{}% deletions for dynamic array took: {:?}",
            p_del * 100.0,
            now.elapsed(),
        );
    }
}

fn experiment_3() {
    let mut data_generator = DataGenerator::new();
    for p_del in [0.001, 0.005, 0.01, 0.05, 0.1] {
        let mut es = Vec::with_capacity(1_000_000);
        for _ in 0..1_000_000 {
            let a = if rand::random::<f32>() < p_del {
                data_generator.gen_search()
            } else {
                data_generator.gen_insertion()
            };
            es.push(a);
        }

        let now = Instant::now();
        let mut treap = Treap::new();
        for &e in &es {
            match e {
                Action::Insertion(e) => treap.insert(e),
                Action::Deletion(_) => unreachable!(),
                Action::Search(k) => {
                    treap.search(k);
                }
            }
        }
        println!(
            "{}% searches for treap took: {:?}",
            p_del * 100.0,
            now.elapsed()
        );

        let now = Instant::now();
        let mut dynamic_array = DynamicArray::new();
        for &e in &es {
            match e {
                Action::Insertion(e) => dynamic_array.insert(e),
                Action::Deletion(_) => unreachable!(),
                Action::Search(k) => {
                    dynamic_array.search(k);
                }
            }
        }
        println!(
            "{}% searches for dynamic array took: {:?}",
            p_del * 100.0,
            now.elapsed(),
        );
    }
}

fn experiment_4() {
    let mut data_generator = DataGenerator::new();
    for num_ins in [100_000, 200_000, 500_000, 800_000, 1_000_000] {
        let mut es = Vec::with_capacity(num_ins);
        for _ in 0..num_ins {
            let a = if rand::random::<f32>() < 0.1 {
                if rand::random::<f32>() < 0.5 {
                    data_generator.gen_deletion()
                } else {
                    data_generator.gen_search()
                }
            } else {
                data_generator.gen_insertion()
            };
            es.push(a);
        }

        let now = Instant::now();
        let mut treap = Treap::new();
        for &e in &es {
            match e {
                Action::Insertion(e) => treap.insert(e),
                Action::Deletion(k) => treap.delete(k),
                Action::Search(k) => {
                    treap.search(k);
                }
            }
        }
        println!(
            "{num_ins} ops with 5% deleletion & 5% search for treap took: {:?}",
            now.elapsed()
        );

        let now = Instant::now();
        let mut dynamic_array = DynamicArray::new();
        for &e in &es {
            match e {
                Action::Insertion(e) => dynamic_array.insert(e),
                Action::Deletion(k) => dynamic_array.delete(k),
                Action::Search(k) => {
                    dynamic_array.search(k);
                }
            }
        }
        println!(
            "{num_ins} ops with 5% deleletion & 5% search for dynamic array took: {:?}",
            now.elapsed(),
        );
    }
}
