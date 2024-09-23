use rand::Rng; 
use std::f64::consts::PI; 

fn rho_s(x: i128, s: f64) -> f64{
    ((-PI*(x as f64).powf(2.0))/s.powf(2.0)).exp()
}

pub fn sample_z(s: f64, n: usize) -> i128{
    let t_n = (n as f64).log2(); 
    let left_bound = (-s*t_n).ceil() as i128; 
    let right_bound = (s*t_n).floor() as i128; 

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
