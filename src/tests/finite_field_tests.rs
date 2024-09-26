#[cfg(test)]
mod math_tests {
    use num_bigint::BigInt;
    use safhe_house::math::finite_field::{primitive_nth_root_of_unity, square_root_mod_p};

    #[test]
    fn tonelli_shanks() {
        assert_eq!(square_root_mod_p(&BigInt::from(5), &BigInt::from(41)), BigInt::from(28)); 
    }


    #[test]
    fn primitive_nth_root_of_unity_test() {
        let w = primitive_nth_root_of_unity(&BigInt::from(7681), 4);
        assert!(w == BigInt::from(3383) || w == BigInt::from(4298) || w == BigInt::from(7680));
    }

}
