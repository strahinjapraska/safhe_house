#[cfg(test)]
mod fft_tests {

    use num_bigint::BigInt;
    use safhe_house::math::fft::{fft, ifft};

    #[test]
    fn fft_test() {
        let h = vec![BigInt::from(5), BigInt::from(6), BigInt::from(7), BigInt::from(8)];
        let n =4; 
        let p = &BigInt::from(7681);
        let w = &BigInt::from(3383);
        
        let fft_res = fft(&h, n, &w, p);
        assert_eq!(
            fft_res,
            vec![
                BigInt::from(26),
                BigInt::from(913),
                BigInt::from(7679),
                BigInt::from(6764)
            ]
        );
    }

    #[test]
    fn ifft_test() {
        let g = vec![BigInt::from(1), BigInt::from(2), BigInt::from(3), BigInt::from(4)];
        let n = 4;
        let w = BigInt::from(3383);
        let p = BigInt::from(7681);

        let fft_result = fft(&g, n, &w.clone(), &p.clone());
        assert_eq!(
            fft_result,
            vec![
                BigInt::from(10),
                BigInt::from(913),
                BigInt::from(7679),
                BigInt::from(6764)
            ]
        );

        assert_eq!(ifft(&fft_result, n, &w, &p), g);
    }
}
