use std::ops::{Add, Mul};

use crate::math::ring::QuotientRing;

use super::params::Params;

pub struct Ciphertext{
   pub c0: Vec<i64>,
   pub c1: Vec<i64>, 
   pub(crate) ring: QuotientRing, 
   pub params: Params
} 

impl Add for Ciphertext{
   type Output = Ciphertext;

   fn add(self, rhs: Self) -> Ciphertext {
      let (c0, c1) = rayon::join(
         || self.ring.add(&self.c0, &rhs.c0),
         || rhs.ring.add(&self.c1, &rhs.c1)
      ); 

      Ciphertext{c0, c1, ring: self.ring.clone(), params: self.params}
   }
}

impl Mul for Ciphertext{
   type Output = Ciphertext;

   fn mul(self, rhs: Self) -> Self::Output {      
      self.relin(
         &self.scale(&self.ring.mul(&self.c0, &rhs.c0)), 
         &self.scale(&self.ring.add(&self.ring.mul(&self.c0, &rhs.c1), &self.ring.mul(&self.c1, &rhs.c0))), 
         &self.scale(&self.ring.mul(&self.c1, &rhs.c1))
      )
   }
}

impl Ciphertext{
   fn scale(&self, c: &Vec<i64>) -> Vec<i64>{
         self.ring.scalar_div(
            self.params.p, &self.ring.scalar_mul_no_mod(self.params.t, &c)
         )
   }
   fn relin(&self, c0: &Vec<i64>, c1: &Vec<i64> , c2: &Vec<i64>) -> Ciphertext{
         panic!("Not implemented");
   }
}