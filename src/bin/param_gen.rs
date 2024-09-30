use safhe_house::math::finite_field::{primitive_nth_root_of_unity, square_root_mod_p};
use safhe_house::schemes::bfv::params::{get_params, PARAMS};

fn main(){
    let params = PARAMS::get_all();
    for p in params{
        let param = get_params(p.clone());
        let w = primitive_nth_root_of_unity(&param.p(), param.n());
        let w_inv = w.clone().invert(&param.p()).expect("No modular inverse");
        let phi = square_root_mod_p(&w, &param.p());
        let phi_inv = phi.clone().invert(&param.p()).expect("No modular inverse");
        println!("------------------------------{:?}------------------------------", p);
        println!(
            "w: Integer::from_str_radix(\"{}\", 10).expect(\"Cannot convert\"),\n\
             w_inv: Integer::from_str_radix(\"{}\", 10).expect(\"Cannot convert\"),\n\
             phi: Integer::from_str_radix(\"{}\", 10).expect(\"Cannot convert\"),\n\
             phi_inv: Integer::from_str_radix(\"{}\", 10).expect(\"Cannot convert\"),"
            ,w, w_inv, phi, phi_inv);
        println!("-----------------------------------------------------------------------");
    }
}