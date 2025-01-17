// method returning list of prime numbers up to n
fn prime(n: i32) -> Vec<i32> {
    let mut v = Vec::new();
    for i in 2..n {
        let mut is_prime = true;
        for j in 2..i {
            if i % j == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            v.push(i);
        }
    }
    v
}

// main program printing prime numbers up to 100
fn main() {
    for i in prime(100) {
        println!("{}", i);
    }
}
