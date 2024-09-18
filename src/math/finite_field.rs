pub fn primitive_nth_root_of_unity(p: i64, n: usize) -> i64{
    
    let mut x = 1; 
    loop{ 
        let g = mod_pow(x, (p-1)/n as i64, p); 

        if mod_pow(g as i64, (n/2) as i64, p) != 1i64{
            return g; 
        }
        x+=1; 
    }
}

pub fn legrende_symbol(a: i64, p: i64) -> i64{
    mod_pow(a, (p-1)/2, p)
}

pub fn mod_pow(g: i64,k: i64, p: i64) -> i64{
    // Montgomery-Ladder modpow 
    let mut r0 = 1i64;
    let mut r1 = g; 

    let k_bin = format!("{:b}", k); 
    for b in k_bin.chars(){
        if b == '0' {
            r1 = reduce(r1*r0, p);
            r0 = reduce(r0*r0, p);
        }else{
            r0 = reduce(r0*r1, p);
            r1 = reduce(r1*r1, p); 
        }
    }
    r0
}


pub fn modulo(a: i64, p: i64) -> i64{
    assert!(p > 1); 

    let rem = reduce(a, p);

    if rem > p/2{
        rem - p 
    }
    else{
        rem 
    }
}

pub fn reduce(a: i64, p: i64) -> i64{

    // Barret Reduction 
    let q = a/p; 
    a - q*p

}

pub fn square_root_mod_p(n: i64 , p: i64) -> i64{
    // Tonelli-Shanks algorithm 

    // Euler's cirterion 
    assert!(mod_pow(n, (p-1)/2, p) == 1i64);

    let mut q = (p-1)/2; 
    let mut s = 1; 
    while q%2 == 0{
        q /=2; 
        s+=1; 
    }

    let mut z = 2; 

    while legrende_symbol(z, p ) != p-1{
        z+=1; 
    }

    let mut m = s; 
    let mut c = mod_pow(z, q, p); 
    let mut t = mod_pow(n, q, p); 
    let mut r = mod_pow(n, (q+1)/2, p); 

    let mut t2; 
    while reduce(t-1, p) != 0{
        t2 = reduce(t*t, p); 

        let mut i = 1; 
        while i < m{
            if reduce(t2-1, p) == 0{
                break; 
            }
            t2 = reduce(t2*t2, p); 
            i+=1; 
        }
        let b = mod_pow(c, 1<<(m - i - 1), p); 
        r = reduce(r*b,p); 
        c = reduce(b*b, p); 
        t = reduce(t*c, p); 
        m = i; 
    }

    r 
}

pub fn inv_mod(a: i64, p: i64) -> i64{
    mod_pow(a, p-2, p)
}
