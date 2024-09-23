use rand::Rng;

pub fn random_binary_vector(n: usize) -> Vec<i128>{

    let mut rng = rand::thread_rng(); 

    (0..n)
    .map(|_| rng.gen_range(0..=1))
    .collect()

}

pub fn random_uniform_vector(n: usize, p: i128) -> Vec<i128>{

    let mut rng = rand::thread_rng(); 

    (0..n)
    .map(|_| rng.gen_range(0..=p-1))
    .collect()

}