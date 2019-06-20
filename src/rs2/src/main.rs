fn is_prime(i: u16, primes: &Vec<u16>) -> bool {
    for prime in primes {
        if i % prime == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let mut primes: Vec<u16> = Vec::new();
    let mut i: u16 = 2;
    let mut pcount: u16 = 0;
    while pcount < 1000 {
        if is_prime(i, &primes) {
            pcount += 1;
            primes.push(i);
            println!("{}", i);
        }
        i += if i == 2 { 1 } else { 2 }; // next odd number
    }
}
