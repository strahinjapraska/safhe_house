use rug::{ Complete, Integer};
use rayon::prelude::*;

use crate::math::{fft::{fft, ifft}, finite_field::{modulo, primitive_nth_root_of_unity, reduce, square_root_mod_p}};


pub fn mul(a: &Vec<Integer>, b: &Vec<Integer>, p: &Integer) -> Vec<Integer>{
    assert!(a.len() == b.len()); 

    let n = a.len();
    let w = &primitive_nth_root_of_unity(p, n);


    let phi = square_root_mod_p(w, p); 
    
    let inv_phi = phi.clone().invert(p).expect("No inverse for phi mod p");

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

    a_cap = fft(&a_cap, n, &w, p); 
    b_cap = fft(&b_cap, n, &w, p);

    let mut c : Vec<Integer>= a_cap
    .par_iter()
    .zip(b_cap.par_iter())
    .map(|(a_val, b_val)|{ 
        reduce(&(a_val * b_val).complete(), p)
    }).collect(); 

    c = ifft(&mut c, n, &w, p);

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
           modulo(&((a_val + Integer::from(s/2))/s),p)
    }).collect()
}
