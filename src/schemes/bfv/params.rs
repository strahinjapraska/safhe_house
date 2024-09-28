use std::collections::HashMap;

use lazy_static::lazy_static;
use rug::Integer;


#[derive(Clone)]
pub struct Params{
    pub (crate) s: f64, 
    pub (crate) n: usize, 
    pub (crate) p: Integer,  
    pub (crate) t: Integer,   
    pub (crate) rp: Integer,
    pub (crate) l: usize, 
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
        let p = Integer::from(1061093377); 
        m.insert(
            "PARAMS1",
            Params {
                s: 8.0,
                n,
                p: p.clone(),
                t: Integer::from(1024),
                rp: p.clone().sqrt(),
                l: 2
            },
            
        );

        let n = 2048; 
        let p = Integer::from(144115188076060673u64); 
        m.insert(
            "PARAMS2", 
            Params { 
                s: 8.0, 
                n, 
                p: p.clone(),
                t: Integer::from(1024), 
                rp: p.clone().sqrt(),
                l: 2
            }
        );
    

        let n = 8192; 
        let p = Integer::from_str_radix("36695977855841144185773134324833391052745039826692497979801421430190766017415756929120296849762010984874789",10).expect("Can't convert"); 
        m.insert(
            "PARAMS3", 
            Params { 
                s: 8.0, 
                n, 
                p: p.clone(), 
                t: Integer::from(1024), 
                rp: p.clone().sqrt(), 
                l: 2
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