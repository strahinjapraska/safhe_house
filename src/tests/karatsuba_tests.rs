#[cfg(test)]
pub mod karatsuba_tests {
    use safhe_house::math::karatsuba::karatsuba;
    use num_bigint::BigInt;

    #[test]
    fn karatsuba_test() {
        let p = vec![BigInt::from(1), BigInt::from(3), BigInt::from(1), BigInt::from(2)];
        let q = vec![BigInt::from(2), BigInt::from(1), BigInt::from(2), BigInt::from(1)];

        assert_eq!(karatsuba(&p, &q), vec![
            BigInt::from(2), 
            BigInt::from(7), 
            BigInt::from(7), 
            BigInt::from(12), 
            BigInt::from(7), 
            BigInt::from(5), 
            BigInt::from(2)
        ]);
    }

    #[test]
    fn karatsuba_test2() {
        let p = vec![BigInt::from(1), BigInt::from(2), BigInt::from(3)];
        let q = vec![BigInt::from(4), BigInt::from(5), BigInt::from(6)];

        assert_eq!(karatsuba(&p, &q), vec![
            BigInt::from(4), 
            BigInt::from(13), 
            BigInt::from(28), 
            BigInt::from(27), 
            BigInt::from(18)
        ]);
    }

    #[test]
    fn karatsuba_test3() {
        let p = vec![BigInt::from(4), BigInt::from(3), BigInt::from(2), BigInt::from(1)];
        let q = vec![BigInt::from(1), BigInt::from(2), BigInt::from(3), BigInt::from(4)];

        assert_eq!(karatsuba(&p, &q), vec![
            BigInt::from(4), 
            BigInt::from(11), 
            BigInt::from(20), 
            BigInt::from(30), 
            BigInt::from(20), 
            BigInt::from(11), 
            BigInt::from(4)
        ]);
    }

    #[test]
    fn karatsuba_test4() {
        let p = vec![BigInt::from(5), BigInt::from(4), BigInt::from(1), BigInt::from(3), BigInt::from(2)];
        let q = vec![BigInt::from(5), BigInt::from(4), BigInt::from(3), BigInt::from(2), BigInt::from(1)];

        assert_eq!(karatsuba(&p, &q), vec![
            BigInt::from(25), 
            BigInt::from(40), 
            BigInt::from(36), 
            BigInt::from(41), 
            BigInt::from(38), 
            BigInt::from(23), 
            BigInt::from(13), 
            BigInt::from(7), 
            BigInt::from(2)
        ]);
    }

    #[test]
    fn karatsuba_test5() {
        let p = vec![BigInt::from(3), BigInt::from(2), BigInt::from(1)];
        let q = vec![BigInt::from(5), BigInt::from(4), BigInt::from(1)];

        assert_eq!(karatsuba(&p, &q), vec![
            BigInt::from(15), 
            BigInt::from(22), 
            BigInt::from(16), 
            BigInt::from(6), 
            BigInt::from(1)
        ]);
    }

    #[test]
    fn karatsuba_test6() {
        let p = vec![BigInt::from(1), BigInt::from(1), BigInt::from(1), BigInt::from(1), BigInt::from(1), BigInt::from(1)];
        let q = vec![BigInt::from(1), BigInt::from(1), BigInt::from(1), BigInt::from(1), BigInt::from(1), BigInt::from(1)];

        assert_eq!(karatsuba(&p, &q), vec![
            BigInt::from(1), 
            BigInt::from(2), 
            BigInt::from(3), 
            BigInt::from(4), 
            BigInt::from(5), 
            BigInt::from(6), 
            BigInt::from(5), 
            BigInt::from(4), 
            BigInt::from(3), 
            BigInt::from(2), 
            BigInt::from(1)
        ]);
    }

    #[test]
    fn karatsuba_test7() {
        let p = vec![BigInt::from(4), BigInt::from(1), BigInt::from(3), BigInt::from(2)];
        let q = vec![BigInt::from(1), BigInt::from(3), BigInt::from(2), BigInt::from(5)];

        assert_eq!(karatsuba(&p, &q), vec![
            BigInt::from(4), 
            BigInt::from(13), 
            BigInt::from(14), 
            BigInt::from(33), 
            BigInt::from(17), 
            BigInt::from(19), 
            BigInt::from(10)
        ]);
    }

    #[test]
    fn karatsuba_test8() {
        let p = vec![BigInt::from(2), BigInt::from(3), BigInt::from(1), BigInt::from(0), BigInt::from(0)];
        let q = vec![BigInt::from(1), BigInt::from(-1), BigInt::from(0), BigInt::from(1), BigInt::from(0)];

        assert_eq!(karatsuba(&p, &q), vec![
            BigInt::from(2), 
            BigInt::from(1), 
            BigInt::from(-2), 
            BigInt::from(1), 
            BigInt::from(3), 
            BigInt::from(1), 
            BigInt::from(0), 
            BigInt::from(0), 
            BigInt::from(0)
        ]);
    }
}
