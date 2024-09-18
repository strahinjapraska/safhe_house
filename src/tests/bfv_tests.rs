#[cfg(test)]
mod bfv_tests{
    use safhe_house::schemes::bfv::{bfv::BFV, plaintext::Plaintext};



    #[test]
    fn encryption_equation_test(){
        let message = vec![0, 1, 1, 0];
        let (sk, pk) = BFV::gen_keys();

        let c = pk.encrypt(&Plaintext{message: message.clone()});
        
        let res = sk.decrypt(&c);
        
        assert_eq!(message, res.message);
    }

    #[test]
    fn homomorphic_add(){
        let m1 = vec![0, 1, 0, 0]; 

        let m2 = vec![0, 0, 0, 1]; 

        let (sk, pk) = BFV::gen_keys(); 

        let c1 = pk.encrypt(&Plaintext{message: m1.clone()}); 
        let c2 = pk.encrypt(&Plaintext { message: m2.clone()}); 

        let encrypted_res = c1+c2;

        let decrypted_res = sk.decrypt(&encrypted_res); 

        assert_eq!(vec![0, 1, 0, 1], decrypted_res.message);
    }
}