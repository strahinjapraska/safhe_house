pub fn primitive_nth_root_of_unity(p: i128, n: usize) -> i128{
    
    let mut x = 1; 
    loop{ 
        let g = mod_pow(x, (p-1)/n as i128, p); 

        if mod_pow(g as i128, (n/2) as i128, p) != 1i128{
            return g; 
        }
        x+=1; 
    }
}

pub fn legrende_symbol(a: i128, p: i128) -> i128{
    mod_pow(a, (p-1)/2, p)
}

pub fn mod_pow(g: i128,k: i128, p: i128) -> i128{
    // Montgomery-Ladder modpow 
    let mut r0 = 1i128;
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


pub fn modulo(a: i128, p: i128) -> i128 {
    assert!(p > 1);

    let mut rem = reduce(a, p);

    if rem > p / 2 {
        rem -= p;
    } else if rem <= -p / 2 {
        rem += p;
    }

    rem
}

pub fn reduce(a: i128, p: i128) -> i128 {
    let mut rem = a % p;
    if rem < 0 {
        rem += p;
    }
    rem
}

pub fn square_root_mod_p(n: i128 , p: i128) -> i128{
    // Tonelli-Shanks algorithm 

    // Euler's cirterion 
    assert!(mod_pow(n, (p-1)/2, p) == 1i128);

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

pub fn inv_mod(a: i128, p: i128) -> i128{
    mod_pow(a, p-2, p)
}
