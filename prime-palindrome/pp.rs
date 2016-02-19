fn main() {
    let mut v:Vec<i32> = vec![2];
    for _ in 0..1000 {
        let prime = primes(&v);
        if prime == -1 {
            break;
        }
        v.push(prime);
    }
    'top:for prime in v.iter().rev() {
		let prime_s = format!("{}", &prime).into_bytes();
		let mut prime_r = format!("{}", &prime).into_bytes();
		prime_r.reverse();
		if prime_s == prime_r {
			println!("{}", prime);
			break 'top;
		}
    }
}

fn primes(v:&Vec<i32>) -> i32 {
    if v[v.len()-1] < 1000 {
        let mut i = v[v.len()-1];
        'inf:loop {
            i+=1;
            let mut prime:bool = true;
            'this:for x in v.iter() {
                if i%x==0 {
                    prime = false;
                    break 'this;
                }
            }
            if prime==true {
                return i;
            }
        }
    }else{
        return -1; // stop making primes after 1000
    }
}
