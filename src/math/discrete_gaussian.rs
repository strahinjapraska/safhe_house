use rand::Rng; 
use std::f64::consts::PI; 

fn rho_s(x: i64, s: f64) -> f64{
    (-PI * (x as f64).powi(2) / s.powi(2)).exp()
}

pub fn sample_z(s: f64, n: usize) -> i64{
    let t_n = (n as f64).log2(); 
    let left_bound = (-s*t_n).ceil() as i64; 
    let right_bound = (s*t_n).floor() as i64; 

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

pub fn sample_discrete_gaussian_ring_element(n: usize, s: f64) -> Vec<i64>{
    (0..n).map(|_| sample_z(s, n)).collect()
}