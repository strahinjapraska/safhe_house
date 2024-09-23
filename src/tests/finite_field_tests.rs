#[cfg(test)]
mod math_tests{
    use safhe_house::math::finite_field::{inv_mod, mod_pow, pow, primitive_nth_root_of_unity, square_root_mod_p};


   
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
    fn pow_test(){
        assert_eq!(pow(9, 3), 9*9*9);
    }

}