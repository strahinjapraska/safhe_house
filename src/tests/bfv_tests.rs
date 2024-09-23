#[cfg(test)]
mod bfv_tests{
    use safhe_house::schemes::bfv::{bfv::BFV, plaintext::Plaintext};



    #[test]
    fn encryption_equation_test(){

        let message = vec![14; 1024];
        
        let (sk, pk) = BFV::gen_keys();

        let c = pk.encrypt(&Plaintext{message: message.clone()});
        
        let res = sk.decrypt(&c);
        
        assert_eq!(message, res.message);
    }


    #[test]
    fn homomorphic_add(){
        let m1 = vec![5; 1024];
        let m2 = vec![19; 1024];

        let clear_res = vec![24; 1024];

        let (sk, pk) = BFV::gen_keys(); 

        let c1 = pk.encrypt(&Plaintext{message: m1.clone()}); 
        let c2 = pk.encrypt(&Plaintext { message: m2.clone()}); 

        let encrypted_res = pk.add(&c1, &c2);

        let decrypted_res = sk.decrypt(&encrypted_res); 

        assert_eq!(clear_res, decrypted_res.message);
    }

    #[test]
    fn homomorphic_mul(){

        let m1 = vec![5;1024]; 
        let m2 = vec![5;1024];


        let clear_res = vec![25; 1024];

        let (sk, pk) = BFV::gen_keys(); 

        let c1 = pk.encrypt(&Plaintext{message: m1.clone()}); 
        let c2 = pk.encrypt(&Plaintext{message: m2.clone()}); 

        let encrypted_res = pk.mul(&c1, &c2);

        let decrypted_res = sk.decrypt(&encrypted_res); 

        assert_eq!(clear_res, decrypted_res.message);
    }
}