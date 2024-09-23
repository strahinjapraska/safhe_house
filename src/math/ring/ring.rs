use rayon::prelude::*;

use crate::math::{fft::{fft, ifft}, finite_field::{inv_mod, mod_pow, modulo, primitive_nth_root_of_unity, reduce, square_root_mod_p}};


fn _precompute_values(){
    panic!("Not implemented")
}


#[derive(Clone)]
pub struct QuotientRing{
    pub n: usize, 
    pub p: i128, 
}

impl QuotientRing{
    pub fn mul(&self, a: &Vec<i128>, b: &Vec<i128>) -> Vec<i128>{
        assert!(a.len() == b.len()); 
    
        let n = a.len();

        let w = primitive_nth_root_of_unity(self.p, n);
        
        let phi = square_root_mod_p(w, self.p); 
        let inv_phi = inv_mod(phi, self.p); 

        let mut a_cap = vec![0; a.len()];
        let mut b_cap = vec![0; b.len()];

        a_cap
        .par_iter_mut()
        .enumerate()
        .for_each(|(i, a_cap_i)| {
            let phi_i = mod_pow(phi, i as i128, self.p);
            *a_cap_i = reduce(a[i] * phi_i, self.p);
        });

        b_cap
        .par_iter_mut()
        .enumerate()
        .for_each(|(i, b_cap_i)| {
            let phi_i = mod_pow(phi, i as i128, self.p);
            *b_cap_i = reduce(b[i] * phi_i, self.p);
        });

        a_cap = fft(&a_cap, n, w, self.p); 
        b_cap = fft(&b_cap, n, w, self.p);

        let mut c : Vec<i128>= a_cap
        .par_iter()
        .zip(b_cap.par_iter())
        .map(|(a_val, b_val)|{ 
            reduce((*a_val) * (*b_val), self.p)
        }).collect(); 

        c = ifft(&mut c, n, w, self.p);

        c
        .par_iter_mut()
        .enumerate()
        .for_each(|(i, c_val)|{
            *c_val = modulo((*c_val)*mod_pow(inv_phi, i as i128, self.p),self.p);
        });

        c


    }
    pub fn add(&self, a: &Vec<i128>, b: &Vec<i128>) -> Vec<i128>{
        a
        .par_iter()
        .zip(b.par_iter())
        .map(|(a_val, b_val)|{ 
            modulo((*a_val) + (*b_val), self.p)
        }).collect()
    }

    pub fn neg(&self, a:  &Vec<i128>) -> Vec<i128>{
        a
        .par_iter()
        .map(|a_val|{ 
            modulo(-(*a_val), self.p)
        }).collect()
    } 

   
    pub fn scalar_mul(&self, s: i128, a: &Vec<i128>) -> Vec<i128>{
        a
        .par_iter()
        .map(|a_val|{ 
            modulo((*a_val)*s, self.p)
        }).collect()
    }

   
    
    pub fn scalar_div(&self, s: i128, a: &Vec<i128>) -> Vec<i128>{
        a
        .par_iter()
        .map(|a_val|{ 
            
                if ((*a_val) < 0) == (s<0){
                    ((*a_val) + s/2)/s
                }else{
                    ((*a_val) - s/2)/s
                  }
        }).collect()
    }

   
}
