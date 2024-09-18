use std::ops::Add;

use crate::math::ring::QuotientRing;

pub struct Ciphertext{
   pub c0: Vec<i64>,
   pub c1: Vec<i64>, 
   pub ring: QuotientRing
} 

impl Add for Ciphertext{
   type Output = Ciphertext;

   fn add(self, rhs: Self) -> Ciphertext {
      let (c0, c1) = rayon::join(
         || self.ring.add(&self.c0, &rhs.c0),
         || rhs.ring.add(&self.c1, &rhs.c1)
      ); 

      Ciphertext{c0, c1, ring: self.ring.clone()}
   }
}