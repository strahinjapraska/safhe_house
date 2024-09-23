use crate::math::karatsuba::karatsuba;

use super::ring::QuotientRing;
use rayon::prelude::*; 

impl QuotientRing{
    pub fn scalar_mul_no_mod(&self, s: i128, a: &Vec<i128>) -> Vec<i128>{
        a
        .par_iter()
        .map(|a_val|{ 
            (*a_val)*s
        }).collect()
    }

  
    pub fn mul_no_mod(&self, a: &Vec<i128>, b:&Vec<i128>) -> Vec<i128>{
        let mut res = karatsuba(a, b); 
        for i in 0..res.len(){
            if i >= self.n && res[i] != 0{
                let parity = if (i/self.n)%2 == 0{1}else{-1}; 
                res[i%self.n] += parity*res[i];
            }
        }
        res[0..self.n].to_vec() 
    }

    pub fn add_no_mod(&self, a: &Vec<i128>, b: &Vec<i128>) -> Vec<i128>{
        a
        .par_iter()
        .zip(b.par_iter())
        .map(|(a_val, b_val)|{ 
            (*a_val) + (*b_val)
        }).collect()
    }

}