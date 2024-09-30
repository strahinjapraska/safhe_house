use std::f64::consts::PI;

use rug::ops::CompleteRound;
use rug::Complex;
use rug::Float; 

use rug::Integer; 

#[derive(PartialEq, Clone)]
pub enum FftMode{
    FFT, 
    IFFT
}

pub fn fft(a: &Vec<Complex>, n: usize, precision: usize, mode: FftMode) -> Vec<Complex>{
    assert!(n.is_power_of_two(), "n must be power of 2");

    if n == 1 {
        return a.to_vec(); 
    }

    let precision_u32 = precision.try_into().unwrap();
    

    let mut w = Complex::with_val((precision_u32, precision_u32), (1.0, 0.0)); 
    let angle = if mode == FftMode::FFT{
        Float::with_val(precision_u32, 2.0*PI/(n as f64))
    }else{
        Float::with_val(precision_u32, -2.0*PI/(n as f64))
    };

    let cos = Float::with_val(precision_u32, angle.clone().cos()); 
    let sin = Float::with_val(precision_u32, angle.sin());
    let w_n = Complex::with_val((precision_u32, precision_u32), (cos, sin)); 

    let a_0: Vec<Complex> = a.iter()
    .step_by(2)
    .cloned()
    .collect();
    
    let a_1: Vec<Complex> = a.iter()
        .skip(1)
        .step_by(2)
        .cloned()
        .collect();

    let a_0_cap = fft(&a_0, n/2, precision, mode.clone()); 
    let a_1_cap = fft(&a_1, n/2, precision, mode.clone()); 

    let mut a_cap: Vec<Complex> = vec![Complex::with_val((precision_u32, precision_u32), (0.0, 0.0)); n];

    for k in 0..n/2{
        a_cap[k] = (&a_0_cap[k] + &w*&a_1_cap[k]).complete((precision_u32, precision_u32));  
        a_cap[k+n/2] = (&a_0_cap[k] - &w*&a_1_cap[k]).complete((precision_u32, precision_u32));
        w*=&w_n;
    }

    a_cap
}
pub fn fft_mul(p: &Vec<Integer> , q: &Vec<Integer>, precision: usize) -> Vec<Integer>{
    assert!(p.len() == q.len()); 

    let n = p.len(); 

    let precision_u32: u32 = precision.try_into().unwrap(); 

    let mut p_cap: Vec<Complex> = p
    .iter()
    .map(|p_val|{ 
        Complex::with_val(precision_u32, p_val)
    }).collect(); 

    let mut q_cap: Vec<Complex> = q
    .iter()
    .map(|q_val|{ 
        Complex::with_val(precision_u32, q_val)
    }).collect(); 

    p_cap.resize(2*n, Complex::with_val(precision_u32, 0.0f32));
    q_cap.resize(2*n, Complex::with_val(precision_u32, 0.0f32));
  
    p_cap = fft(&p_cap, 2*n, precision, FftMode::FFT);
    q_cap = fft(&q_cap, 2*n, precision, FftMode::FFT);

    let mut c: Vec<Complex> = p_cap
    .iter()
    .zip(q_cap.iter())
    .map(|(p_val, q_val)|{ 
        (p_val * q_val).complete((precision_u32, precision_u32))
    }).collect(); 

    c = fft(&c, 2*n, precision, FftMode::IFFT); 

    let n_inv = Complex::with_val(precision_u32, 1.0/(2*n) as f32); 
    c.iter_mut().for_each(|x| {
        *x = (&*x * &n_inv).complete((precision_u32, precision_u32));
    });
    
    c.pop();

    c.iter().map(|x| {
       x.real().clone().round().to_integer().unwrap()
    }).collect()
}