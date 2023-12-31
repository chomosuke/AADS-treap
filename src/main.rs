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
    let mut all_depth = vec![Vec::new(); 1024];
    for _ in 0..100 {
        let mut treap = Treap::new();
        for i in 1..=1024 {
            treap.insert((i as u64, i));
        }
        treap.validate();
        for i in 0..1024 {
            all_depth[i].push(treap.get_depth(i as u32 + 1).unwrap());
        }
        println!(
            "{},{}",
            treap.get_depth(512).unwrap(),
            treap.get_depths().iter().max().unwrap() + 1,
        );
    }
    for d in all_depth{
        println!("{}", d.iter().sum::<usize>() as f64 / d.len() as f64);
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
        print!("{num_ins},{}", now.elapsed().as_millis());
        treap.validate();

        let now = Instant::now();
        let mut dynamic_array = DynamicArray::new();
        for &e in &es {
            dynamic_array.insert(e);
        }
        println!(",{}", now.elapsed().as_millis());
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
        print!("{}%,{}", p_del * 100.0, now.elapsed().as_millis());
        treap.validate();

        let now = Instant::now();
        let mut dynamic_array = DynamicArray::new();
        for &e in &es {
            match e {
                Action::Insertion(e) => dynamic_array.insert(e),
                Action::Deletion(k) => dynamic_array.delete(k),
                Action::Search(_) => unreachable!(),
            }
        }
        println!(",{}", now.elapsed().as_millis());
    }
}

fn experiment_3() {
    let mut data_generator = DataGenerator::new();
    for p_ser in [0.001, 0.005, 0.01, 0.05, 0.1] {
        let mut es = Vec::with_capacity(1_000_000);
        for _ in 0..1_000_000 {
            let a = if rand::random::<f32>() < p_ser {
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
        print!("{}%,{}", p_ser * 100.0, now.elapsed().as_millis());
        treap.validate();

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
        println!(",{}", now.elapsed().as_millis());
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
        print!("{num_ins},{}", now.elapsed().as_millis());
        treap.validate();

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
        println!(",{}", now.elapsed().as_millis());
    }
}
