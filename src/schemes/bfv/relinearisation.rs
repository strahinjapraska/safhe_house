use crate::math::ring::{{add, mul, neg, scalar_mul}, {discrete_gaussian_random_element, uniform_random_element}};

use super::{ciphertext::Ciphertext, public_key::PublicKey, secret_key::SecretKey};
use rug::{ops::Pow, Complete, Integer};

use rayon::prelude::*; 

impl PublicKey{
    pub (crate) fn gen_rlk(secret_key: &SecretKey) -> Vec<(Vec<Integer>,Vec<Integer>)>{
        let (p, w, w_inv, phi, phi_inv, n, rp, l) = (
            &secret_key.params.p,
            &secret_key.params.w,
            &secret_key.params.w_inv,
            &secret_key.params.phi,
            &secret_key.params.phi_inv,
            secret_key.params.n,
            &secret_key.params.rp,
            secret_key.params.l
        );

        let s = &secret_key.secret;
        let s_squared = mul(s, s, p, w, w_inv, phi, phi_inv);

        let mut rlk :Vec<(Vec<Integer>, Vec<Integer>)> = vec![]; 

    
        for i in 0..=l{
            let a_i = uniform_random_element(p, n);
            let e_i = discrete_gaussian_random_element(secret_key.params.s, n); 
         
            rlk.push(
              (
                add(
                    &neg(
                        &add(
                            &mul(&a_i,&s, p, w, w_inv, phi, phi_inv),
                        &e_i, p), 
                        p),
                    &scalar_mul(&rp.pow(i as u32).complete(), &s_squared, p), 
                p),     
                a_i
              )  
            );
        }
    
        rlk 
        
    }
    pub(crate) fn relin(&self, c0: &Vec<Integer>, c1: &Vec<Integer> , c2: &Vec<Integer>) -> Ciphertext{
        let (p, w, w_inv, phi, phi_inv,  rp, l) = (
            &self.params.p,
            &self.params.w,
            &self.params.w_inv,
            &self.params.phi,
            &self.params.phi_inv,
            &self.params.rp,
            self.params.l
        );
       
        let mut decomposition: Vec<Vec<Integer>> = vec![Vec::new(); l + 1];
        decomposition.par_iter_mut().enumerate().for_each(|(i, vec)| {
            for j in 0..c2.len() {
                vec.push((&c2[j] / &rp.clone().pow(i as u32)).complete() % rp);
            }
        });

        let s1 = (0..=l)
        .into_par_iter()  // Parallel iterator
        .map(|i| mul(&self.rlk[i].0, &decomposition[i], p, w, w_inv, phi, phi_inv))
        .reduce(|| vec![Integer::ZERO; self.params.n], |acc, val| add(&acc, &val, p)); 
    
        
        let s2 = (0..=l)
        .into_par_iter()  // Parallel iterator
        .map(|i| mul(&self.rlk[i].1, &decomposition[i], p, w, w_inv, phi, phi_inv))
        .reduce(|| vec![Integer::ZERO; self.params.n], |acc, val| add(&acc, &val, p)); 
        
          

        Ciphertext{c0: add(c0, &s1, p), c1: add(c1, &s2, p)} 
    }
}