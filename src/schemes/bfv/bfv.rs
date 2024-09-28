use super::{params::{get_params,PARAMS}, public_key::PublicKey, secret_key::SecretKey};

pub struct BFV{
}

impl BFV{
    
    pub fn gen_keys(param_set: PARAMS) -> (SecretKey, PublicKey){
        let params = get_params(param_set);


        
        let sk  = SecretKey::new(&params); 
      
        let pk = PublicKey::new(&sk); 
        

        (sk, pk)
    }
}