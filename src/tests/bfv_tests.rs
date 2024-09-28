#[cfg(test)]
mod bfv_tests{
 

    use safhe_house::schemes::bfv::{bfv::BFV, plaintext::Plaintext};
    use safhe_house::schemes::bfv::params::PARAMS::*;
    use num_bigint::BigInt;



    #[test]
    fn encryption_equation_test(){


        let message = vec![BigInt::from(251); 1024];

  
        
        let (sk, pk) = BFV::gen_keys(RlweParams1);


        let c = pk.encrypt(&Plaintext{message: message.clone()});
        
        let res = sk.decrypt(&c);
        
       
        
        assert_eq!(message, res.message);
    }


    #[test]
    fn homomorphic_add_test(){
        let m1 = vec![BigInt::from(17); 1024];
        let m2 = vec![BigInt::from(411); 1024];

        let clear_res = vec![BigInt::from(428); 1024];


        let (sk, pk) = BFV::gen_keys(RlweParams1); 

        
      
        let c1 = pk.encrypt(&Plaintext{message: m1.clone()}); 
        

        let c2 = pk.encrypt(&Plaintext { message: m2.clone()}); 

        let encrypted_res = pk.add(&c1, &c2);
       
        let decrypted_res = sk.decrypt(&encrypted_res); 

        assert_eq!(clear_res, decrypted_res.message);
    }

    #[test]
    fn homomorphic_mul_test(){
        let n = 2048; 

        let mut m1 = vec![BigInt::from(0);n];
        m1[0] = BigInt::from(5); 
        let mut m2 = vec![BigInt::from(0);n];
        m2[0] = BigInt::from(5); 

        let mut clear_res = vec![BigInt::from(0); n];
        clear_res[0] = BigInt::from(25);

      
        let (sk, pk) = BFV::gen_keys(RlweParams2); 
      

        let c1 = pk.encrypt(&Plaintext{message: m1.clone()}); 
        let c2 = pk.encrypt(&Plaintext{message: m2.clone()}); 

        let encrypted_res = pk.mul(&c1, &c2);
    

        let decrypted_res = sk.decrypt(&encrypted_res); 
        
        assert_eq!(clear_res, decrypted_res.message);
    }
}