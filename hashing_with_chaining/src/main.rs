use std::collections::LinkedList;
use rand::prelude::*;
use rbtree::RBTree;

pub fn random_generator(from: u32, to: u32) -> u32 {
    let mut rng = thread_rng();
    return rng.gen_range(from..to);
}

// The hash function is c-universal
const C: usize = 2;

struct SeededHash {
    l: u32,
    a: u32,
    b: u32,
}

impl SeededHash {
    fn new(hash_len: u32) -> SeededHash {
        let base: u32 = 2;
        let randomness_size: u32 = base.pow(31);
        let rand_a: u32 = random_generator(1, randomness_size);
        let rand_b: u32 = random_generator(1, randomness_size);
        return SeededHash {
            a: rand_a,
            b: rand_b,
            l: hash_len
        }
    }
    // Multiply shift hashing as from lecture notes (https://arxiv.org/pdf/1504.06804.pdf) at 3.3
    fn hash(&self, x: u32) -> usize {
        let multiply_add: u32 = self.a.wrapping_mul(x).wrapping_add(self.b);
        return multiply_add.wrapping_shr(32 - self.l) as usize;
    }
}

struct HwC {
    vec: Vec<LinkedList<u32>>,
    hash_function: SeededHash
}

impl<'a> HwC {
    fn new(size: usize) -> HwC {
        let input_len: usize = 4*C*size;
        let hash_len: u32 = log2u(input_len);
        let vec = vec![LinkedList::<u32>::new(); input_len];
        let hash_fn: SeededHash = SeededHash::new(hash_len);
        return HwC {
            vec,
            hash_function: hash_fn
        }
    }
    fn insert(&mut self, elem: u32) {
        let hash_val: usize = self.hash_function.hash(elem);
        self.vec[hash_val].push_back(elem)
    }
    fn query(&self, elem: u32) -> bool {
        let hash_val: usize = self.hash_function.hash(elem);
        self.vec[hash_val].contains(&elem)
    }
}

fn hashing_with_chaining(input: &Vec<u32>) {
    let mut hwc: HwC = HwC::new(input.len());
    for x in input {
        hwc.insert(*x);
    }
    let mut sum: usize = 0;
    for x in input {
        if hwc.query(*x) {
            sum += 1;
        }
    }
    println!("{}", sum);
}

struct Bucket {
    vec: Vec<u32>,
    hash_function: SeededHash,
}

impl Bucket {
    fn new(input_array: &Vec<u32>) -> Bucket {
        let array_len: usize = 2*C*input_array.len().pow(2);
        if array_len == 0 {
            return Bucket {
                vec: vec![0; 0],
                hash_function: SeededHash::new(0)
            }
        }
        let hash_len: u32 = log2u(array_len);
        let arr = vec![0; array_len];
        let mut bucket: Bucket = Bucket {
            vec: arr,
            hash_function: SeededHash::new(hash_len),
        };
        for x in input_array {
            let success: bool = bucket.insert(*x);
            if !success {
                return Bucket::new(input_array)
            }
        }
        return bucket;
    }
    fn insert(&mut self, elem: u32) -> bool {
        if self.vec[self.hash_function.hash(elem)] != 0 {
            return false;
        } else {
            self.vec[self.hash_function.hash(elem)] = elem;
            return true;
        }
    }
    fn query(&self, elem: u32) -> bool {
        return self.vec[self.hash_function.hash(elem)] == elem
    }
}

struct PerfectHashing {
    vec: Vec<Bucket>,
    hash_function: SeededHash,
}

impl PerfectHashing {
    fn new(input_array: &Vec<u32>) -> PerfectHashing {
        let array_len: usize = 4*C*input_array.len();

        let hash_len: u32 = log2u(array_len);
        let hash_fn: SeededHash = SeededHash::new(hash_len);

        let mut buckets: Vec<Vec<u32>> = Vec::new();
        for _ in 0..array_len {
            buckets.push(Vec::new());
        }

        for x in input_array {
            let hash = hash_fn.hash(*x);
            buckets[hash].push(*x);
        }

        let mut sum_of_squares = 0;
        for x in &mut buckets {
            sum_of_squares += x.len().pow(2);
        }
        if sum_of_squares > array_len {
            println!("Fail");
            return PerfectHashing::new(input_array);
        }

        let mut vec = Vec::<Bucket>::with_capacity(array_len);
        for vec_bucket in &mut buckets {
            if vec_bucket.len() == 0 {
                vec.push(Bucket::new(&Vec::new()));
                continue
            }
            let bucket = Bucket::new(vec_bucket);
            vec.push(bucket);
        }
        return PerfectHashing {
            vec,
            hash_function: hash_fn
        }
    }
    // fn insert(&mut self, elem: u32) {
    //     let hash: usize = self.hash_function.hash(elem);
    //     self.vec[hash].insert(elem);
    // }
    fn query(&self, elem: u32) -> bool {
        let hash: usize = self.hash_function.hash(elem);
        return self.vec[hash].query(elem);
    }
}

fn perfect_hashing(input: &Vec<u32>) {
    let ph_struct: PerfectHashing = PerfectHashing::new(&input);

    let mut sum: usize = 0;
    for x in input {
        let s = ph_struct.query(*x) as usize;
        sum = sum+s;
    }
    println!("{}", sum);
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
    const BIT_SIZE: u32 = 10;
    const INPUT_SIZE: usize = 2_i32.pow(BIT_SIZE) as usize;
    let input: Vec<u32> = Vec::from_iter(1..(INPUT_SIZE+1) as u32);
    hashing_with_chaining(&input);
    perfect_hashing(&input);
    rb_tree(&input);
}
