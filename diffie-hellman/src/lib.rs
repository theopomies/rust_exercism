use rand::prelude::*;

pub fn private_key(p: u64) -> u64 {
    thread_rng().gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modular_pow(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modular_pow(b_pub, a, p)
}

fn modular_pow(x: u64, mut y: u64, p: u64) -> u64 {
    let mut res = 1;
    let mut x = x % p;

    if x == 0 {
        return 0;
    }

    while y > 0 {
        if y & 1 != 0 {
            res = (res * x) % p;
        }
        y >>= 1;
        x = x.pow(2) % p;
    }

    return res;
}
