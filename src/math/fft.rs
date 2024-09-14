use super::finite_field_ops::mod_pow;

pub fn fft(a: &Vec<u64> , n: usize, w: u64, p: u64) -> Vec<u64>{
    if n == 1 {
        return a.to_vec(); 
    }

    let a_0:Vec<u64> = a.iter()
           .enumerate()
           .filter(|&(i, _)| i%2 == 0)
           .map(|(_, v)| v.clone())
           .collect(); 

    let a_1: Vec<u64> = a.iter()
               .enumerate()
               .filter(|&(i, _)| i%2 != 0)
               .map(|(_, v)| v.clone())
               .collect(); 

    let w_squared = mod_pow(w, 2, p);
    let a_0_cap = fft(&a_0, n/2, w_squared, p); 
    let a_1_cap = fft(&a_1, n/2, w_squared, p);

    let mut a_cap = vec![u64::from(0u32); n]; 

    for i in 0..n/2-1{
        a_cap[i] = (a_0_cap[i].clone() + mod_pow(w, i as u64, p)*a_1_cap[i].clone())%p; 
        a_cap[i+n/2] = (a_0_cap[i+n/2].clone() + mod_pow(w,(i+n/2) as u64, p)*a_1_cap[i+n/2].clone())%p; 
    }   

    a_cap 

}