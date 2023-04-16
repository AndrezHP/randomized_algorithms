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

struct KeyValuePair {
    key: u64,
    value: u64,
}

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
        return multiply_add.wrapping_shr(64 - self.l) as usize;
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
        let base: u64 = 2;
        let randomness_size: u64 = base.pow(31);
        return IndependentHash {
            a: random_generator(1, randomness_size),
            b: random_generator(1, randomness_size),
            c: random_generator(1, randomness_size),
            d: random_generator(1, randomness_size),
            l: hash_len
        }
    }
    fn hash(&self, x: u64) -> u64 {
        let mut k: u64 = (self.a * x + self.b).wrapping_shr(64 - self.l);
        k = (k * x + self.c).wrapping_shr(64 - self.l);
        k = (k * x + self.d).wrapping_shr(64 - self.l);
        return k
    }
    // g: [n] -> {l}
    fn h(&self, x: u64) -> u64 {
        let k = self.hash(x);
        return k.wrapping_shr(1) & (2u64.pow(self.l) - 1)
    }
    // g: [n] -> {-1, 1}
    fn g(&self, x: u64) -> i64 {
        let k: i64 = self.hash(x) as i64;
        return 2*(k & 1) - 1;
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
    let vec: Vec<u64> = make_random_inputs(100000);
    for i in (0..vec.len()).step_by(2) {
        hwc.insert(vec[i], vec[i+1]);
    }
    println!("Norm: {}", hwc.get_norm());
}

fn make_random_inputs(input_size: usize) -> Vec<u64> {
    let mut vec: Vec<u64> = Vec::new();
    for _ in 0..input_size {
        vec.push(random_generator(1, 2u64.pow(20)));
        vec.push(random_generator(1,2u64.pow(20)));
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
        independent_hash.h(*x);
        independent_hash.g(*x);
    }
    let i_stop = OffsetDateTime::now_utc();
    println!("4-wise independent hash: {}", i_stop - i_start)
}

fn main() -> std::io::Result<()> {
    const TEST_SIZES: [i32; 7] = [12, 14, 16, 18, 20, 22, 24];
    // benchmark_hwc(TEST_SIZES);
    test_hash();
    hwc_test();
    Ok(())
}
