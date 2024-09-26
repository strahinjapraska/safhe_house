use num_bigint::BigInt;

use crate::math::finite_field::reduce;

pub fn fft(a: &Vec<BigInt> , n: usize, w: &BigInt, p: &BigInt) -> Vec<BigInt>{
    assert!(n.is_power_of_two(), "n must be power of 2");
    if n == 1 {
        return a.to_vec(); 
    }

    let a_0: Vec<BigInt> = a.iter()
    .step_by(2)
    .cloned()
    .collect();
    
    let a_1: Vec<BigInt> = a.iter()
        .skip(1)
        .step_by(2)
        .cloned()
        .collect();

    let w_squared = reduce(&(w*w),p);
    let a_0_cap = fft(&a_0, n/2, &w_squared, p); 
    let a_1_cap = fft(&a_1, n/2, &w_squared, p);

    let mut a_cap = vec![BigInt::from(0i32); n]; 

    for i in 0..n/2{
        a_cap[i] = reduce(&(&a_0_cap[i] + &reduce(&(&w.modpow(&BigInt::from(i), p) * &a_1_cap[i]), p)),p);
        a_cap[i+n/2] = reduce(&(&a_0_cap[i] + &reduce(&(&w.modpow(&BigInt::from(i+n/2), p) * &a_1_cap[i]), p)),p);
    }   
    a_cap 

}

pub fn ifft(a: &Vec<BigInt> , n: usize, w: &BigInt, p: &BigInt) -> Vec<BigInt>{
    let w_inv = w.modinv(p).expect("No inverse for w mod p");

    let mut fft_output = fft(a, n, &w_inv, p); 

    let n_inv = BigInt::from(n).modinv(p).expect("No inverse for n mod p");
    
    fft_output.iter_mut().for_each(|e|{
        *e = reduce(&(e.clone()* &n_inv),p);
    });

    fft_output
}