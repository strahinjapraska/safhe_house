pub trait Scheme{

    fn encrypt(); 
    fn decrypt(); 
    fn secret_key_gen(&self) -> Vec<u64>; 
    fn public_key_gen(); 
    fn add(); 
    fn mul(); 
     
}