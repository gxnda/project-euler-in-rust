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
    //What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
    //let all_primes = 2..20;
    let mut found = false;
    let mut num = 20;

    while found == false {
        println!("{}", num);
        found = true;
        for i in 2..20 {
            if num % i != 0 {
                found = false
            }
        }
        num += 1
    }
    num - 1
}
    
fn main() {
    println!("{:?}", euler_5());
}