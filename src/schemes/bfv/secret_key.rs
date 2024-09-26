use crate::math::ring::{ring::{add, mul, scalar_div}, ring_no_mod::scalar_mul_no_mod, ring_rand::binary_random_element};

use super::{ciphertext::Ciphertext, params::Params, plaintext::Plaintext};
use num_bigint::BigInt;
pub struct SecretKey{
    pub(crate) secret: Vec<BigInt>,  
    pub (crate) params: Params,  
}


impl SecretKey{
   
    pub(crate) fn new(params: &Params) -> SecretKey{
        let secret = binary_random_element(params.n);
        
        SecretKey{params: params.clone() , secret}
    }

    pub fn decrypt(&self, ct: &Ciphertext) -> Plaintext{
        let c0 = &ct.c0; 
        let c1 = &ct.c1; 

        Plaintext{message: 
            scalar_div(&self.params.p, 
                &scalar_mul_no_mod(&self.params.t, 
                    &add(c0, 
                        &mul(c1, &self.secret, &self.params.p), 
                        &self.params.p)),
                         &self.params.t)
        }
    }

}


