use super::finite_field::{inv_mod, mod_pow, reduce};

pub fn fft(a: &Vec<i128> , n: usize, w: i128, p: i128) -> Vec<i128>{
    assert!(n.is_power_of_two(), "n must be power of 2");
    if n == 1 {
        return a.to_vec(); 
    }

    let a_0: Vec<i128> = a.iter()
    .step_by(2)
    .cloned()
    .collect();
    
    let a_1: Vec<i128> = a.iter()
        .skip(1)
        .step_by(2)
        .cloned()
        .collect();

    let w_squared = reduce(w*w,p);
    let a_0_cap = fft(&a_0, n/2, w_squared, p); 
    let a_1_cap = fft(&a_1, n/2, w_squared, p);

    let mut a_cap = vec![0i128; n]; 

    for i in 0..n/2{
        a_cap[i] = reduce(
            a_0_cap[i] +
             reduce(mod_pow(w, i as i128, p)*a_1_cap[i],p)
            ,p); 
        a_cap[i+n/2] = reduce(
            a_0_cap[i] + 
            reduce(mod_pow(w,(i+n/2) as i128, p)*a_1_cap[i],p),
            p); 
    }   

    a_cap 

}

pub fn ifft(a: &Vec<i128> , n: usize, w: i128, p: i128) -> Vec<i128>{
    let w_inv = inv_mod(w, p); 
   
    let mut fft_output = fft(a, n, w_inv, p); 

    let n_inv = inv_mod(n as i128, p); 
    
    fft_output.iter_mut().for_each(|e|{
        *e = reduce((*e)*n_inv, p);
    });

    fft_output
}