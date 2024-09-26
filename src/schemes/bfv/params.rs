use std::collections::HashMap;

use num::Num;
use num_bigint::BigInt;
use lazy_static::lazy_static;


#[derive(Clone)]
pub struct Params{
    pub (crate) s: f64, 
    pub (crate) n: usize, 
    pub (crate) p: BigInt,  
    pub (crate) t: BigInt,   
}

pub enum PARAMS{
    /// D = 1, log(p) = 30, t = 1024, n = 1024
    RlweParams1, 
    /// D = 2, log(p) = 58, t = 1024, n = 2048
    RlweParams2,
    /// D = 3, log(p) = 91, t = 1024, n = 4096 
    RlweParams3
}

lazy_static!{
    static ref PARAMSETS: HashMap<&'static str, Params> = {
        let mut m = HashMap::new();

        let n = 1024; 
        let p = BigInt::from(1061093377); 
        m.insert(
            "PARAMS1",
            Params {
                s: 8.0,
                n,
                p,
                t: BigInt::from(1024),
            },
            
        );

        let n = 2048; 
        let p = BigInt::from(144115188076060673u64); 
        m.insert(
            "PARAMS2", 
            Params { 
                s: 8.0, 
                n, 
                p, 
                t: BigInt::from(1024), 
            }
        );
    

        let n = 8192; 
        let p = BigInt::from_str_radix("36695977855841144185773134324833391052745039826692497979801421430190766017415756929120296849762010984874789",10).expect("Can't convert"); 
        m.insert(
            "PARAMS3", 
            Params { 
                s: 8.0, 
                n, 
                p, 
                t: BigInt::from(1024), 
            }
        );
        m
    };
}
pub fn get_params(param: PARAMS) -> Params{
    let name = match param {
        PARAMS::RlweParams1 => "PARAMS1", 
        PARAMS::RlweParams2 => "PARAMS2",
        PARAMS::RlweParams3 => "PARAMS3",
    };
    PARAMSETS.get(name).cloned().expect("No such params")
}