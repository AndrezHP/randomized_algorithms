use std::collections::LinkedList;
use rand::prelude::*;
// use rtrees::rbtree::*;

pub fn random_generator(from: u32, to: u32) -> u32 {
    let mut rng = thread_rng();
    let y: u32 = rng.gen_range(from..to);
    println!("Some random number: {}", y);
    return y;
}

// TODO: implement this
// fn hashing_with_chaining() {
// this is nice
// }

// TODO: implement this
// fn rb_tree() {
// this is also nice
// }

struct SeededHash {
    l: u32,
    a: u32,
    b: u32,
}

impl SeededHash {
    // Multiply shift hashing as from lecture notes (https://arxiv.org/pdf/1504.06804.pdf) at 3.3
    fn hash(&self, x: u32) -> usize {
        let multiply_add: u32 = self.a.wrapping_mul(x).wrapping_add(self.b);
        return multiply_add.wrapping_shr(32 - self.l) as usize;
    }
}

struct HwCDataStructure {
    vec: Vec<Vec<u32>>,
    hash_function: SeededHash,
}
//
// impl HwCDataStructure {
//     fn insert(&mut self, elem: u32) {
//         let hash: usize = self.hash_function(elem);
//         let mut found_array: Vec<u32> = self.vec.get(hash);
//     }
//
//     fn query(&mut self, elem: u32) -> bool {
//         return false;
//     }
// }

struct PerfectHashingDataStructure {
    vec: Vec<u32>,
    hash_function: SeededHash,
}

impl PerfectHashingDataStructure {
    fn insert(&mut self, elem: u32) {
        let hash: usize = self.hash_function.hash(elem);
        println!("insert elem: {}", elem);
        println!("hash index: {}", hash);
        self.vec.insert(hash, self.vec[hash] + 1);
    }

    fn query(&mut self, elem: u32) -> u32 {
        println!("query elem: {}", elem);
        let hash: usize = self.hash_function.hash(elem);
        return self.vec[hash];
    }
}

fn make_random_hash_function(hash_len: u32) -> SeededHash {
    let base: u32 = 2;
    let randomness_size: u32 = base.pow(31);
    let a: u32 = random_generator(1, randomness_size);
    let b: u32 = random_generator(1, randomness_size);
    return SeededHash {
        l: hash_len,
        a,
        b,
    };
}

fn perfect_hashing(input_array: &Vec<u32>) -> PerfectHashingDataStructure {
    let input_len: usize = input_array.len();
    let hash_len = log2u(input_len.pow(2));
    let hash: SeededHash = make_random_hash_function(hash_len);

    let mut vec: Vec<u32> = vec![0; 2*input_len.pow(2)];

    let mut ph_data_structure = PerfectHashingDataStructure {
        vec, // Vec::with_capacity(input_len),
        hash_function: hash,
    };

    for x in input_array {
        ph_data_structure.insert(*x);
    }
    return ph_data_structure;
}

fn log2u(x: usize) -> u32 {
    x.ilog2() + 1
}

fn main() {
    const INPUT_SIZE: usize = 2_i32.pow(10) as usize - 1;
    let mut input: Vec<u32> = Vec::with_capacity(INPUT_SIZE);
    for i in 0..INPUT_SIZE {
        input.push(i as u32);
    }

    // let ij: u32 = 2666;
    // let hash = make_random_hash_function(20);
    // println!("{}", hash.hash(ij));
    // println!("{}", hash.hash(ij));

    let mut ph_struct: PerfectHashingDataStructure = perfect_hashing(&input);

    let mut sum: usize = 0;
    for i in 0..INPUT_SIZE {
        let s = ph_struct.query(i as u32) as usize;
        println!("{}", s);
        sum = sum+s;
    }
    println!("{}", sum);
}
