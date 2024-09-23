use crate::math::ring::ring::QuotientRing;

use super::{ciphertext::Ciphertext, params::Params, plaintext::Plaintext};

pub struct SecretKey{
    pub(crate) secret: Vec<i128>,  
    params: Params, 
    ring: QuotientRing, 
}


impl SecretKey{
   
    pub(crate) fn new() -> SecretKey{
        let params = Params::new(); 
        let n = params.n; 
        let p = params.p; 
        let ring = QuotientRing{n, p}; 
      
        let secret = ring.binary_random_element();
        
        SecretKey{params , secret, ring}
    }

    pub fn decrypt(&self, ct: &Ciphertext) -> Plaintext{
        let ring = QuotientRing{n: self.params.n, p: self.params.t};
        let c0 = &ct.c0; 
        let c1 = &ct.c1; 

        
        Plaintext{message: 
            ring.scalar_div(self.params.p, &ring.scalar_mul_no_mod(self.params.t, &self.ring.add(c0, &self.ring.mul(c1, &self.secret))))
        }
    }
}


