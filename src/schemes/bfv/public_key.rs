use crate::math::ring::QuotientRing;

use super::{ciphertext::Ciphertext, params::Params, plaintext::Plaintext, secret_key::SecretKey};

pub struct PublicKey{
    a: Vec<i64>, 
    b: Vec<i64>, 
    params: Params,
    ring: QuotientRing, 
}

impl PublicKey{
    pub(crate) fn new(s: &SecretKey)-> PublicKey{
        let params = Params::new(); 
        let ring = QuotientRing{n: params.n, p: params.p};
        let a = ring.uniform_random_element();
        let e = ring.discrete_gaussian_random_element(params.s);
        
        let b = ring.neg(&ring.add(&mut ring.mul(&a, &s.secret), &e)); 

        PublicKey{a, b, params, ring}
    }

    pub fn encrypt(&self, pt: &Plaintext) -> Ciphertext{
        let p0 = &self.b; 
        let p1 = &self.a; 

        let u = self.ring.binary_random_element(); 
        let e1 = self.ring.discrete_gaussian_random_element(self.params.s); 
        let e2 = self.ring.discrete_gaussian_random_element(self.params.s);  
        
        let delta = (((self.params.p as f64) / (self.params.t as f64)).floor()) as i64; 
        let c0 = 
        self.ring.add(
            &e1 
        ,&self.ring.add(
            &self.ring.mul(p0,&u),
            &self.ring.scalar_mul(delta, &pt.message)
        ));
        let c1 =  self.ring.add(&e2, &self.ring.mul(&p1, &u)); 
        
        Ciphertext{c0, c1, ring: self.ring.clone(), params: self.params.clone()}
    }
}