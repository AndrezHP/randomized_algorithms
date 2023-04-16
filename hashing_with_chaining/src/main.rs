use rand::prelude::*;
use time::OffsetDateTime;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::ops::Shr;

pub fn random_generator(from: u64, to: u64) -> u64 {
    let mut rng = thread_rng();
    return rng.gen_range(from..to);
}

// The hash function is c-universal
const C: usize = 2;

struct SeededHash {
    l: u32,
    a: u64,
    b: u64,
}

impl SeededHash {
    fn new(hash_len: u32) -> SeededHash {
        let randomness_size: u64 = 2u64.pow(63);
        let mut rand_a: u64 = random_generator(1, randomness_size);
        if rand_a % 2 == 0 {
            rand_a += 1
        }
        let rand_b: u64 = random_generator(0, randomness_size);
        return SeededHash {
            a: rand_a,
            b: rand_b,
            l: hash_len
        }
    }
    // Multiply shift hashing as from lecture notes (https://arxiv.org/pdf/1504.06804.pdf) at 3.3
    fn hash(&self, x: u64) -> usize {
        let multiply_add: u64 = self.a.wrapping_mul(x).wrapping_add(self.b);
        return multiply_add.shr(64 - self.l) as usize;
    }
}

struct HwC {
    vec: Vec<Vec<u64>>,
    hash_function: SeededHash
}

impl HwC {
    fn new(size: usize) -> HwC {
        let input_len: usize = size;
        let hash_len: u32 = log2u(input_len);
        let vec = vec![Vec::<u64>::new(); input_len];
        let hash_fn: SeededHash = SeededHash::new(hash_len);
        return HwC {
            vec,
            hash_function: hash_fn
        }
    }
    fn insert(&mut self, key: u64, value: u64) {
        let hash_val: usize = self.hash_function.hash(key);
        for i in (0..self.vec[hash_val].len()).step_by(2) {
            if self.vec[hash_val][i] == key {
                self.vec[hash_val][i+1] += value;
                return;
            }
        }
        self.vec[hash_val].push(key);
        self.vec[hash_val].push(value)
    }
    fn get_norm(&self) -> u64 {
        let mut sum: u64 = 0;
        for vec in &self.vec {
            for i in (0..vec.len()).step_by(2) {
                sum += vec[i+1].pow(2);
            }
        }
        return sum;
    }
}

fn log2u(x: usize) -> u32 {
    x.ilog2()
}

// 4-wise independent hash function
struct IndependentHash {
    a: u64,
    b: u64,
    c: u64,
    d: u64,
    l: u32,
}

impl IndependentHash {
    fn new(hash_len: u32) -> IndependentHash {
        let randomness_size: u64 = 2u64.pow(31);
        return IndependentHash {
            a: random_generator(1, randomness_size),
            b: random_generator(1, randomness_size),
            c: random_generator(1, randomness_size),
            d: random_generator(1, randomness_size),
            l: hash_len
        }
    }
    fn hash(&self, x: u64) -> (u64, i64) {
        let prime = 2u64.pow(31) - 1;
        let mut k: u64 = (self.a * x + self.b) % prime;
        k = (k * x + self.c) % prime;
        k = (k * x + self.d) % prime;

        let h = k.shr(1) & (2u64.pow(self.l) - 1);
        // -> {-1, 1}
        let g = 2*((k as i64) & 1) - 1;
        return (h, g)
    }
}

struct NormSketch {
    vec: Vec<i64>,
    hash_function: IndependentHash,
}

impl NormSketch {
    fn new(r: usize) -> NormSketch {
        let hash = IndependentHash::new(log2u(r));
        return NormSketch {
            vec: vec![0; r],
            hash_function: hash,
        }
    }
    fn update(&mut self, key: u64, value: i64) {
        let (h, g) = self.hash_function.hash(key);
        self.vec[h as usize] += g*value;
    }
    fn query(&self) -> i64 {
        let mut sum = 0;
        for x in &self.vec {
            sum += x.pow(2);
        }
        return sum
    }
}

fn make_writable_file(file_name: &str) -> File {
    return OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_name.to_owned() + ".txt")
        .unwrap();
}

fn hwc_test() {
    let mut hwc: HwC = HwC::new(100000);
    let vec: Vec<u64> = make_updates_of_1(100000, 100000);
    for i in (0..vec.len()).step_by(2) {
        hwc.insert(vec[i], vec[i+1]);
    }
    println!("Norm: {}", hwc.get_norm());
}

fn make_updates_of_1(input_size: u64, key_size: u64) -> Vec<u64> {
    let mut vec: Vec<u64> = Vec::new();
    for i in 0..input_size {
        vec.push(i % key_size);
        vec.push(1);
    }
    return vec
}

fn test_hash() {
    let input_size = 10usize.pow(6);
    let input: Vec<u64> = Vec::from_iter(0..input_size as u64);

    let hash = SeededHash::new(log2u(input_size));
    let independent_hash = IndependentHash::new(log2u(input_size));

    let c_start = OffsetDateTime::now_utc();
    for x in &input {
        hash.hash(*x);
    }
    let c_stop = OffsetDateTime::now_utc();
    println!("Old hash: {}", c_stop - c_start);

    let i_start = OffsetDateTime::now_utc();
    for x in &input {
        independent_hash.hash(*x);
    }
    let i_stop = OffsetDateTime::now_utc();
    println!("4-wise independent hash: {}", i_stop - i_start)
}

fn exercise7hwc() {
    let number_of_updates = 10u64.pow(9);
    let key_sizes: Vec<u64> = Vec::from_iter((6..28+1).step_by(2));
    for k in key_sizes {
        let key_size = 2u64.pow(k as u32);
        let mut hwc = HwC::new(key_size as usize);

        let i_start = OffsetDateTime::now_utc();
        for i in (0..number_of_updates).step_by(2) {
            hwc.insert(i % key_size, 1);
        }
        let i_stop = OffsetDateTime::now_utc();
        println!("HwC update with n = 2^{}: {}", k, i_stop - i_start);


        let q_start = OffsetDateTime::now_utc();
        hwc.get_norm();
        let q_stop = OffsetDateTime::now_utc();
        println!("HwC query with n = 2^{}: {}", k, q_stop - q_start);
    }
}

fn exercise7norm_sketch() {
    let number_of_updates = 10u64.pow(9);
    let key_sizes: Vec<u64> = Vec::from_iter((6..28+1).step_by(2));
    for k in key_sizes {
        let key_size = 2u64.pow(k as u32);
        let mut norm_sketch = NormSketch::new(2usize.pow(7));

        let i_start = OffsetDateTime::now_utc();
        for i in (0..number_of_updates).step_by(2) {
            norm_sketch.update(i % key_size, 1);
        }
        let i_stop = OffsetDateTime::now_utc();
        println!("NormSketch update with n = 2^{}: {}", k, i_stop - i_start);

        let q_start = OffsetDateTime::now_utc();
        norm_sketch.query();
        let q_stop = OffsetDateTime::now_utc();
        println!("NormSketch query with n = 2^{}: {}", k, q_stop - q_start)
    }
}

fn main() -> std::io::Result<()> {
    // test_hash();
    // hwc_test();
    exercise7hwc();
    exercise7norm_sketch();
    Ok(())
}
