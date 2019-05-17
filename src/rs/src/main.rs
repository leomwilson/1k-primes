fn is_prime(n: u16) -> bool {
    for i in (2..n) {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}

fn main() {
    let mut p: u16 = 2;
    let mut i: u16 = 0;
    while i < 1000 {
        if is_prime(p) {
            println!("{}", p);
            i += 1;
        }
        p += 1;
    }
}
