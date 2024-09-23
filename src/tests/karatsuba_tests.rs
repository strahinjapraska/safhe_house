#[cfg(test)]
pub mod karatsuba_tests{
    use safhe_house::math::karatsuba::karatsuba;

    #[test]
    fn karatsuba_test(){
        
        let p = vec![1, 3, 1, 2]; 
        let q = vec![2, 1, 2, 1];

        assert_eq!(karatsuba(&p, &q), vec![2, 7, 7, 12, 7, 5, 2, 0]);
    }

}