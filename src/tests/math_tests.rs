#[cfg(test)]
mod math_tests{
    use safhe_house::math::{fft::{fft, ifft}, finite_field::{inv_mod, mod_pow, primitive_nth_root_of_unity, square_root_mod_p}, ring::QuotientRing};


   
    #[test]
    fn tonelli_shanks(){
        assert_eq!(square_root_mod_p(5, 41), 28); 
    }

    #[test]
    fn mod_pow_test(){
        assert_eq!(mod_pow(2, 5, 13),6);
    }

    #[test]
    fn primitive_nth_root_of_unity_test(){
        let w = primitive_nth_root_of_unity(7681, 4);
        assert!(w == 3383 || w == 4298 || w == 7680)
    }

    #[test]
    fn inv_mod_test(){
       assert_eq!(inv_mod(3, 11),4)
    }

    #[test]
    fn cyclotomic_ring_mul_mod_p_test(){
        let mut g:Vec<i64> = vec![1, 2, 3, 4];
        let mut h:Vec<i64> = vec![5, 6, 7, 8];  

        let ring = QuotientRing{
            n: 4, 
            p: 7681, 
        };         

        assert_eq!(ring.mul(&mut g, &mut h), vec![-56, -36, 2, 60]);
    } 


    #[test]
    fn fft_test(){
        let h = vec![5,6,7,8];
        let fft_res = fft(&h, 4, 3383, 7681);
        assert_eq!(fft_res, vec![26, 913, 7679, 6764]);
    }


    #[test]
    fn ifft_test(){
        let g = vec![1,2,3,4];
        let n = 4; 
        let w = 3383; 
        let p = 7681;

        let fft_result =  fft(&g, n,w, p); 
        assert_eq!(fft_result, vec![10,913,7679, 6764]);

        assert_eq!(ifft(&fft_result, n, w, p), g);      
      
    }
}