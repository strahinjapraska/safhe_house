use rug::Integer;

#[derive(Clone)]
pub struct Ciphertext{
   pub c0: Vec<Integer>,
   pub c1: Vec<Integer>, 
} 