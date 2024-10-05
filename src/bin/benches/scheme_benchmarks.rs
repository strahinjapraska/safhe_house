use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::time::{Duration, Instant};

use csv::Writer;
use safhe_house::schemes::bfv::params::{get_params, PARAMS};
use safhe_house::math::ring::{mul_no_mod, uniform_random_element, PolyMulAlgorithm};

fn run_benchmarks_write_to_file() -> Result<(), Box<dyn Error>>{
    let params = [
    RlweParams1, RlweParams2, RlweParams3, RlweParams4, RlweParams5, RlweParams6, RlweParams7, RlweParams8, RlweParams9, RlweParams10
   ].to_vec();

    let path = Path::new("src/bin/benches/timings/scheme_benchmark_time.csv");

    let file = File::create(&path)?; 
    let mut writer = Writer::from_writer(file); 

    for p in params{
        
        let param = get_params(p.clone());

        let time = Instant::now(); 
        let elapsed1 = time.elapsed();

        writer.write_record(&[])?;  
        let _= writer.flush();   
        
    }
  

    Ok(())
}
fn main(){
    let _ = run_benchmarks_write_to_file();
}