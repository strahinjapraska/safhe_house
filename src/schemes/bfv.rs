use crate::schemes::scheme::Scheme;
use crate::math::statistics::sample_discrete_gaussian_ring_element; 

pub struct BFV{
    d: usize, 
    n: usize, 
    s: f64, 
}

impl Scheme for BFV{
    fn encrypt() {
        todo!()
    }

    fn decrypt() {
        todo!()
    }

    fn secret_key_gen(&self) -> Vec<u64>{
        sample_discrete_gaussian_ring_element(self.d, self.s, self.n)
    }

    fn public_key_gen() {
        todo!()
    }

    fn add() {
        todo!()
    }

    fn mul() {
        todo!()
    }
    
    
}

impl BFV{
    pub fn new() -> BFV{
        BFV{d: 16, s: 2.3, n:64}
    }
}