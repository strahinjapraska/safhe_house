pub fn karatsuba(p: &Vec<i128>, q:&Vec<i128>) -> Vec<i128>{
    let n = p.len() - 1; 

    if n == 0{
        return vec![p[0]*q[0]]; 
    }
    if n == 1{
        return vec![p[0]* q[0], p[1]*q[0] + p[0]*q[1], p[1]*q[1]];
    }

    let m = (n as f64/2.0).ceil() as usize;

    let mut p_prime = vec![0; m+1];
    let mut q_prime = vec![0; m+1];

    for i in 0..=m-1{
        p_prime[i] = p[i] + p[m+i]; 
        q_prime[i] = q[i] + q[m+i]; 
    }

  
    if n > 2*m-1{
        p_prime[m] = p[n]; 
        q_prime[m] = q[n]; 
    }

    let mut r1 = karatsuba(&p[0..=m-1].to_vec(), &q[0..=m-1].to_vec()); 
    let mut r2 = karatsuba(&p[m..=n].to_vec(), &q[m..=n].to_vec());  
    let mut r3 = karatsuba(&p_prime, &q_prime);

    r1.resize(2*m+1, 0);
    r2.resize(2*m+1, 0);
    r3.resize(2*m+1, 0);

    let mut r4 = vec![0i128; 2*m+1];
    for i in 0..=2*m{
        r4[i] = r3[i] - r1[i] - r2[i];
    }    
    

    let mut r = vec![0; 2*n+2]; 
    for i in 0..=2*n{ 
        r[i] = *r1.get(i).unwrap_or(&0);
        if i>=m{
            r[i] += r4.get(i-m).unwrap_or(&0);
        }
        if i>=2*m{
            r[i] += r2.get(i-2*m).unwrap_or(&0);
        }
    }

    r


}