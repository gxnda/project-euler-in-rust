#![allow(dead_code)]
mod useful_functions;

fn euler_1() -> usize {
    //Multiples of 3 or 5
    //Problem 1
    //If we list all the natural numbers below 10 that are multiples of 3 or 5, 
    //we get 3, 5, 6 and 9. The sum of these multiples is 23.
    //Find the sum of all the multiples of 3 or 5 below 1000.
    let limit = 1000;
    let mut sum = 0;
    for num in 0..limit {
        if (num % 3 == 0) || (num % 5 == 0) {
            sum += num;
        }
    }
    return sum
}

fn euler_2() -> usize {
    //By considering the terms in the Fibonacci sequence whose values do not exceed four million, find the sum of the even-valued terms.
    let limit = 4000000;
    let mut a = 1; let mut b = 1;
    let mut sum = 0;
    while b < limit {
        if b % 2 == 0  {
            sum += b
        }
        let temp = a + b;
        a = b;
        b = temp;
    }
    sum
}

fn euler_3() -> usize {
    //The prime factors of 13195 are 5, 7, 13 and 29.
    //What is the largest prime factor of the number 600851475143 ?
    let num: usize = 600851475143; 
    let primes = useful_functions::sieve_of_eratosthenes(num);
    let mut i = 2;
    let mut largest = 0;
    while i*i <= num {
        if primes.contains(&i) == true {
            if num % i == 0 {
                if i > largest {
                    largest = i;
                }
            }
        }
        i += 1
    }
    largest
}

fn main() {
    println!("{:?}", euler_3());
}