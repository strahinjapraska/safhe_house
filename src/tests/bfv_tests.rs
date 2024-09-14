#[cfg(test)]
mod bfv_tests{

    use safhe_house::schemes::bfv::*; 
    use safhe_house::math::discrete_gaussian::*;
    use safhe_house::schemes::scheme::Scheme; 
    #[test]
    fn init(){

        let bfv = BFV::new(16); 
        println!("Initialized BFV");
        println!("SampleZ: {}", sample_z(2.3, 64));
        println!("Sample ring element: {:?}", sample_discrete_gaussian_ring_element(20, 2.3, 64)); 
        println!("Key-gen: {:?}", bfv.secret_key_gen());
        
    }
}