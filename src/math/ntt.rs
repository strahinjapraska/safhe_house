use rug::{Complete, Integer};

use crate::math::finite_field::reduce;

pub fn ntt(a: &Vec<Integer>, n: usize, w: &Integer, p: &Integer) -> Vec<Integer>{
    assert!(n.is_power_of_two(), "n must be power of 2");
    if n == 1 {
        return a.to_vec(); 
    }

    let a_0: Vec<Integer> = a.iter()
    .step_by(2)
    .cloned()
    .collect();
    
    let a_1: Vec<Integer> = a.iter()
        .skip(1)
        .step_by(2)
        .cloned()
        .collect();

    let w_squared = reduce(&(w*w).complete(),p);
    let a_0_cap = ntt(&a_0, n/2, &w_squared, p);
    let a_1_cap = ntt(&a_1, n/2, &w_squared, p);

    let mut a_cap = vec![Integer::from(0i32); n]; 

    for i in 0..n/2{
        a_cap[i] = reduce(&(&a_0_cap[i] + &reduce(&(&w.clone().pow_mod(&Integer::from(i), p).expect("Cannot modpow") * &a_1_cap[i]).complete(), p)).complete(),p);
        a_cap[i+n/2] = reduce(&(&a_0_cap[i] + &reduce(&(&w.clone().pow_mod(&Integer::from(i+n/2), p).expect("Cannot modpow") * &a_1_cap[i]).complete(), p)).complete(),p);
    }   
    a_cap 

}

pub fn intt(a: &Vec<Integer>, n: usize, w_inv: &Integer, p: &Integer) -> Vec<Integer>{
    let mut fft_output = ntt(a, n, &w_inv, p);

    let n_inv = Integer::from(n).invert(p).expect("No inverse for n mod p");
    
    fft_output.iter_mut().for_each(|e|{
        *e*= &n_inv; 
        *e = reduce(e,p);
    });

    fft_output
}