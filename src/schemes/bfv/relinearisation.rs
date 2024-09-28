use crate::math::ring::{ring::{add, mul, neg, scalar_mul}, ring_rand::{discrete_gaussian_random_element, uniform_random_element}};

use super::{ciphertext::Ciphertext, public_key::PublicKey, secret_key::SecretKey};
use rug::{ops::Pow, Complete, Integer};

use rayon::prelude::*; 

impl PublicKey{
    pub (crate) fn gen_rlk(secret_key: &SecretKey) -> Vec<(Vec<Integer>,Vec<Integer>)>{
        let p = &secret_key.params.p;
        let s = &secret_key.secret;
        let s_squared = mul(s, s, p); 
        let n = secret_key.params.n; 


        let rp = &secret_key.params.rp; 

        let mut rlk :Vec<(Vec<Integer>, Vec<Integer>)> = vec![]; 
      
        let l = secret_key.params.l;
    
        for i in 0..=l{
            let a_i = uniform_random_element(p, n);
            let e_i = discrete_gaussian_random_element(secret_key.params.s, n); 
         
            rlk.push(
              (
                add(
                    &neg(
                        &add(
                            &mul(&a_i,&s, p),
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
        let p = &self.params.p; 
        let rp = &self.params.rp; 
        let l = self.params.l; 
       
        let mut decomposition: Vec<Vec<Integer>> = vec![Vec::new(); l+1];
        
        let decomposed_ciphertext = c2.clone();    
    
        for i in 0..=l{
            for j in 0..decomposed_ciphertext.len(){
                decomposition[i].push((&decomposed_ciphertext[j]/&rp.clone().pow(i as u32)).complete()%rp);
            }
        }

        let mut s1 = mul(&self.rlk[0].0, &decomposition[0], p);
        let mut s2 = mul(&self.rlk[0].1, &decomposition[0], p);
        for i in 1..=l{
            s1 = add(&mul(&self.rlk[i].0, &decomposition[i], p), &s1, p); 
            s2 = add(&mul(&self.rlk[i].1, &decomposition[i], p), &s2, p); 
        }

        Ciphertext{c0: add(c0, &s1, p), c1: add(c1, &s2, p)}
    }
}