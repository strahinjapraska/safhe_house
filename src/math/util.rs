use num_bigint::BigInt; 
use rand::Rng;
use rug::Integer;

use super::ring::{ring::scalar_div, ring_no_mod::scalar_mul_no_mod};

pub fn random_binary_vector(n: usize) -> Vec<Integer>{

    let mut rng = rand::thread_rng(); 

    (0..n)
    .map(|_| {
            let num = rng.gen_range(0..=1);
            Integer::from(num)
        })
    .collect()

}

pub fn scale(c: &Vec<BigInt>, p: &BigInt, t: &BigInt) -> Vec<BigInt>{
    scalar_div(p, 
    &scalar_mul_no_mod(t, &c)
    ,p)
}
