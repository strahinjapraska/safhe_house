use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::time::Instant;

use csv::Writer;
use rug::Integer;
use safhe_house::schemes::bfv::bfv::BFV;
use safhe_house::schemes::bfv::params::{get_params, PARAMS};
use safhe_house::math::ring::discrete_gaussian_random_element;
use safhe_house::schemes::bfv::plaintext::Plaintext;
use PARAMS::*;

fn run_benchmarks_write_to_file() -> Result<(), Box<dyn Error>>{
    let params = [
    RlweParams1, RlweParams2, RlweParams3, RlweParams4, RlweParams5, RlweParams6, RlweParams7, RlweParams8, RlweParams9, RlweParams10
   ].to_vec();

    let path = Path::new("src/bin/benches/timings/scheme_benchmark_time.csv");

    let file = File::create(&path)?; 
    let mut writer = Writer::from_writer(file); 

    for p in params{
        
        let param = get_params(p.clone());

        let mut m1 = vec![Integer::from(0);param.n()];
        m1[0] = Integer::from(5); 
        let mut m2 = vec![Integer::from(0);param.n()];
        m2[0] = Integer::from(5); 

        let time = Instant::now();
        let _ = discrete_gaussian_random_element(8.0, param.n());
        let elapsed1 = time.elapsed();

        let time = Instant::now();
        let (sk, pk) = BFV::gen_keys(p); 
        let elapsed2 = time.elapsed();

        let time = Instant::now();
        let c1 = pk.encrypt(&Plaintext{message: m1.clone()});
        let elapsed3 = time.elapsed();

        let c2 = pk.encrypt(&Plaintext{message: m2.clone()}); 

        let time = Instant::now();
        let _ = pk.add(&c1, &c2);
        let elapsed4 = time.elapsed();

        let time = Instant::now();
        let encrypted_res = pk.mul(&c1, &c2);
        let elapsed5 = time.elapsed();


        let time = Instant::now();
        let _ = pk.sub(&c1, &c2);
        let elapsed6 = time.elapsed();

        let time = Instant::now();
        let _ = pk.neg(&c1);
        let elapsed7 = time.elapsed();

        let time = Instant::now();
        let _ = sk.decrypt(&encrypted_res); 
        let elapsed8 = time.elapsed();

 
        
        writer.write_record(&[elapsed1.as_millis().to_string(), elapsed2.as_millis().to_string(), elapsed3.as_millis().to_string(), elapsed4.as_micros().to_string(),
        elapsed5.as_millis().to_string(),elapsed6.as_micros().to_string(),elapsed7.as_micros().to_string(), elapsed8.as_millis().to_string()])?;  
        let _= writer.flush(); 
        
    }
  

    Ok(())
}
fn main(){
    let _ = run_benchmarks_write_to_file();
}