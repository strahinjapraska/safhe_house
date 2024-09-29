#[cfg(test)]
mod fft_tests {

    use rug::Integer;
    use safhe_house::math::ntt::{ntt, intt};

    #[test]
    fn ntt_test() {
        let h = vec![Integer::from(5), Integer::from(6), Integer::from(7), Integer::from(8)];
        let n =4; 
        let p = &Integer::from(7681);
        let w = &Integer::from(3383);
        
        let fft_res = ntt(&h, n, &w, p);
        assert_eq!(
            fft_res,
            vec![
                Integer::from(26),
                Integer::from(913),
                Integer::from(7679),
                Integer::from(6764)
            ]
        );
    }

    #[test]
    fn intt_test() {
        let g = vec![Integer::from(1), Integer::from(2), Integer::from(3), Integer::from(4)];
        let n = 4;
        let w = Integer::from(3383);
        let p = Integer::from(7681);
        let w_inv = w.clone().invert(&p).expect("Cannot get modular inverse");

        let fft_result = ntt(&g, n, &w, &p);
        assert_eq!(
            fft_result,
            vec![
                Integer::from(10),
                Integer::from(913),
                Integer::from(7679),
                Integer::from(6764)
            ]
        );

        assert_eq!(intt(&fft_result, n, &w_inv, &p), g);
    }
}
