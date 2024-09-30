#[cfg(test)]
pub mod fft_tests{
    use rug::Integer;
    use safhe_house::math::fft::fft_mul;


    
    #[test]
    pub fn fft_test1(){
        let p = vec![Integer::from(1), Integer::from(3), Integer::from(1), Integer::from(2)];
        let q = vec![Integer::from(2), Integer::from(1), Integer::from(2), Integer::from(1)];


        assert_eq!(fft_mul(&p, &q, 32), vec![
            Integer::from(2), 
            Integer::from(7), 
            Integer::from(7), 
            Integer::from(12), 
            Integer::from(7), 
            Integer::from(5), 
            Integer::from(2)
        ]);

    }


    #[test]
    fn fft_test2() {
        let p = vec![Integer::from(4), Integer::from(3), Integer::from(2), Integer::from(1)];
        let q = vec![Integer::from(1), Integer::from(2), Integer::from(3), Integer::from(4)];

        assert_eq!(fft_mul(&p, &q, 32), vec![
            Integer::from(4), 
            Integer::from(11), 
            Integer::from(20), 
            Integer::from(30), 
            Integer::from(20), 
            Integer::from(11), 
            Integer::from(4)
        ]);
    }


    #[test]
    fn fft_test3() {
        let p = vec![Integer::from(4), Integer::from(1), Integer::from(3), Integer::from(2)];
        let q = vec![Integer::from(1), Integer::from(3), Integer::from(2), Integer::from(5)];

        assert_eq!(fft_mul(&p, &q, 32), vec![
            Integer::from(4), 
            Integer::from(13), 
            Integer::from(14), 
            Integer::from(33), 
            Integer::from(17), 
            Integer::from(19), 
            Integer::from(10)
        ]);
    }
}