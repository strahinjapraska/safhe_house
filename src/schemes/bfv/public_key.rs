use crate::math::ring::{{add, mul, neg, scalar_mul},{binary_random_element, discrete_gaussian_random_element, uniform_random_element}};

use super::{ciphertext::Ciphertext, params::Params, plaintext::Plaintext, secret_key::SecretKey};
use rug::{Complete, Integer};


pub struct PublicKey{
    pub (crate) a: Vec<Integer>, 
    pub (crate) b: Vec<Integer>, 
    pub(crate) params: Params, 
    pub(crate) rlk: Vec<(Vec<Integer>, Vec<Integer>)>,
}

impl PublicKey{
    pub(crate) fn new(s: &SecretKey)-> PublicKey{
        let params = s.params.clone();

        let a = uniform_random_element(&params.p, params.n);
        let e = discrete_gaussian_random_element(params.s, params.n);
        
      
        let b = neg(&add(&mul(&a, &s.secret, &params.p, &params.w, &params.w_inv, &params.phi, &params.phi_inv), &e, &params.p), &params.p);
      
        let rlk  = PublicKey::gen_rlk(&s);

        PublicKey{a, b, params, rlk}
    }

    pub fn encrypt(&self, pt: &Plaintext) -> Ciphertext{
        let p0 = &self.b; 
        let p1 = &self.a; 

        let u = binary_random_element(self.params.n); 

        
        let e1 = discrete_gaussian_random_element(self.params.s, self.params.n); 
        let e2 = discrete_gaussian_random_element(self.params.s, self.params.n);  
        
        let delta = &self.params.p/&self.params.t;
        
        let c0 = add(
            &e1 
        ,&add(
            &mul(p0,&u, &self.params.p, &self.params.w, &self.params.w_inv, &self.params.phi, &self.params.phi_inv),
            &scalar_mul(&delta.complete(), &pt.message, &self.params.p), 
            &self.params.p
            ), 
            &self.params.p
        );
        let c1 =  add(&e2, &mul(&p1, &u, &self.params.p, &self.params.w, &self.params.w_inv, &self.params.phi, &self.params.phi_inv), &self.params.p);
        
        Ciphertext{c0, c1}
    }
}