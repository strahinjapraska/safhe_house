#[cfg(test)]
mod ring_tests {
    use num::BigInt;
    use safhe_house::math::ring::{ring::mul, ring_no_mod::mul_no_mod};

    #[test]
    fn ring_mul_test() {
        let mut g = vec![BigInt::from(1), BigInt::from(2), BigInt::from(3), BigInt::from(4)];
        let mut h = vec![BigInt::from(5), BigInt::from(6), BigInt::from(7), BigInt::from(8)];
        let p = BigInt::from(7681);

        assert_eq!(
            mul(&mut g, &mut h, &p),
            vec![
                BigInt::from(-56), 
                BigInt::from(-36), 
                BigInt::from(2), 
                BigInt::from(60)
            ]
        );
    }

    #[test]
    fn ring_mul_no_mod_test() {
        let p = vec![BigInt::from(1), BigInt::from(3), BigInt::from(1), BigInt::from(2)];
        let q = vec![BigInt::from(2), BigInt::from(1), BigInt::from(2), BigInt::from(1)];

        assert_eq!(
            vec![
                BigInt::from(-5), 
                BigInt::from(2), 
                BigInt::from(5), 
                BigInt::from(12)
            ],
            mul_no_mod(&p, &q, 4)
        );
    }

    #[test]
    fn ring_mul_no_mod_test2() {
        let p = vec![BigInt::from(1), BigInt::from(2), BigInt::from(3), BigInt::from(4)];
        let q = vec![BigInt::from(4), BigInt::from(3), BigInt::from(2), BigInt::from(1)];

        assert_eq!(
            vec![
                BigInt::from(-16), 
                BigInt::from(0), 
                BigInt::from(16), 
                BigInt::from(30)
            ],
            mul_no_mod(&p, &q, 4)
        );
    }

    #[test]
    fn ring_mul_no_mod_test3() {
        let p = vec![BigInt::from(2), BigInt::from(3), BigInt::from(1), BigInt::from(0), BigInt::from(0)];
        let q = vec![BigInt::from(1), BigInt::from(-1), BigInt::from(0), BigInt::from(1), BigInt::from(0)];

        assert_eq!(
            vec![
                BigInt::from(1), 
                BigInt::from(1), 
                BigInt::from(-2), 
                BigInt::from(1), 
                BigInt::from(3)
            ],
            mul_no_mod(&p, &q, 5)
        );
    }
}
