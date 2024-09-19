

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
            s: 8.0,
            n: 1024, 
            p: 1061093377, 
            t: 1024}
    }

    
}