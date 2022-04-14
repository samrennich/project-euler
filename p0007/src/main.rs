/*
Problem 7: 10001st Prime

By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13,
we can see that the 6th prime is 13.
What is the 10001st prime number?

Answer: 104743
*/

const RANGE: i32 = 10001;

fn main() {
    println!("{}", primal::StreamingSieve::nth_prime(RANGE as usize));
}