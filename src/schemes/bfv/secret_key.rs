use crate::math::ring::{{add, mul, scalar_div}, scalar_mul_no_mod, binary_random_element};

use super::{ciphertext::Ciphertext, params::Params, plaintext::Plaintext};
use rug::Integer;
pub struct SecretKey{
    pub(crate) secret: Vec<Integer>,  
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
                        &mul(c1, &self.secret, &self.params.p, &self.params.w, &self.params.w_inv, &self.params.phi, &self.params.phi_inv),
                        &self.params.p)),
                         &self.params.t)
        }
    }

}


