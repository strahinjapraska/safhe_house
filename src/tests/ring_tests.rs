#[cfg(test)]
mod ring_tests{
    use safhe_house::math::ring::ring::QuotientRing;

    #[test]
    fn ring_mul(){
        let mut g:Vec<i128> = vec![1, 2, 3, 4];
        let mut h:Vec<i128> = vec![5, 6, 7, 8];  
    
        let ring = QuotientRing{
            n: 4, 
            p: 7681, 
        };         

        assert_eq!(ring.mul(&mut g, &mut h), vec![-56, -36, 2, 60]);
    } 

}