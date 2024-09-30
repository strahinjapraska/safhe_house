use crate::math::{fft::fft_mul, karatsuba::karatsuba};
use num::pow;
use rayon::prelude::*; 
use rug::{Complete, Integer}; 


#[derive(PartialEq)]
pub enum PolyMulAlgorithm{
    Fft, 
    Karatsuba, 
    ToomCook
}

pub fn scalar_mul_no_mod(s: &Integer, a: &Vec<Integer>) -> Vec<Integer>{
    a
    .par_iter()
    .map(|a_val|{ 
        (a_val*s).complete()
    }).collect()
}


pub fn mul_no_mod(a: &Vec<Integer>, b:&Vec<Integer>, n: usize, algo: PolyMulAlgorithm, precision: usize) -> Vec<Integer>{
    let res = if algo == PolyMulAlgorithm::Karatsuba{ 
        karatsuba(a, b)
    }else{
        fft_mul(a, b, precision)
    }; 

    let mut reduced = res[..n].to_vec();

    for i in n..res.len() {          
            reduced[i%n] += Integer::from(pow(-1, i/n))*&res[i];
    }
    reduced
}


pub fn add_no_mod(a: &Vec<Integer>, b: &Vec<Integer>) -> Vec<Integer>{
    a
    .par_iter()
    .zip(b.par_iter())
    .map(|(a_val, b_val)|{ 
        (a_val + b_val).complete()
    }).collect()
}
