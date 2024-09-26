use crate::math::ring::{ring::{add, scalar_div}, ring_no_mod::{add_no_mod, mul_no_mod, scalar_mul_no_mod}};

use super::{ciphertext::Ciphertext, public_key::PublicKey};
use num_bigint::BigInt;

impl PublicKey{
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