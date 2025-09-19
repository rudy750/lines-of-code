// Sample Rust file
fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    
    /* Check for divisors
       up to sqrt(n) */
    for i in 2..=(n as f64).sqrt() as u32 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let num = 17;
    println!("{} is prime: {}", num, is_prime(num));
}