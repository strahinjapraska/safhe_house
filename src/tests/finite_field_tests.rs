#[cfg(test)]
mod math_tests {
    use rug::Integer;
    use safhe_house::math::finite_field::{primitive_nth_root_of_unity, square_root_mod_p};

    #[test]
    fn tonelli_shanks() {
        assert_eq!(square_root_mod_p(&Integer::from(5), &Integer::from(41)), Integer::from(28)); 
    }


    #[test]
    fn primitive_nth_root_of_unity_test() {
        let w = primitive_nth_root_of_unity(&Integer::from(7681), 4);
        assert!(w == Integer::from(3383) || w == Integer::from(4298) || w == Integer::from(7680));
    }

}
