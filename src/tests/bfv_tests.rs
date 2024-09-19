#[cfg(test)]
mod bfv_tests{
    use safhe_house::{math::{ring::QuotientRing, util::random_uniform_vector}, schemes::bfv::{bfv::BFV, plaintext::Plaintext}};



    #[test]
    fn encryption_equation_test(){

        let message = random_uniform_vector(1024, 512);
        
        let (sk, pk) = BFV::gen_keys();

        let c = pk.encrypt(&Plaintext{message: message.clone()});
        
        let res = sk.decrypt(&c);
        
        assert_eq!(message, res.message);
    }


    #[test]
    fn homomorphic_add(){
        let ring = QuotientRing{n: 1024, p: 1024};

        let m1 = random_uniform_vector(1024, 512);
        let m2 = random_uniform_vector(1024, 512);

        let clear_res = ring.add(&m1, &m2);


        let (sk, pk) = BFV::gen_keys(); 

        let c1 = pk.encrypt(&Plaintext{message: m1.clone()}); 
        let c2 = pk.encrypt(&Plaintext { message: m2.clone()}); 

        let encrypted_res = c1+c2;

        let decrypted_res = sk.decrypt(&encrypted_res); 

        assert_eq!(clear_res, decrypted_res.message);
    }

    #[test]
    fn homomorphic_mul(){
        let mut m1 = vec![0;1023];
        m1.push(2);
        let mut m2 = vec![0; 1023];
        m2.push(4); 

        let mut clear_res = vec![0; 1023];
        clear_res.push(8);

        let (sk, pk) = BFV::gen_keys(); 

        let c1 = pk.encrypt(&Plaintext{message: m1.clone()}); 
        let c2 = pk.encrypt(&Plaintext { message: m2.clone()}); 

        let encrypted_res = c1*c2;

        let decrypted_res = sk.decrypt(&encrypted_res); 

        assert_eq!(clear_res, decrypted_res.message);
    }
}