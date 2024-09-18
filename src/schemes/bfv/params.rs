

#[derive(Clone)]
pub struct Params{
    pub (crate) s: f64, 
    pub (crate) n: usize, 
    pub (crate) p: i64,  
    pub (crate) t: i64,   
}

impl Params{
    pub fn new() -> Params{
        Params{
            s: 1.0,
            n: 4, 
            p: 211, 
            t: 2}
    }

    
}
