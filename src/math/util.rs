use rand::Rng;
use rug::Integer;

use super::ring::{scalar_div, scalar_mul_no_mod};


pub fn random_binary_vector(n: usize) -> Vec<Integer>{

    let mut rng = rand::thread_rng(); 

    (0..n)
    .map(|_| {
            let num = rng.gen_range(0..=1);
            Integer::from(num)
        })
    .collect()

}

pub fn scale(c: &Vec<Integer>, p: &Integer, t: &Integer) -> Vec<Integer>{
    scalar_div(p, 
    &scalar_mul_no_mod(t, &c)
    ,p)
}
