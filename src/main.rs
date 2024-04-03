#![allow(dead_code)]

use std::str::FromStr;
use crate::useful_functions::is_prime;

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
    let mut i = 1;
    let mut n = 600851475143;
    while i * i <= n {
        if n % i == 0 {
            n = n / i
        }
        i += 1;
    }
    return n;
}

fn euler_4() -> isize {
    //Find the largest palindrome made from the product of two 3-digit numbers.
    let mut palindrome = 0;
    for i in 100..999 {
        for j in 100..999 {
            let product_str: String = (i * j).to_string();
            if product_str == product_str.chars().rev().collect::<String>() && (i * j) > palindrome {
                palindrome = i * j;
            }
        }
    }
    palindrome
}

fn euler_5() -> i32 {
    //2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
    //What is the smallest positive number that is evenly divisible by all the numbers from 1 to 20?
    //let all_primes = 2..20;
    
    // 232792560
    let mut found = false;
    let mut num = 20;

    while found == false {
        if num % 100000000 == 0 {
            println!("{}", num);
        }
        found = true;
        for i in 2..20 {
            if num % i != 0 {
                found = false;
                break
            }
        }
        num += 1
    }
    num - 1
}

fn get_prime_permutations_of_wildcards(to_check: String, mut prime_perms: Vec<String>) -> Vec<String> {
    // to_check is an integer containing wildcards, such as "5*3"
    return if to_check.contains("*") {
        for replace_char in 0..9 {
            let replaced_str = to_check.replace("*", &replace_char.to_string());
            prime_perms = get_prime_permutations_of_wildcards(replaced_str, prime_perms);
        }
        prime_perms
    } else {
        if is_prime(&u64::from_str(&to_check).unwrap()) {
            prime_perms.push(to_check);
        } 
        prime_perms
    }
}


fn euler_51() -> Vec<String> {
    
    // By replacing the 1st digit of the 2-digit number *3, 
    // it turns out that six of the nine possible values: 13, 23, 43, 53, 73, and 83, 
    // are all prime.
    // By replacing the 3rd and 4th digits of 56**3 with the same digit, 
    // this 5-digit number is the first example having seven primes among the ten generated 
    // numbers, yielding the family: 56003, 56113, 56333, 56443, 56663, 56773, and 56993. 
    // Consequently, 56003, being the first member of this family, is the smallest prime 
    // with this property.
    // Find the smallest prime which, by replacing part of the number 
    // (not necessarily adjacent digits) with the same digit, 
    // is part of an eight prime value family.
    
    for i in 1.. {
        if is_prime(&i) {
            
        }
    }
    
    let wildcard_str: &str = "56**3";
    let perms = get_prime_permutations_of_wildcards(wildcard_str.parse().unwrap(), 
                                                    Vec::new());
    return perms;
}
    
fn main() {
    println!("{:?}", euler_51());
}