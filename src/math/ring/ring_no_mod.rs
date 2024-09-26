use crate::math::karatsuba::karatsuba;
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
    let mut res = karatsuba(a, b);

    for i in n..res.len() {          
        let sign = if (i/n) % 2 == 0 { BigInt::from(1) } else { BigInt::from(-1) };
        let temp = std::mem::take(&mut res[i]);
        res[i%n] += sign * temp;
    }
    res.truncate(n);
    res
}


pub fn add_no_mod(a: &Vec<BigInt>, b: &Vec<BigInt>) -> Vec<BigInt>{
    a
    .par_iter()
    .zip(b.par_iter())
    .map(|(a_val, b_val)|{ 
        a_val + b_val
    }).collect()
}
