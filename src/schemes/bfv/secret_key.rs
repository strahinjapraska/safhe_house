use crate::math::{discrete_gaussian::sample_discrete_gaussian_ring_element, ring::QuotientRing};

use super::{ciphertext::Ciphertext, params::Params, plaintext::Plaintext};

pub struct SecretKey{
    pub(crate) secret: Vec<i64>,  
    params: Params, 
    ring: QuotientRing, 
}


impl SecretKey{
   
    pub(crate) fn new() -> SecretKey{
        let params = Params::new(); 
        let n = params.n; 
        let p = params.p; 
        let secret = sample_discrete_gaussian_ring_element(n, params.s); 
        
        SecretKey{params , secret, ring: QuotientRing{n, p}}
    }

    pub fn decrypt(&self, ct: &Ciphertext) -> Plaintext{
        let ring = QuotientRing{n: self.params.n, p: self.params.t};
        let c0 = &ct.c0; 
        let c1 = &ct.c1; 

        let delta = (self.params.t as f64/self.params.p as f64).round() as i64;

        Plaintext{message: ring.scalar_div(delta, &self.ring.add(c0, &self.ring.mul(c1, &self.secret)))}
    }
}


