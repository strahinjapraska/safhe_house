use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::time::{Duration, Instant};

use csv::Writer;
use safhe_house::schemes::bfv::params::{get_params, PARAMS};
use safhe_house::math::ring::{mul_no_mod, uniform_random_element, PolyMulAlgorithm};

fn run_benchmarks_write_to_file() -> Result<(), Box<dyn Error>>{
    let params = PARAMS::get_all();

    let path = Path::new("src/bin/benches/timings/poly_mul_benchmark_time.csv");

    let file = File::create(&path)?; 
    let mut writer = Writer::from_writer(file); 

    // Too long computation for last set of params
    for p in params.iter().take(params.len() -1){
        
        let param = get_params(p.clone());
        let a = uniform_random_element(&param.p(), param.n());
        let b = uniform_random_element(&param.p(), param.n());
        println!("------------------------------{:?}------------------------------", p);

        let time = Instant::now(); 
        mul_no_mod(&a, &b, param.n(), PolyMulAlgorithm::Karatsuba, param.prec());
        let elapsed1 = time.elapsed(); 
        println!("Karatsuba time: {:?}", elapsed1);

        let time = Instant::now(); 
        mul_no_mod(&a, &b, param.n(), PolyMulAlgorithm::Fft, param.prec());
        let elapsed2= time.elapsed(); 
        println!("Fft time: {:?}", elapsed2);

        let elapsed3: Duration; 
        // Schoolbook algorithm cannot compute this 
        if param.n() < 30000{
            let time = Instant::now();
            mul_no_mod(&a, &b, param.n(), PolyMulAlgorithm::SchoolBook, param.prec());
            elapsed3 = time.elapsed(); 
            println!("Schoolbook time: {:?}", elapsed3);
        }else{
            elapsed3 = Duration::from_secs(0);
        }

        println!("-----------------------------------------------------------------------");

        writer.write_record(&[elapsed1.as_millis().to_string(), elapsed2.as_millis().to_string(), elapsed3.as_millis().to_string(), param.n().to_string()])?;  
        let _= writer.flush();   
        
    }
  

    Ok(())
}
fn main(){
    let _ = run_benchmarks_write_to_file();
}