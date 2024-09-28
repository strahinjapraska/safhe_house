use rug::{ops::Pow, Complete, Integer};

pub fn primitive_nth_root_of_unity(p: &Integer, n: usize) -> Integer{
    
    let mut x = Integer::from(1i32); 
    let exponent = (p- Integer::from(1i32))/n; 

    loop{ 
        
        let g = x.clone().pow_mod(&exponent, &p).expect("Cannot mod pow");

        if g.clone().pow_mod(&Integer::from(n/2), &p).expect("Cannot mod pow") != Integer::from(1i32){
            return g;
        }
        x+=1; 
    }
}


pub fn modulo(a: &Integer, p: &Integer) -> Integer{
    assert!(*p > Integer::from(1i32));

    let mut rem = reduce(a, p);

    let half_p = Integer::from(p/2); 

    if rem > half_p {
        rem -= p;
    } else if rem <= -half_p {
        rem += p;
    }

    rem
}

pub fn reduce(a: &Integer, p: &Integer) -> Integer{
    a.clone().modulo(p)
}

pub fn square_root_mod_p(n: &Integer , p: &Integer) -> Integer{
    // Tonelli-Shanks algorithm 


    // Euler's cirterion 
    assert!(legrende_symbol(n, p) == Integer::from(1i32)); 

    let mut q = (p - Integer::from(1i32))/Integer::from(2i32); 
    let mut s = 1; 
    while Integer::from(q%2) == Integer::from(0i32){
        q/=Integer::from(2i32); 
        s+=1; 
    }

    let mut z = Integer::from(2i32); 

    while legrende_symbol(&z, p) != Integer::from(p-1){
        z+=1; 
    }

    let mut m = s; 
    let mut c = z.pow_mod(&q, p).expect("Cannot modpow");
    let mut t = n.clone().pow_mod(&q, p).expect("Cannot modpow");
    let mut r = n.clone().pow_mod(&((q+1)/2), p).expect("Cannot modpow");

    let mut t2; 
    while reduce(&(&t - Integer::from(1i32)), p) != Integer::from(0i32){
        t2 = reduce(&(&t*&t).complete(), p); 

        let mut i = 1; 
        while i < m{
            if reduce(&(&t2-Integer::from(1i32)), p) == Integer::from(0i32){
                break; 
            }
            t2 = reduce(&(&t2*&t2).complete(), p); 
            i+=1; 
        }
    
    
        let b = c.pow_mod(&Integer::from(2i32).pow(m-i-1), p).expect("Cannot modpow");
        r = reduce(&(r*&b),p); 
        c = reduce(&(&b*&b).complete(), p); 
        t = reduce(&(t*&c), p); 
        m = i; 
    }

    r 
}

pub fn legrende_symbol(a: &Integer, p: &Integer) -> Integer{
    let exponent = (p - Integer::from(1i32))/2;
    a.clone().pow_mod(&exponent, p).expect("Cannot modpow")
}