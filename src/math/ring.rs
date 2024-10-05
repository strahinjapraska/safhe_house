use num::pow;
use rand::Rng;
use rayon::prelude::*;
use rug::{ops::NegAssign, rand::RandState, Complete, Integer};

use crate::math::{discrete_gaussian::sample_z, polymul::fft::fft_mul, finite_field::{modulo, reduce}, polymul::karatsuba::karatsuba, polymul::ntt::{intt, ntt}, util::random_binary_vector};

use super::polymul::school_book::schoolbook;


#[derive(PartialEq)]
pub enum PolyMulAlgorithm{
    Default, 
    Fft, 
    Karatsuba,  
    SchoolBook
}

pub fn mul(a: &Vec<Integer>, b: &Vec<Integer>, p: &Integer, w: &Integer, w_inv: &Integer, phi: &Integer, inv_phi: &Integer) -> Vec<Integer>{
    assert!(a.len() == b.len()); 

    let n = a.len();

    let mut a_cap = vec![Integer::from(0i32); a.len()];
    let mut b_cap = vec![Integer::from(0i32); b.len()];

    a_cap
    .par_iter_mut()
    .enumerate()
    .for_each(|(i, a_cap_i)| {
        let phi_i = phi.clone().pow_mod( &Integer::from(i), p).expect("Cannot modpow");
        *a_cap_i = reduce(&(&a[i] * phi_i), p);
    });

    b_cap
    .par_iter_mut()
    .enumerate()
    .for_each(|(i, b_cap_i)| {
        let phi_i = phi.clone().pow_mod( &Integer::from(i), p).expect("Cannot modpow");
        *b_cap_i = reduce(&(&b[i] * phi_i), p);
    });

    a_cap = ntt(&a_cap, n, &w, p);
    b_cap = ntt(&b_cap, n, &w, p);

    let mut c : Vec<Integer>= a_cap
    .par_iter()
    .zip(b_cap.par_iter())
    .map(|(a_val, b_val)|{ 
        reduce(&(a_val * b_val).complete(), p)
    }).collect(); 

    c = intt(&mut c, n, &w_inv, p);

    c
    .par_iter_mut()
    .enumerate()
    .for_each(|(i, c_val)|{
        let inv_phi_i = inv_phi.clone().pow_mod(&Integer::from(i), p).expect("Cannot modpow");
        *c_val *= inv_phi_i;
        *c_val = modulo(c_val, p);    
    });

    c


}
pub fn add(a: &Vec<Integer>, b: &Vec<Integer>, p: &Integer) -> Vec<Integer>{
    a
    .par_iter()
    .zip(b.par_iter())
    .map(|(a_val, b_val)|{ 
        modulo(&((a_val + b_val)).complete(), p)
    }).collect()
}

pub fn neg(a:  &Vec<Integer>, p: &Integer) -> Vec<Integer>{
    a
    .par_iter()
    .map(|a_val|{ 
        modulo(&(-a_val).complete(), p)
    }).collect()
} 

pub fn scalar_mul(s: &Integer, a: &Vec<Integer>, p: &Integer) -> Vec<Integer>{
    a
    .par_iter()
    .map(|a_val|{ 
        modulo(&(a_val*s).complete(), p)
    }).collect()
}



pub fn scalar_div(s: &Integer, a: &Vec<Integer>, p: &Integer) -> Vec<Integer>{
    a
    .par_iter()
    .map(|a_val|{ 
           if (*a_val < Integer::from(0)) == (s < &Integer::from(0)){
            modulo(&((a_val + Integer::from(s/2))/s),p)
           }else{
            modulo(&((a_val - Integer::from(s/2))/s),p)
           }
    }).collect()
}

pub fn scalar_mul_no_mod(s: &Integer, a: &Vec<Integer>) -> Vec<Integer>{
    a
    .par_iter()
    .map(|a_val|{ 
        (a_val*s).complete()
    }).collect()
}

pub fn scalar_div_no_mod(s: &Integer, a: &Vec<Integer>) -> Vec<Integer>{
    a
    .par_iter()
    .map(|a_val|{ 
        (a_val/s).complete()
    }).collect()
}


pub fn mul_no_mod(a: &Vec<Integer>, b:&Vec<Integer>, n: usize, algo: PolyMulAlgorithm, precision: usize) -> Vec<Integer>{
    let res = if algo == PolyMulAlgorithm::Karatsuba{ 
        karatsuba(a, b)
    }else if algo == PolyMulAlgorithm::Fft{
        fft_mul(a, b, precision)
    }else if algo == PolyMulAlgorithm::SchoolBook{
        schoolbook(a, b)
    }else{
        karatsuba(a, b)
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

pub fn neg_no_mod(a:  &Vec<Integer>) -> Vec<Integer>{
    a
    .par_iter()
    .map(|a_val|{ 
        (-a_val).complete()
    }).collect()
} 


pub fn sub_no_mod(a: &Vec<Integer>, b: &Vec<Integer>) -> Vec<Integer>{
    a
    .par_iter()
    .zip(b.par_iter())
    .map(|(a_val, b_val)|{ 
        (a_val - b_val).complete()
    }).collect()
}

pub fn point_wise_mul_no_mod(a: &Vec<Integer>, b: &Vec<Integer>) -> Vec<Integer>{
    a
    .par_iter()
    .zip(b.par_iter())
    .map(|(a_val, b_val)|{ 
        (a_val * b_val).complete()
    }).collect()
}



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

