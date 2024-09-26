#[cfg(test)]
mod bfv_tests{
    use std::time::Instant;

    use safhe_house::math::karatsuba::{self, karatsuba};
    use safhe_house::math::ring::ring_no_mod::mul_no_mod;
    use safhe_house::schemes::bfv::{bfv::BFV, plaintext::Plaintext};
    use safhe_house::schemes::bfv::params::PARAMS::*;
    use num_bigint::BigInt;



    #[test]
    fn encryption_equation_test(){


        let message = vec![BigInt::from(251); 1024];

        let start = Instant::now(); 
        
        let (sk, pk) = BFV::gen_keys(RlweParams1);


        let c = pk.encrypt(&Plaintext{message: message.clone()});
        
        let res = sk.decrypt(&c);
        
        println!("{:?}", start.elapsed());
        
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
    fn test(){
        let n = 2048; 
        let mut m1 = vec![BigInt::from(0);n];
        m1[n-1] = BigInt::from(5); 
        let mut m2 = vec![BigInt::from(0);n];
        m2[n-1] = BigInt::from(5); 
        println!("{:?}",mul_no_mod(&m1, &m2, n));
    }
    #[test]
    fn homomorphic_mul_test(){

        let n = 2048; 

        let mut m1 = vec![BigInt::from(0);n];
        m1[n-1] = BigInt::from(5); 
        let mut m2 = vec![BigInt::from(0);n];
        m2[n-1] = BigInt::from(5); 

        let mut clear_res = vec![BigInt::from(0); n];
        clear_res[n-1] = BigInt::from(25);

        let start = Instant::now(); 
        let (sk, pk) = BFV::gen_keys(RlweParams2); 
        println!("{:?}", start.elapsed());

        let c1 = pk.encrypt(&Plaintext{message: m1.clone()}); 
        let c2 = pk.encrypt(&Plaintext{message: m2.clone()}); 

        let encrypted_res = pk.mul(&c1, &c2);

        let decrypted_res = sk.decrypt(&encrypted_res); 
        
        assert_eq!(clear_res, decrypted_res.message);
    }
}