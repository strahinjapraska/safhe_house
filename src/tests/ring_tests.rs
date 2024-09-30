#[cfg(test)]
mod ring_tests {
    use rug::Integer;
    use safhe_house::math::finite_field::{primitive_nth_root_of_unity, square_root_mod_p};
    use safhe_house::math::ring::PolyMulAlgorithm;
    use safhe_house::math::ring::{mul, mul_no_mod};

    #[test]
    fn ring_mul_test() {
        let mut g = vec![Integer::from(1), Integer::from(2), Integer::from(3), Integer::from(4)];
        let mut h = vec![Integer::from(5), Integer::from(6), Integer::from(7), Integer::from(8)];
        let p = Integer::from(7681);
        let n = 4;
        let w = primitive_nth_root_of_unity(&p, n);
        let w_inv = w.clone().invert(&p).expect("Cannot get an inverse");
        let phi = square_root_mod_p(&w, &p);
        let phi_inv = phi.clone().invert(&p).expect("Cannot get an inverse");

        assert_eq!(
            mul(&mut g, &mut h, &p, &w, &w_inv, &phi, &phi_inv),
            vec![
                Integer::from(-56), 
                Integer::from(-36), 
                Integer::from(2), 
                Integer::from(60)
            ]
        );
    }

    #[test]
    fn ring_mul_no_mod_test() {
        let p = vec![Integer::from(1), Integer::from(3), Integer::from(1), Integer::from(2)];
        let q = vec![Integer::from(2), Integer::from(1), Integer::from(2), Integer::from(1)];

        assert_eq!(
            vec![
                Integer::from(-5), 
                Integer::from(2), 
                Integer::from(5), 
                Integer::from(12)
            ],
            mul_no_mod(&p, &q, 4, PolyMulAlgorithm::Fft, 32)
        );
    }

    #[test]
    fn ring_mul_no_mod_test2() {
        let p = vec![Integer::from(1), Integer::from(2), Integer::from(3), Integer::from(4)];
        let q = vec![Integer::from(4), Integer::from(3), Integer::from(2), Integer::from(1)];

        assert_eq!(
            vec![
                Integer::from(-16), 
                Integer::from(0), 
                Integer::from(16), 
                Integer::from(30)
            ],
            mul_no_mod(&p, &q, 4, PolyMulAlgorithm::Fft, 32)
        );
    }

    #[test]
    fn ring_mul_no_mod_test3() {
        let p = vec![Integer::from(2), Integer::from(3), Integer::from(1), Integer::from(0), Integer::from(0)];
        let q = vec![Integer::from(1), Integer::from(-1), Integer::from(0), Integer::from(1), Integer::from(0)];

        assert_eq!(
            vec![
                Integer::from(1), 
                Integer::from(1), 
                Integer::from(-2), 
                Integer::from(1), 
                Integer::from(3)
            ],
            mul_no_mod(&p, &q, 5, PolyMulAlgorithm::Karatsuba, 0)
        );
    }
}
