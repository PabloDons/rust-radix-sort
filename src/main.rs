use rand::Rng;
use rayon::prelude::*;
use itertools::Itertools;

fn main() {
    let l_size = 16;
    const DIGIT_BIT_SIZE: u32 = 4;
    let digit_bit_size = 4;
    let cpus = num_cpus::get();
    let chunk_size = l_size/cpus;

    let mut rand_array: Vec<u32> = Vec::with_capacity(l_size);
    let mut rng = rand::thread_rng();
    for _ in 0..l_size {
        rand_array.push(rng.gen());
    }

    // let chunk_size = l_size/cpus;
    const NUM_DIGITS: usize = 2usize.pow(DIGIT_BIT_SIZE);
    let num_digits = 2u32.pow(DIGIT_BIT_SIZE);
    let num_length_digits = 32/digit_bit_size;

    let mut all_buckets: Vec<Vec<Vec<u32>>> = Vec::with_capacity(cpus);
    for d in 0..num_length_digits {
        rand_array.par_chunks(chunk_size).map(|slice| {
            let mut buckets: Vec<Vec<u32>> = Vec::with_capacity(NUM_DIGITS);
            for _ in 0..NUM_DIGITS {
                buckets.push(Vec::new());
            }

            for num in slice.iter() {
                let digit = ((*num) >> (digit_bit_size*d)) % num_digits;
                buckets[digit as usize].push(*num)
            }
            buckets
        }).collect_into_vec(&mut all_buckets);

        rand_array.clear();

        for i in 0..NUM_DIGITS {
            for chunk_buckets in all_buckets.iter() {
                rand_array.extend(chunk_buckets[i].iter());
            }
        }
    }

    for (num1, num2) in rand_array.iter().tuple_windows() {
        if num1 > num2 {
            panic!("incorrect sort");
        }
    }

    println!("{}", rand_array[0]);
}

