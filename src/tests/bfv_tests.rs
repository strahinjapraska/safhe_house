#[cfg(test)]
mod bfv_tests{
 


    use std::time::Instant;
    use safhe_house::schemes::bfv::{bfv::BFV, plaintext::Plaintext};
    use safhe_house::schemes::bfv::params::PARAMS::*;
    use rug::Integer;



    #[test]
    fn encryption_equation_test(){


        let message = vec![Integer::from(251); 1024];
        
        let (sk, pk) = BFV::gen_keys(RlweParams1);


        let c = pk.encrypt(&Plaintext{message: message.clone()});
        
        let res = sk.decrypt(&c);
        
       
        
        assert_eq!(message, res.message);
    }


    #[test]
    fn homomorphic_add_test(){
        let m1 = vec![Integer::from(-8); 1024];
        let m2 = vec![Integer::from(411); 1024];

        let clear_res = vec![Integer::from(403); 1024];


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

        let mut m1 = vec![Integer::from(0);n];
        m1[0] = Integer::from(5); 
        let mut m2 = vec![Integer::from(0);n];
        m2[0] = Integer::from(5); 

        let mut clear_res = vec![Integer::from(0); n];
        clear_res[0] = Integer::from(25);


        let (sk, pk) = BFV::gen_keys(RlweParams2); 


        let c1 = pk.encrypt(&Plaintext{message: m1.clone()});
        let c2 = pk.encrypt(&Plaintext{message: m2.clone()}); 


        let time = Instant::now();
        let encrypted_res = pk.mul(&c1, &c2);
        println!("Mul: {:?}", time.elapsed());

        let decrypted_res = sk.decrypt(&encrypted_res); 
        
        assert_eq!(clear_res, decrypted_res.message);
    }

    #[test]
    fn homomorphic_neg_test(){
        let m1 = vec![Integer::from(-17); 1024];

        let clear_res = vec![Integer::from(17); 1024];

        let (sk, pk) = BFV::gen_keys(RlweParams1); 

        let c1 = pk.encrypt(&Plaintext{message: m1.clone()}); 
        
        let encrypted_res = pk.neg(&c1);
       
        let decrypted_res = sk.decrypt(&encrypted_res); 

        assert_eq!(clear_res, decrypted_res.message);
    }
    
}