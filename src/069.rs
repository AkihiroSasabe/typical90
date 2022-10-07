use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use proconio::marker::Chars;
fn main() {
    input! {
        n: usize,
        k: usize
    }

    let modulo = 1_000_000_000 + 7;

    // k * k-1 * k-2 * k-2 * k-2 * .... * k-2 が求めたい答え。 
    if n == 1 {
        println!("{}", k % modulo);
        return;
    }
    else if n == 2 {
        println!("{}", (k * (k-1)) % modulo);
        return;
    }
    else {
        if k == 1{
            println!("0");
            return;
        }
        // answer = (k-2).pow(n as u32 - 2); // Overflowするし、O(n)の計算量でTLEする。
        let answer = get_remainder_for_exp_func(k-2, n-2, modulo);
        println!("{}", k * (k-1) % modulo * answer % modulo);
    }

    // let answer = ten_to_base(11, 8);
    // println!("{}", answer); // 13
    // let answer = ten_to_base(2021, 9);
    // println!("{}", answer); // 2685
}

// base^(x) % mod を繰り返し二乗法により、O(log2(x))の計算量で求める　(O(x)だとTLE)
fn get_remainder_for_exp_func(mut base: usize, mut exponent: usize, modulo: usize) -> usize {
    // Example
    // 3^14 mod 100  を求める                                base = 3, exp = 14
    //     3^14  = (3^2)^7                         (mod 100) base = 3^2 = 9, exp = 7
    // <=> 3^14 = (3^2 % 100)^7                    (mod 100)    exp % 2 == 1 => remainder * base
    // <=> 3^14 = (3^4)^3 * (3^2)^1                (mod 100) base = 3^4 = 81, exp = 3
    // <=> 3^14 = (3^4 % 100)^3 * (3^2)^1          (mod 100) 
    // <=> 3^14 = (3^8)^1 * (3^4)^1 * (3^2)^1      (mod 100) base = 3^8 = 6561 = 61, exp = 1
    // <=> 3^14 = (3^8 % 100)^1 * (3^4)^1 * (3^2)^1(mod 100)
    // <=> 3^14 = (6561 % 100)^1 * (81)^1 * (9)^1  (mod 100)
    // <=> 3^14 = (61)^1 * (81)^1 * (9)^1          (mod 100)
    // <=> 3^14 = 44,469 = 69                      (mod 100)
    let mut remainder = 1;
    while exponent != 0 {
        if exponent % 2 == 1 {
            remainder = (remainder * base) % modulo;
        }
        base = (base * base) % modulo;
        exponent /= 2;
    }
    return remainder;
}


// 10進法のxをn(base)進法に変換
fn ten_to_base(mut x: usize, base: usize) -> usize {
    // ==== How to use ====
    // let answer = ten_to_base(11, 8);
    // println!("{}", answer); // 13
    // let answer = ten_to_base(2021, 9);
    // println!("{}", answer); // 2685   
    // ==== Theory ====
    // The number x can be expressed in n-decimal notation as follows:
    // (where 0 <= ai <= n-1)
    // x   = a0 * (n^0) + a1 * (n^1) * a2 * (n^2) + ... + amax * (n^max)
    // ==== Example ====
    // 11  = 2*(8**0) + 3*(8**1) 
    // a0 = x % 8
    // a1 = (x / 8) % 8
    // a2 = (x / 8^2) % 8 but (x / 8^2) == 0 => max = 1
    // ... 
    let mut answer = 0;
    let mut digit: u32 = 0;
    while x != 0 {
        let amari = x % base;
        answer += amari * 10_usize.pow(digit);
        x /= base;
        digit += 1;
    }
    return answer;
}