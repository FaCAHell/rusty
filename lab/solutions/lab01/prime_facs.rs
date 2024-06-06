fn factorize(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut divisor = 2;

    while n > 1 {
        while n % divisor == 0 {
            factors.push(divisor);
            n = n / divisor;
        }
        divisor += 1;
    }

    factors
}

fn main() {
    println!("256 is {}", 256u32.is_power_of_two());
    let number = 120;
    let factors = factorize(number);
    println!("Prime factors of {}: {:?}", number, factors);
}