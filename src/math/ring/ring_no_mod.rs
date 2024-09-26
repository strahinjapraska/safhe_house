use crate::math::karatsuba::karatsuba;
use num::pow;
use rayon::prelude::*; 
use num_bigint::BigInt; 


pub fn scalar_mul_no_mod(s: &BigInt, a: &Vec<BigInt>) -> Vec<BigInt>{
    a
    .par_iter()
    .map(|a_val|{ 
        a_val*s
    }).collect()
}


pub fn mul_no_mod(a: &Vec<BigInt>, b:&Vec<BigInt>, n: usize) -> Vec<BigInt>{
    let res = karatsuba(a, b);
    let mut reduced = res[..n].to_vec();

    for i in n..res.len() {          
            reduced[i%n] += BigInt::from(pow(-1, i/n))*&res[i];
    }
    reduced
}


pub fn add_no_mod(a: &Vec<BigInt>, b: &Vec<BigInt>) -> Vec<BigInt>{
    a
    .par_iter()
    .zip(b.par_iter())
    .map(|(a_val, b_val)|{ 
        a_val + b_val
    }).collect()
}
