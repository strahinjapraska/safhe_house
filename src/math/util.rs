use num_bigint::BigInt; 
use rand::Rng;

pub fn random_binary_vector(n: usize) -> Vec<BigInt>{

    let mut rng = rand::thread_rng(); 

    (0..n)
    .map(|_| {
            let num = rng.gen_range(0..=1);
            BigInt::from(num)
        })
    .collect()

}
