#[cfg(test)]
pub mod toom_cook_tests{
    use rug::Integer;
    use safhe_house::math::polymul::toom_cook::toom_cook_4;


    
    #[test]
    pub fn toom_cook_test1(){
        let p = vec![Integer::from(1), Integer::from(3), Integer::from(1), Integer::from(2)];
        let q = vec![Integer::from(2), Integer::from(1), Integer::from(2), Integer::from(1)];


        assert_eq!(toom_cook_4(&p, &q), vec![
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
    fn toom_cook_test2() {
        let p = vec![Integer::from(4), Integer::from(3), Integer::from(2), Integer::from(1)];
        let q = vec![Integer::from(1), Integer::from(2), Integer::from(3), Integer::from(4)];

        assert_eq!(toom_cook_4(&p, &q), vec![
            Integer::from(4), 
            Integer::from(11), 
            Integer::from(20), 
            Integer::from(30), 
            Integer::from(20), 
            Integer::from(11), 
            Integer::from(4)
        ]);
    }
}