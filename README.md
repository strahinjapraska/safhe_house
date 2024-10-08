FHE in Rust 

Usage
```rust
let m1 = vec![Integer::from(-8); 1024];
let m2 = vec![Integer::from(411); 1024];

let clear_res = vec![Integer::from(403); 1024];

let (sk, pk) = BFV::gen_keys(RlweParams1); 

let c1 = pk.encrypt(&Plaintext{message: m1.clone()}); 
let c2 = pk.encrypt(&Plaintext { message: m2.clone()}); 

let encrypted_res = pk.add(&c1, &c2);
let decrypted_res = sk.decrypt(&encrypted_res); 

assert_eq!(clear_res, decrypted_res.message);
```

Python `maturin` is needed to run python scripts for discrete gaussian plots.

Run `maturin build`.

Then you will get the `.whl` generated just call `pip install` with it's path.

You can then import discrete gaussian from `safhe_house` in python. 
