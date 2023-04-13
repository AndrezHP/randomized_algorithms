use rand::prelude::*;
use time::OffsetDateTime;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;

pub fn random_generator(from: u32, to: u32) -> u32 {
    let mut rng = thread_rng();
    return rng.gen_range(from..to);
}

// The hash function is c-universal
const C: usize = 2;

struct KeyValuePair {
    key: u32,
    value: u32,
}

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
    vec: Vec<Vec<u32>>,
    hash_function: SeededHash
}

impl HwC {
    fn new(size: usize) -> HwC {
        let input_len: usize = size;
        let hash_len: u32 = log2u(input_len);
        let vec = vec![Vec::<u32>::new(); input_len];
        let hash_fn: SeededHash = SeededHash::new(hash_len);
        return HwC {
            vec,
            hash_function: hash_fn
        }
    }
    fn insert(&mut self, key: u32, value: u32) {
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
    fn get_norm(&self) -> u32 {
        let mut sum: u32 = 0;
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

// fn hashing_with_chaining(input: &Vec<KeyValuePair>, file: &mut File) {
//     let c_start = OffsetDateTime::now_utc();
//     let mut hwc: HwC = HwC::new(input.len());
//     for x in input {
//         hwc.insert(x.key, x.value);
//     }
//     let c_stop = OffsetDateTime::now_utc();
//     writeln!(file, "Construction time: {}", c_stop - c_start).expect("Cannot write to file");
//
//
//     // let mut sum: usize = 0;
//     let q_start = OffsetDateTime::now_utc();
//     let norm: u32 = hwc.get_norm();
//     // println!("{}", sum);
//     let q_stop = OffsetDateTime::now_utc();
//     writeln!(file, "Query time: {}", q_stop - q_start).expect("Cannot write to file");
//
//     let mut longest_ll: usize = 0;
//     for ll in hwc.vec {
//         if ll.len() > longest_ll {
//             longest_ll = ll.len();
//         }
//     }
//     writeln!(file, "Longest linked list: {}", longest_ll).expect("Cannot write to file");
// }

fn make_writable_file(file_name: &str) -> File {
    return OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_name.to_owned() + ".txt")
        .unwrap();
}

// fn benchmark_hwc(test_sizes: [i32; 7]) {
//     let mut file: File = make_writable_file("hwc");
//
//     for test_size in test_sizes {
//         writeln!(file, "Test size: {}", test_size).expect("Cannot write to file");
//         // let input_size: usize = 2_i32.pow(test_size as u32) as usize;
//         // let input: Vec<u32> = Vec::from_iter(1..(input_size +1) as u32);
//         // hashing_with_chaining(&input, &mut file);
//     }
// }

fn hwc_test() {
    let mut hwc: HwC = HwC::new(1000);
    let vec: Vec<u32> = make_random_inputs(1000);
    for i in (0..vec.len()).step_by(2) {
        hwc.insert(vec[i], vec[i+1]);
    }
    println!("Norm: {}", hwc.get_norm());
}

fn make_random_inputs(input_size: usize) -> Vec<u32> {
    let mut vec: Vec<u32> = Vec::new();
    for _ in 0..input_size {
        vec.push(random_generator(1, 1000));
        vec.push(random_generator(1,1000));
    }
    return vec
}


fn main() -> std::io::Result<()> {
    const TEST_SIZES: [i32; 7] = [12, 14, 16, 18, 20, 22, 24];
    // benchmark_hwc(TEST_SIZES);
    hwc_test();
    Ok(())
}
