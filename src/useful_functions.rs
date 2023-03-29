
pub fn sieve_of_eratosthenes(n: usize) -> Vec<usize> {
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..=n {
        if is_prime[i] {
            let mut j = i * i;
            while j <= n {
                is_prime[j] = false;
                j += i;
            }
        }
    }
    let mut primes = vec![];
    for i in 0..is_prime.len() -1 {
        if is_prime[i] == true {
            primes.push(i);
        }
    }
    return primes;
    
}

pub fn is_prime(num) -> bool {
    
}