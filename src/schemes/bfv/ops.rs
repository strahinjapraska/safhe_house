use rayon::prelude::*;
use rug::Integer; 
use crate::math::{ring::{add, add_no_mod, mul_no_mod, neg_no_mod, PolyMulAlgorithm}, util::scale};

use super::{ciphertext::Ciphertext, public_key::PublicKey};

impl PublicKey{
    pub fn add(&self, lhs: &Ciphertext, rhs: &Ciphertext) -> Ciphertext {
        let (c0, c1) = rayon::join(
           || add(&lhs.c0, &rhs.c0, &self.params.p),
           || add(&lhs.c1, &rhs.c1, &self.params.p)
        ); 
        Ciphertext{c0, c1}
    }

    pub fn mul(&self, lhs: &Ciphertext, rhs: &Ciphertext) -> Ciphertext {      
        
        let mut res: Vec<Vec<Integer>> = vec![
            (lhs.c0.clone(), rhs.c0.clone()), 
            (lhs.c1.clone(), rhs.c0.clone()),
            (lhs.c0.clone(), rhs.c1.clone()), 
            (lhs.c1.clone(), rhs.c1.clone()),
        ]
        .into_par_iter() 
        .map(|(a, b)| mul_no_mod(&a, &b, self.params.n, PolyMulAlgorithm::Fft, self.params.precision))
        .collect();   
        
        res[1] = add_no_mod(&res[1], &res[2]); 
        res[2] = res[3].clone(); 
        res.pop();

        let scaled_res: Vec<Vec<Integer>> = res
        .into_par_iter()
        .map(|a| scale(&a, &self.params.p, &self.params.t))
        .collect();

        self.relin(
            &scaled_res[0], &scaled_res[1], &scaled_res[2]
        )
    }

    pub fn neg(&self, c: &Ciphertext) -> Ciphertext {
        let (c0, c1) = rayon::join(
     || neg_no_mod(&c.c0), 
     || neg_no_mod(&c.c1)
        ); 
        Ciphertext{c0, c1}
    }

    
}

