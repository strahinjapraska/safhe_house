#[cfg(test)]
pub mod karatsuba_tests {
    use safhe_house::math::karatsuba::karatsuba;
    use rug::Integer;

    #[test]
    fn karatsuba_test() {
        let p = vec![Integer::from(1), Integer::from(3), Integer::from(1), Integer::from(2)];
        let q = vec![Integer::from(2), Integer::from(1), Integer::from(2), Integer::from(1)];

        assert_eq!(karatsuba(&p, &q), vec![
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
    fn karatsuba_test2() {
        let p = vec![Integer::from(1), Integer::from(2), Integer::from(3)];
        let q = vec![Integer::from(4), Integer::from(5), Integer::from(6)];

        assert_eq!(karatsuba(&p, &q), vec![
            Integer::from(4), 
            Integer::from(13), 
            Integer::from(28), 
            Integer::from(27), 
            Integer::from(18)
        ]);
    }

    #[test]
    fn karatsuba_test3() {
        let p = vec![Integer::from(4), Integer::from(3), Integer::from(2), Integer::from(1)];
        let q = vec![Integer::from(1), Integer::from(2), Integer::from(3), Integer::from(4)];

        assert_eq!(karatsuba(&p, &q), vec![
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
    fn karatsuba_test4() {
        let p = vec![Integer::from(5), Integer::from(4), Integer::from(1), Integer::from(3), Integer::from(2)];
        let q = vec![Integer::from(5), Integer::from(4), Integer::from(3), Integer::from(2), Integer::from(1)];

        assert_eq!(karatsuba(&p, &q), vec![
            Integer::from(25), 
            Integer::from(40), 
            Integer::from(36), 
            Integer::from(41), 
            Integer::from(38), 
            Integer::from(23), 
            Integer::from(13), 
            Integer::from(7), 
            Integer::from(2)
        ]);
    }

    #[test]
    fn karatsuba_test5() {
        let p = vec![Integer::from(3), Integer::from(2), Integer::from(1)];
        let q = vec![Integer::from(5), Integer::from(4), Integer::from(1)];

        assert_eq!(karatsuba(&p, &q), vec![
            Integer::from(15), 
            Integer::from(22), 
            Integer::from(16), 
            Integer::from(6), 
            Integer::from(1)
        ]);
    }

    #[test]
    fn karatsuba_test6() {
        let p = vec![Integer::from(1), Integer::from(1), Integer::from(1), Integer::from(1), Integer::from(1), Integer::from(1)];
        let q = vec![Integer::from(1), Integer::from(1), Integer::from(1), Integer::from(1), Integer::from(1), Integer::from(1)];

        assert_eq!(karatsuba(&p, &q), vec![
            Integer::from(1), 
            Integer::from(2), 
            Integer::from(3), 
            Integer::from(4), 
            Integer::from(5), 
            Integer::from(6), 
            Integer::from(5), 
            Integer::from(4), 
            Integer::from(3), 
            Integer::from(2), 
            Integer::from(1)
        ]);
    }

    #[test]
    fn karatsuba_test7() {
        let p = vec![Integer::from(4), Integer::from(1), Integer::from(3), Integer::from(2)];
        let q = vec![Integer::from(1), Integer::from(3), Integer::from(2), Integer::from(5)];

        assert_eq!(karatsuba(&p, &q), vec![
            Integer::from(4), 
            Integer::from(13), 
            Integer::from(14), 
            Integer::from(33), 
            Integer::from(17), 
            Integer::from(19), 
            Integer::from(10)
        ]);
    }

    #[test]
    fn karatsuba_test8() {
        let p = vec![Integer::from(2), Integer::from(3), Integer::from(1), Integer::from(0), Integer::from(0)];
        let q = vec![Integer::from(1), Integer::from(-1), Integer::from(0), Integer::from(1), Integer::from(0)];

        assert_eq!(karatsuba(&p, &q), vec![
            Integer::from(2), 
            Integer::from(1), 
            Integer::from(-2), 
            Integer::from(1), 
            Integer::from(3), 
            Integer::from(1), 
            Integer::from(0), 
            Integer::from(0), 
            Integer::from(0)
        ]);
    }
}
