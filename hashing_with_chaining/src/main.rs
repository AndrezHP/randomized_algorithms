use std::collections::LinkedList;
use std::ops::{Deref, Index};
use rand::prelude::*;
use rbtree::RBTree;

pub fn random_generator(from: u32, to: u32) -> u32 {
    let mut rng = thread_rng();
    return rng.gen_range(from..to);
}

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

struct HwCDataStructure<> {
    vec: Vec<LinkedList<u32>>,
    hash_function: SeededHash,
}

impl HwCDataStructure<> {
    fn insert(&mut self, elem: u32) {
        let hash: usize = self.hash_function.hash(elem);

        if let Some(found_list) = self.vec.get_mut(hash) {
            (*found_list).push_front(elem);
            self.vec[hash] = *found_list;
        }
        // let found_list = match found_list_opt {
        //     Some(x) => x,
        //     None => LinkedList::new(),
        // };
        // found_list.push(elem);




    }

    fn query(&mut self, elem: u32) -> bool {
        let hash: usize = self.hash_function.hash(elem);

        return self.vec[hash].contains(&elem);
    }
}

fn hashing_with_chaining(input_array: &Vec<u32>) -> HwCDataStructure {
    let input_len: usize = input_array.len();
    let hash_len: u32 = log2u(2*input_len.pow(2)) - 1;
    let hash: SeededHash = make_random_hash_function(hash_len);

    let mut vec: Vec<LinkedList<u32>> = Vec::with_capacity(2*input_len.pow((2))); // vec![LinkedList; 2*input_len.pow(2)];
    for i in 0..input_len {
        let list: LinkedList<u32> = LinkedList::new();
        vec[i] = list
    }

    let mut hwc_struct: HwCDataStructure = HwCDataStructure {
        vec,
        hash_function: hash,
    };

    for x in input_array {
        hwc_struct.insert(*x);
    }

    return hwc_struct;
}

// struct HwCDataStructure {
//     vec: Vec<Vec<u32>>,
//     hash_function: SeededHash,
// }

// impl HwCDataStructure {
//     fn insert(&mut self, elem: u32) {
//         let hash: usize = self.hash_function(elem);
//         let mut found_array: Vec<u32> = self.vec.get(hash);
//     }
//     fn query(&mut self, elem: u32) -> bool {
//         return false;
//     }
// }

// TODO: implement this
// fn hashing_with_chaining() {
// this is nice
// }

struct PerfectHashingDataStructure {
    vec: Vec<u32>,
    hash_function: SeededHash,
}

impl PerfectHashingDataStructure {
    fn insert(&mut self, elem: u32) {
        let hash: usize = self.hash_function.hash(elem);
        self.vec[hash] += 1;
    }
    fn query(&mut self, elem: u32) -> bool {
        let hash: usize = self.hash_function.hash(elem);
        return self.vec[hash] != 0;
    }
}

fn perfect_hashing(input: &Vec<u32>) {
    let mut ph_struct: PerfectHashingDataStructure = make_perfect_hashing_structure(&input);

    let mut sum: usize = 0;
    for x in input {
        let s = ph_struct.query(*x) as usize;
        sum = sum+s;
    }
    println!("{}", sum);
}

fn make_perfect_hashing_structure(input: &Vec<u32>) -> PerfectHashingDataStructure {
    let input_len: usize = input.len();
    let hash_len = log2u(input_len.pow(2));
    let hash: SeededHash = make_random_hash_function(hash_len);

    let vec: Vec<u32> = vec![0; input_len.pow(2)];
    let mut ph_data_structure = PerfectHashingDataStructure {
        vec,
        hash_function: hash,
    };
    for x in input {
        ph_data_structure.insert(*x);
    }
    return ph_data_structure;
}

fn log2u(x: usize) -> u32 {
    x.ilog2()
}

fn rb_tree(input: &Vec<u32>) {
    let tree = make_rb_tree(input);

    let mut sum: usize = 0;
    for x in input {
        if tree.contains_key(x) {
            sum += 1;
        }
    }
    println!("{}", sum);
}

fn make_rb_tree(input: &Vec<u32>) -> RBTree<u32, u32> {
    let mut tree = RBTree::new();
    for x in input {
        tree.insert(*x, *x);
    }
    return tree;
}



fn test_hashing_with_chaining() {
    const INPUT_SIZE: usize = 2_i32.pow(16) as usize;
    let mut input: Vec<u32> = Vec::with_capacity(INPUT_SIZE);

    for i in 0..INPUT_SIZE {
        input.push(i as u32);
    }

    let mut hwc_struct: HwCDataStructure = hashing_with_chaining(&input);
}

fn main() {
    // test_perfect_hashing();
    test_hashing_with_chaining();
    perfect_hashing(&input);
    rb_tree(&input);
}
