// prime number sieve in Rust

fn main() {
    let mut sieve = vec![true; 1000];
    sieve[0] = false;
    sieve[1] = false;

    for i in 2..1000 {
        if sieve[i] {
            for j in (i * i..1000).step_by(i) {
                sieve[j] = false;
            }
        }
    }

    for i in 0..1000 {
        if sieve[i] {
            println!("{}", i);
        }
    }
}
