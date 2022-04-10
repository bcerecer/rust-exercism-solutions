pub fn nth(n: u32) -> u32 {
    // Used for memoization
    let mut primes: Vec<u32> = vec![];
    (2..)
        .filter(|candidate: &u32| {
            if is_prime(*candidate, primes.to_vec()) {
                primes.push(*candidate);
                true
            } else {
                false
            }
        })
        .nth(n as usize)
        .unwrap()
}

fn is_prime(candidate: u32, primes: Vec<u32>) -> bool {
    !primes.iter().any(|divisor| candidate % divisor == 0)
}
