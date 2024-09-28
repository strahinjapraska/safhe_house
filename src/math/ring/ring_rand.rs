use rand::Rng;
use rug::{ops::NegAssign, rand::RandState, Integer};


use crate::math::{discrete_gaussian::sample_z, util::random_binary_vector};



pub fn uniform_random_element(p: &Integer, n: usize) -> Vec<Integer> {
    // Uniformly sample a random element from Rq
    let mut rng = RandState::new();
    let mut rng_bool = rand::thread_rng();
    let mut polynomial:Vec<Integer> = Vec::with_capacity(n); 

    let half_p: Integer = p.clone() / 2;
 
    for _ in 0..n{
        let mut r = Integer::from(Integer::random_below(half_p.clone(), &mut rng));
        if rng_bool.gen_bool(0.5){
            r.neg_assign();
        }
        polynomial.push(r);
    }

    polynomial
}

pub fn binary_random_element(n: usize) -> Vec<Integer> {
    // Sample from R2 
    random_binary_vector(n)
}

pub fn discrete_gaussian_random_element(sigma: f64, n: usize) -> Vec<Integer>{
    (0..n).map(|_| Integer::from(sample_z(sigma, n))).collect()
}
