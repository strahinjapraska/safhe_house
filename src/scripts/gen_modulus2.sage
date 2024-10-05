desired_sizes = [
    (19,  512),
    (38,  1024),
    (64,  2048),
    (89, ,
    (94, 131072), 
    (120,), 
    
]
  
for d in desired_sizes: 
    bit_size = d[0] 
    n = 2 * d[1]

    k = ceil(2^bit_size / n)

    while True:
        q = k * n + 1
        if is_prime(q):
            break
        k += 1

    assert(ceil(log(q,2).n()) == d[0]) 
    assert(q%n == 1) 

    print("let n = "+ str(d[1])+";")
    print("let p = Integer::from_str_radix(\""+str(q)+"\", 10).expect(\"Cannot convert\");") 