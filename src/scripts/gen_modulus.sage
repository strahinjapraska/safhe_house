desired_size = [
                (91,   2048), 
                (95,   4096), 
                (130,  4096), 
                (165,  4096),
                (171,  8192), 
                ]
for d in desired_size:
    q = next_prime(2^(d[0]))
    while True: 
        if q%(2*d[1]) == 1: 
            break 
        q = next_prime(q) 
    
    print("let n = "+ str(d[1]))
    print("let p = Integer::from_str_radix(\""+str(q)+"\", 10).expect(\"Cannot convert\")") 