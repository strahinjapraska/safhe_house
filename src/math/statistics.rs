use rand::Rng; 
use std::f64::consts::PI; 

pub fn rho_s(x: u64, s: f64) -> f64{
    (-PI * (x as f64).powi(2) / s.powi(2)).exp()
}

pub fn sample_z(s: f64, n: usize) -> u64{
    let t_n = (n as f64).ln(); 
    let left_bound = (-s*t_n).ceil() as u64; 
    let right_bound = (s*t_n).floor() as u64; 

    let mut rng = rand::thread_rng(); 

    loop{
        let x = rng.gen_range(left_bound..=right_bound); 
        let acceptance_prob = rho_s(x, s); 

        let u: f64 = rng.gen(); 

        if u <= acceptance_prob{
            return x;  
        }
    }
}

pub fn sample_discrete_gaussian_ring_element(d: usize, s: f64, n: usize) -> Vec<u64>{
    (1..d).map(|_| sample_z(s, n)).collect()
}