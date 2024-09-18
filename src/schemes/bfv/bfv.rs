use super::{public_key::PublicKey, secret_key::SecretKey};

pub struct BFV{
}

impl BFV{
    pub fn gen_keys() -> (SecretKey, PublicKey){
        let sk  = SecretKey::new(); 
        let pk = PublicKey::new(&sk); 

        (sk, pk)
    }
}