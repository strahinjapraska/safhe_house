#[cfg(test)]
mod ring_tests{
    use safhe_house::math::ring::ring::QuotientRing;

    #[test]
    fn ring_mul_test(){
        let mut g:Vec<i128> = vec![1, 2, 3, 4];
        let mut h:Vec<i128> = vec![5, 6, 7, 8];  
    
        let ring = QuotientRing{
            n: 4, 
            p: 7681, 
        };         

        assert_eq!(ring.mul(&mut g, &mut h), vec![-56, -36, 2, 60]);
    } 

    #[test]
    fn ring_mul_no_mod_test(){
        let p = vec![1, 3, 1, 2]; 
        let q = vec![2, 1, 2, 1];

        let ring = QuotientRing{
            n: 4, 
            p: 7681, 
        };  

        assert_eq!(vec![-5, 2, 5, 12], ring.mul_no_mod(&p, &q));

    }



}