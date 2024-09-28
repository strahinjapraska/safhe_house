use rug::{Complete, Integer};

pub fn karatsuba(p: &Vec<Integer>, q:&Vec<Integer>) -> Vec<Integer>{
    let n = p.len() - 1; 

    if n == 0{
        return vec![(&p[0]*&q[0]).complete()]; 
    }
    if n == 1{
        return vec![
            (&p[0]* &q[0]).complete(), 
            (&p[1]*&q[0]).complete() + (&p[0]*&q[1]).complete(), 
            (&p[1]*&q[1]).complete()
        ];
    }

    let m = (n as f64/2.0).ceil() as usize;

    let mut p_prime = vec![Integer::from(0i32); m+1];
    let mut q_prime = vec![Integer::from(0i32); m+1];

    for i in 0..=m-1{
        p_prime[i] = (&p[i] + &p[m+i]).into(); 
        q_prime[i] = (&q[i] + &q[m+i]).into(); 
    }

  
    if n > 2*m-1{
        p_prime[m] = p[n].clone(); 
        q_prime[m] = q[n].clone(); 
    }

    let mut r1 = karatsuba(&p[0..m].to_vec(), &q[0..m].to_vec()); 
    let mut r2 = karatsuba(&p[m..=n].to_vec(), &q[m..=n].to_vec());  
    let mut r3 = karatsuba(&p_prime, &q_prime);

    let resize_len = 2*m+1;
    r1.resize(resize_len, Integer::from(0i32));
    r2.resize(resize_len, Integer::from(0i32));
    r3.resize(resize_len, Integer::from(0i32));

    let mut r4 = vec![Integer::from(0i32); resize_len];
    for i in 0..=2*m{
        let temp = (&r3[i] - &r1[i]).complete();
        r4[i] = temp - &r2[i];
    }    
    

    let mut r = vec![Integer::from(0i32); 2*n+1]; 
    for i in 0..=2*n{ 
        r[i] += r1.get(i).unwrap_or(&Integer::from(0i32));
        if i>=m{
            r[i] += r4.get(i-m).unwrap_or(&Integer::from(0i32));
        }
        if i>=2*m{
            r[i] += r2.get(i-2*m).unwrap_or(&Integer::from(0i32));
        }
    }

    r


}