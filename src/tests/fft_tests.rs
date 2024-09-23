#[cfg(test)]
mod fft_tests{

    use safhe_house::math::fft::{ifft, fft};

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