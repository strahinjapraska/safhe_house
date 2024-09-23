use rand::Rng;

use crate::math::{discrete_gaussian::sample_z, util::random_binary_vector};

use super::ring::QuotientRing;


impl QuotientRing{
    pub fn uniform_random_element(&self) -> Vec<i128> {
        // Uniformly sample a random element from Rq
        let mut rng = rand::thread_rng(); 
        let mut polynomial:Vec<i128> = Vec::with_capacity(self.n); 

        let half_p = self.p/2; 
        for _ in 0..self.n{
            polynomial.push(rng.gen_range(-half_p..=half_p)); 
        }

        polynomial

    }

    pub fn binary_random_element(&self) -> Vec<i128> {
        // Sample from R2 
        random_binary_vector(self.n)
    }

    pub fn discrete_gaussian_random_element(&self, sigma: f64) -> Vec<i128>{
        (0..self.n).map(|_| sample_z(sigma, self.n)).collect()
    }

}