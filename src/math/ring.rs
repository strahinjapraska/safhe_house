use rand::Rng;
use rayon::prelude::*;

use crate::math::{fft::{fft, ifft}, finite_field::{inv_mod, mod_pow, reduce,modulo}};

use super::{discrete_gaussian::sample_z, finite_field::{primitive_nth_root_of_unity, square_root_mod_p}, util::random_binary_vector};

fn _precompute_values(){
    panic!("Not implemented")
}


#[derive(Clone)]
pub struct QuotientRing{
    pub n: usize, 
    pub p: i64, 
}

impl QuotientRing{
    pub fn mul(&self, a: &Vec<i64>, b: &Vec<i64>) -> Vec<i64>{
        assert!(a.len() == b.len()); 
    
        let n = a.len();

        let w = primitive_nth_root_of_unity(self.p, n);
        
        let phi = square_root_mod_p(w, self.p); 
        let inv_phi = inv_mod(phi, self.p); 

        let mut a_cap = vec![0; a.len()];
        let mut b_cap = vec![0; b.len()];

        a_cap
        .par_iter_mut()
        .enumerate()
        .for_each(|(i, a_cap_i)| {
            let phi_i = mod_pow(phi, i as i64, self.p);
            *a_cap_i = reduce(a[i] * phi_i, self.p);
        });

        b_cap
        .par_iter_mut()
        .enumerate()
        .for_each(|(i, b_cap_i)| {
            let phi_i = mod_pow(phi, i as i64, self.p);
            *b_cap_i = reduce(b[i] * phi_i, self.p);
        });

        a_cap = fft(&a_cap, n, w, self.p); 
        b_cap = fft(&b_cap, n, w, self.p);

        let mut c : Vec<i64>= a_cap
        .par_iter()
        .zip(b_cap.par_iter())
        .map(|(a_val, b_val)|{ 
            reduce((*a_val) * (*b_val), self.p)
        }).collect(); 

        c = ifft(&mut c, n, w, self.p);

        c
        .par_iter_mut()
        .enumerate()
        .for_each(|(i, c_val)|{
            *c_val = modulo((*c_val)*mod_pow(inv_phi, i as i64, self.p),self.p);
        });

        c


    }
    pub fn add(&self, a: &Vec<i64>, b: &Vec<i64>) -> Vec<i64>{
        a
        .par_iter()
        .zip(b.par_iter())
        .map(|(a_val, b_val)|{ 
            modulo((*a_val) + (*b_val), self.p)
        }).collect()
    }

    pub fn neg(&self, a:  &Vec<i64>) -> Vec<i64>{
        a
        .par_iter()
        .map(|a_val|{ 
            modulo(-(*a_val), self.p)
        }).collect()
    } 

    pub fn uniform_random_element(&self) -> Vec<i64> {
        // Uniformly sample a random element from Rq
        let mut rng = rand::thread_rng(); 
        let mut polynomial:Vec<i64> = Vec::with_capacity(self.n); 

        let half_p = self.p/2; 
        for _ in 0..self.n{
            polynomial.push(rng.gen_range(-half_p..=half_p)); 
        }

        polynomial

    }

    pub fn binary_random_element(&self) -> Vec<i64> {
        // Sample from R2 
        random_binary_vector(self.n)
    }

    pub fn discrete_gaussian_random_element(&self, sigma: f64) -> Vec<i64>{
        (0..self.n).map(|_| sample_z(sigma, self.n)).collect()
    }

    pub fn scalar_mul(&self, s: i64, a: &Vec<i64>) -> Vec<i64>{
        a
        .par_iter()
        .map(|a_val|{ 
            modulo((*a_val)*s, self.p)
        }).collect()
    }

    pub fn scalar_mul_no_mod(&self, s: i64, a: &Vec<i64>) -> Vec<i64>{
        a
        .par_iter()
        .map(|a_val|{ 
            (*a_val)*s
        }).collect()
    }
    

    pub fn scalar_div(&self, s: i64, a: &Vec<i64>) -> Vec<i64>{
        a
        .par_iter()
        .map(|a_val|{ 
            modulo(
                ((
                    ((*a_val) as f64)/
                    (s as f64)
                    ).round()) as i64,
                self.p)
        }).collect()
    }

}
