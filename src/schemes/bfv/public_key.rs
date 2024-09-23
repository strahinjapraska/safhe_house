

use crate::math::ring::ring::QuotientRing;
use super::{ciphertext::Ciphertext, params::Params, plaintext::Plaintext, secret_key::SecretKey};
use rayon::prelude::*; 

pub struct PublicKey{
    a: Vec<i128>, 
    b: Vec<i128>, 
    params: Params,
    ring: QuotientRing, 
    rlk: Vec<(Vec<i128>, Vec<i128>)>
}

impl PublicKey{
    pub(crate) fn new(s: &SecretKey)-> PublicKey{
        let params = Params::new(); 
        let ring = QuotientRing{n: params.n, p: params.p};
        let a = ring.uniform_random_element();
        let e = ring.discrete_gaussian_random_element(params.s);
        
        let b = ring.neg(&ring.add(&mut ring.mul(&a, &s.secret), &e)); 

        let rlk  = gen_rlk(&ring, &params, &s);

        PublicKey{a, b, params, ring, rlk}
    }

    pub fn encrypt(&self, pt: &Plaintext) -> Ciphertext{
        let p0 = &self.b; 
        let p1 = &self.a; 

        let u = self.ring.binary_random_element(); 
        let e1 = self.ring.discrete_gaussian_random_element(self.params.s); 
        let e2 = self.ring.discrete_gaussian_random_element(self.params.s);  
        
        let delta = (((self.params.p as f64) / (self.params.t as f64)).floor()) as i128; 
        let c0 = 
        self.ring.add(
            &e1 
        ,&self.ring.add(
            &self.ring.mul(p0,&u),
            &self.ring.scalar_mul(delta, &pt.message)
        ));
        let c1 =  self.ring.add(&e2, &self.ring.mul(&p1, &u)); 
        
        Ciphertext{c0, c1}
    }

    pub fn add(&self, lhs: &Ciphertext, rhs: &Ciphertext) -> Ciphertext {
        let ring_clone = self.ring.clone();
        let (c0, c1) = rayon::join(
           || self.ring.add(&lhs.c0, &rhs.c0),
           || ring_clone.add(&lhs.c1, &rhs.c1)
        ); 
        Ciphertext{c0, c1}
    }

    pub fn mul(&self, lhs: &Ciphertext, rhs: &Ciphertext) -> Ciphertext {      
        
        self.relin(
           &scale(&self.ring, &self.params,&self.ring.mul_no_mod(&lhs.c0, &rhs.c0)), 
           &scale(&self.ring, &self.params,&self.ring.add_no_mod(&self.ring.mul_no_mod(&lhs.c0, &rhs.c1), &self.ring.mul_no_mod(&lhs.c1, &rhs.c0))), 
           &scale(&self.ring, &self.params,&self.ring.mul_no_mod(&lhs.c1, &rhs.c1))
        )
    }

    fn relin(&self, c0: &Vec<i128>, c1: &Vec<i128> , c2: &Vec<i128>) -> Ciphertext{
        return Ciphertext{c0: c0.clone(), c1: c1.clone()};
        let l = (self.params.p as f64).log(self.params.t as f64) as usize; 
       
        let mut decomposition: Vec<Vec<i128>> = vec![Vec::new(); l+1];

        decomposition.iter_mut().enumerate().for_each(|(j, vec)| {
            c2.iter().for_each(|&a_i| {
                let b_ij = (a_i / (self.params.t<<j)) % self.params.t;
                vec.push(b_ij);
            });
        });

        let mut s1 = self.ring.mul(&self.rlk[0].0, &decomposition[0]);
        let mut s2 = self.ring.mul(&self.rlk[0].1, &decomposition[0]);
        for i in 0..=l{
            s1 = self.ring.add(&self.ring.mul(&self.rlk[i].0, &decomposition[i]), &s1); 
            s2 = self.ring.add(&self.ring.mul(&self.rlk[i].1, &decomposition[i]), &s2); 
        }

        Ciphertext{c0: self.ring.add(c0, &s1), c1: self.ring.add(c1, &s2)}
    }

   
}

fn gen_rlk(ring: &QuotientRing, params: &Params, secret_key: &SecretKey) -> Vec<(Vec<i128>,Vec<i128>)>{

    let l = (params.p as f64).log(params.t as f64) as i128; 
    let mut rlk :Vec<(Vec<i128>, Vec<i128>)> = vec![]; 

    let s_squared = ring.mul(&secret_key.secret, &secret_key.secret); 
    for i in 0..=l{
        let a_i = ring.uniform_random_element();
        let e_i = ring.discrete_gaussian_random_element(params.s); 
        // we consider T to be power of two 
        rlk.push(
            (ring.add(&ring.neg(&ring.add(&ring.mul(&a_i, &secret_key.secret), &e_i)), &ring.scalar_mul(params.t<<i, &s_squared)), 
            a_i)
        )
    }

    rlk 
    
}

fn scale(ring: &QuotientRing, params: &Params, c: &Vec<i128>) -> Vec<i128>{
    ring.scalar_div(params.p, 
    &ring.scalar_mul_no_mod(params.t, &c)
    )
}