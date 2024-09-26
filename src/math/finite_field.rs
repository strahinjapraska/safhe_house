use num_bigint::BigInt;


pub fn primitive_nth_root_of_unity(p: &BigInt, n: usize) -> BigInt{
    
    let mut x = BigInt::from(1i32); 
    let exponent = (p- BigInt::from(1i32))/n; 

    loop{ 
        
        let g = x.modpow(&exponent, &p);

        if g.modpow(&BigInt::from(n/2), &p) != BigInt::from(1i32){
            return g;
        }
        x+=1; 
    }
}


pub fn modulo(a: &BigInt, p: &BigInt) -> BigInt{
    assert!(*p > BigInt::from(1i32));

    let mut rem = reduce(a, p);

    if rem > p / 2 {
        rem -= p;
    } else if rem <= -p / 2 {
        rem += p;
    }

    rem
}

pub fn reduce(a: &BigInt, p: &BigInt) -> BigInt{
    a.modpow(&BigInt::from(1i32), &p)
}

pub fn square_root_mod_p(n: &BigInt , p: &BigInt) -> BigInt{
    // Tonelli-Shanks algorithm 


    // Euler's cirterion 
    assert!(legrende_symbol(n, p) == BigInt::from(1i32)); 

    let mut q = (p - BigInt::from(1i32))/BigInt::from(2i32); 
    let mut s = 1; 
    while &q%2 == BigInt::from(0i32){
        q/=BigInt::from(2i32); 
        s+=1; 
    }

    let mut z = BigInt::from(2i32); 

    while legrende_symbol(&z, p) != p-1{
        z+=1; 
    }

    let mut m = s; 
    let mut c = z.modpow(&q, p);
    let mut t = n.modpow(&q, p);
    let mut r = n.modpow(&((q+1)/2), p);

    let mut t2; 
    while reduce(&(&t - BigInt::from(1i32)), p) != BigInt::from(0i32){
        t2 = reduce(&(&t*&t), p); 

        let mut i = 1; 
        while i < m{
            if reduce(&(&t2-BigInt::from(1i32)), p) == BigInt::from(0i32){
                break; 
            }
            t2 = reduce(&(&t2*&t2), p); 
            i+=1; 
        }
    
    
        let b = c.modpow(&BigInt::from(2i32).pow(m-i-1), p);
        r = reduce(&(r*&b),p); 
        c = reduce(&(&b*&b), p); 
        t = reduce(&(t*&c), p); 
        m = i; 
    }

    r 
}

pub fn legrende_symbol(a: &BigInt, p: &BigInt) -> BigInt{
    let exponent = (p - BigInt::from(1i32))/2;
    a.modpow(&exponent, p)
}