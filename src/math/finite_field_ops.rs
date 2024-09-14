
use rand::Rng;

pub fn primitive_nth_root_of_unity(p: u64, n: usize) -> u64{
    let mut rng = rand::thread_rng(); 

    loop{ 
        let x  = rng.gen_range(1..p);
        let g = mod_pow(x, (p-1)/n as u64, p); 

        if mod_pow(g, (n/2) as u64, p) != 1u64{
            return g; 
        }
    }
}

pub fn mod_pow(g: u64,k: u64, p: u64) -> u64{
    // Montgomery-Ladder modpow 
    let mut r0 = 1u64;
    let mut r1 = g; 

    let k_bin = format!("{:b}", k); 
    for b in k_bin.chars().rev(){
        if b == '0' {
            r1 = (r1*r0)%p;
            r0 = (r0*r0)%p;
        }else{
            r0 = (r0*r1)%p;
            r1 = (r1*r1)%p; 
        }
    }
    r0 
}

pub fn modulo(a: u64, p: u64) -> u64{
    panic!("Not implemented")
}