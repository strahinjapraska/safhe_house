use crate::math::ring::{ring::{add, mul, neg, scalar_div, scalar_mul}, ring_no_mod::{add_no_mod, mul_no_mod, scalar_mul_no_mod}, ring_rand::{binary_random_element, discrete_gaussian_random_element, uniform_random_element}};

use super::{ciphertext::Ciphertext, params::Params, plaintext::Plaintext, secret_key::SecretKey};
use num_bigint::*; 


pub struct PublicKey{
    a: Vec<BigInt>, 
    b: Vec<BigInt>, 
    pub(crate) params: Params, 
    pub(crate) rlk: Vec<(Vec<BigInt>, Vec<BigInt>)>
}

impl PublicKey{
    pub(crate) fn new(s: &SecretKey)-> PublicKey{
        let params = s.params.clone();

        let a = uniform_random_element(&params.p, params.n);
        let e = discrete_gaussian_random_element(params.s, params.n);
        
        let b = neg(&add(&mul(&a, &s.secret, &params.p), &e, &params.p), &params.p); 

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
            &mul(p0,&u, &self.params.p),
            &scalar_mul(&delta, &pt.message, &self.params.p), 
            &self.params.p
            ), 
            &self.params.p
        );
        let c1 =  add(&e2, &mul(&p1, &u, &self.params.p), &self.params.p); 
        
        Ciphertext{c0, c1}
    }
    

    pub fn add(&self, lhs: &Ciphertext, rhs: &Ciphertext) -> Ciphertext {
        let (c0, c1) = rayon::join(
           || add(&lhs.c0, &rhs.c0, &self.params.p),
           || add(&lhs.c1, &rhs.c1, &self.params.p)
        ); 
        Ciphertext{c0, c1}
    }

    pub fn mul(&self, lhs: &Ciphertext, rhs: &Ciphertext) -> Ciphertext {      
        
        self.relin(
            &scale(&mul_no_mod(&lhs.c0, &rhs.c0, self.params.n), &self.params.p, &self.params.t), 
            &scale(&add_no_mod(&mul_no_mod(&lhs.c0, &rhs.c1, self.params.n), &mul_no_mod(&lhs.c1, &rhs.c0, self.params.n)), &self.params.p, &self.params.t), 
            &scale(&mul_no_mod(&lhs.c1, &rhs.c1, self.params.n), &self.params.p, &self.params.t), 
           
        )
    }
}

fn scale(c: &Vec<BigInt>, p: &BigInt, t: &BigInt) -> Vec<BigInt>{
    scalar_div(p, 
    &scalar_mul_no_mod(t, &c)
    ,p)
}