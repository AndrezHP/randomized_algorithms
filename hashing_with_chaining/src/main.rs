use std::collections::LinkedList;
use rand::prelude::*;
use rbtree::RBTree;
use std::cell::RefCell;

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

struct HwCDataStructure {
    vec: Vec<Option<RefCell<LinkedList<u32>>>>,
    hash_function: SeededHash,
}

impl HwCDataStructure {
    fn insert(&mut self, elem: u32) {
        let hash: usize = self.hash_function.hash(elem);
        self.vec[hash].as_ref().unwrap().borrow_mut().push_back(elem);
    }
    fn query(&mut self, elem: u32) -> bool {
        let hash: usize = self.hash_function.hash(elem);
        return self.vec[hash].as_ref().unwrap().borrow().contains(&elem);
    }
}

fn hashing_with_chaining(input: &Vec<u32>) {
    let mut hwc_struct: HwCDataStructure = make_hashing_with_chaining(&input);
    let mut sum: usize = 0;
    for x in input {
        if hwc_struct.query(*x) {
            sum += 1;
        }
    }
    println!("{}", sum);
}

fn make_hashing_with_chaining(input_array: &Vec<u32>) -> HwCDataStructure {
    let input_len: usize = 4*input_array.len();
    let hash_len: u32 = log2u(input_len);
    let hash: SeededHash = make_random_hash_function(hash_len);

    let mut vec = vec![Some(RefCell::new(LinkedList::new())); input_len];
    for i in 0..input_len {
        let list = Some(RefCell::new(LinkedList::new()));
        vec[i] = list;
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

fn main() {
    const INPUT_SIZE: usize = 2_i32.pow(22) as usize;
    let input: Vec<u32> = Vec::from_iter(0..INPUT_SIZE as u32);
    hashing_with_chaining(&input);
    // perfect_hashing(&input);
    rb_tree(&input);
}
