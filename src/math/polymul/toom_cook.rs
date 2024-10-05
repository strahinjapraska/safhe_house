use rug::Integer;

use crate::math::ring::{add_no_mod, point_wise_mul_no_mod, scalar_div_no_mod, scalar_mul_no_mod, sub_no_mod};


pub fn toom_cook_4(a: &Vec<Integer> , b: &Vec<Integer>) -> Vec<Integer>{
    assert!(a.len() == b.len());  
    let n = a.len(); 
 
    assert!(n%4 == 0); 

    let split = n/4; 

    let (a_0, a_1, a_m1, a_2, a_m2, a_3, a_inf) = eval(a); 
    let (b_0, b_1, b_m1, b_2, b_m2, b_3, b_inf) = eval(b); 

    let c_0 = point_wise_mul_no_mod(&a_0, &b_0);
    let c_1 = point_wise_mul_no_mod(&a_1, &b_1);
    let c_m1 = point_wise_mul_no_mod(&a_m1, &b_m1);
    let c_2 = point_wise_mul_no_mod(&a_2, &b_2);
    let c_m2 = point_wise_mul_no_mod(&a_m2, &b_m2);
    let c_3 = point_wise_mul_no_mod(&a_3, &b_3);
    let c_inf = point_wise_mul_no_mod(&a_inf, &b_inf);


    let t0 = sub_no_mod(&sub_no_mod(&scalar_div_no_mod(&Integer::from(2), &add_no_mod(&c_1, &c_m1)), &c_0), &c_inf); 
    let t1 = scalar_div_no_mod(&Integer::from(8), &sub_no_mod(&sub_no_mod(&add_no_mod(&c_2, &c_m2), &scalar_mul_no_mod(&Integer::from(2),&c_0)),&scalar_mul_no_mod(&Integer::from(128),&c_inf)));
    let v4 = scalar_div_no_mod(&Integer::from(3), &sub_no_mod(&t1, &t0));
    let v2 = sub_no_mod(&t0, &v4);
    let t0 = scalar_div_no_mod(&Integer::from(2), &sub_no_mod(&c_1, &c_m1)); 
    let t1 = scalar_div_no_mod(&Integer::from(3),&sub_no_mod(&scalar_div_no_mod(&Integer::from(4), &sub_no_mod(&c_2, &c_m2)), &t0)); 
    let t2 = scalar_div_no_mod(&Integer::from(3), &sub_no_mod(&sub_no_mod(&sub_no_mod(&add_no_mod(&c_3, &c_0), &scalar_mul_no_mod(&Integer::from(9),&v2)),&scalar_mul_no_mod(&Integer::from(81),&v4)),&scalar_mul_no_mod(&Integer::from(729), &c_inf)));
    let t2 = sub_no_mod(&scalar_div_no_mod(&Integer::from(8), &sub_no_mod(&t2, &t0)), &t1);    
    let v5 = scalar_div_no_mod(&Integer::from(5), &t2);
    let v3 = sub_no_mod(&t1, &t2);
    let v1 = sub_no_mod(&sub_no_mod(&t0, &v3), &v5);

    let mut c = vec![Integer::from(0); 2*n -1]; 
    for i in 0..c_inf.len(){
        c[i] += c_0[i].clone(); 
        c[split+i] += v1[i].clone();
        c[2*split + i] += v2[i].clone(); 
        c[3*split + i] += v3[i].clone();  
        c[4*split + i] += v4[i].clone(); 
        c[5*split + i] += v5[i].clone();  
        c[6*split + i] += c_inf[i].clone();  
    }
    c 


}

fn split_to_4_parts(x: &Vec<Integer>, split: usize) -> (Vec<Integer> , Vec<Integer>, Vec<Integer>, Vec<Integer>){
    let a0 = x[0..split].to_vec(); 
    let a1 = x[split..2*split].to_vec(); 
    let a2 = x[2*split..3*split].to_vec(); 
    let a3 = x[3*split..].to_vec(); 

    return (a0, a1, a2, a3)
}

fn eval(x: &Vec<Integer>) -> (Vec<Integer> , Vec<Integer>, Vec<Integer>, Vec<Integer>, Vec<Integer> , Vec<Integer>, Vec<Integer>){

    let (x0, x1, x2, x3) = split_to_4_parts(x, x.len()/4);

    let x_0 = x0.clone();
    let x_1 =  add_no_mod(&add_no_mod(&x0, &x1), &add_no_mod(&x2, &x3)); 
    let x_m1 = add_no_mod(&sub_no_mod(&x0, &x1), &sub_no_mod(&x2, &x3)); 

    let t1 = scalar_mul_no_mod(&Integer::from(2), &x1);
    let t2 = scalar_mul_no_mod(&Integer::from(4), &x2);
    let t3 = scalar_mul_no_mod(&Integer::from(8), &x3);
    let x_2=  add_no_mod(&add_no_mod(&x0, &t1), &add_no_mod(&t2, &t3)); 
    let x_m2 = add_no_mod(&sub_no_mod(&x0, &t1), &sub_no_mod(&t2, &t3)); 

    let t1 = scalar_mul_no_mod(&Integer::from(3), &x1);
    let t2 = scalar_mul_no_mod(&Integer::from(9), &x2);
    let t3 = scalar_mul_no_mod(&Integer::from(27), &x3);
    let x_3=  add_no_mod(&add_no_mod(&x0, &t1), &add_no_mod(&t2, &t3)); 

    let x_inf = x3; 

    (x_0, x_1, x_m1, x_2, x_m2, x_3,  x_inf)
}