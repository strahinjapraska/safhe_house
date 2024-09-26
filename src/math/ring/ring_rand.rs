use num_bigint::{BigInt, RandBigInt};
use rand::thread_rng;


use crate::math::{discrete_gaussian::sample_z, util::random_binary_vector};



pub fn uniform_random_element(p: &BigInt, n: usize) -> Vec<BigInt> {
    // Uniformly sample a random element from Rq
    let mut rng = thread_rng(); 
    let mut polynomial:Vec<BigInt> = Vec::with_capacity(n); 

    let half_p = p/2; 
    let lower_bound = -&half_p; 
    let upper_bound = &half_p + 1u32;
    for _ in 0..n{
        polynomial.push(rng.gen_bigint_range(&lower_bound, &upper_bound)); 
    }

    polynomial
}

pub fn binary_random_element(n: usize) -> Vec<BigInt> {
    // Sample from R2 
    random_binary_vector(n)
}

pub fn discrete_gaussian_random_element(sigma: f64, n: usize) -> Vec<BigInt>{
    (0..n).map(|_| BigInt::from(sample_z(sigma, n))).collect()
}
