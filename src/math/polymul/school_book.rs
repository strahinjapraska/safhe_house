use rug::Integer;

pub fn schoolbook(a: &Vec<Integer> , b: &Vec<Integer>) -> Vec<Integer>{
    let n = a.len(); 
    let m = b.len(); 

    let mut c = vec![Integer::from(0); n + m - 1]; 

    for i in 0..n{
        for j in 0..m{
            c[i+j] += &a[i]*&b[j]; 
        }
    }
    c
}