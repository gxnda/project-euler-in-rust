
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

// def is_prime(num: int) -> bool:
//     num = abs(num)
//     factor = 2
//     if num < factor:
//         return False
//     while factor**2 < num + 1:
//         if num % factor == 0:
//             return False
//         factor += 1
//     return True
pub fn is_prime(num: &u64) -> bool {
    let mut factor: u64 = 2;
    return if num < &factor {
        false
    } else {
        while u64::pow(factor, 2) < (num + 1) {
            if num % factor == 0 {
                return false; // not prime by divisibility
            }
            factor += 1;
        }
        true
    }
}
