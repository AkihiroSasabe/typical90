use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use proconio::marker::Chars;
fn main() {
    input! {
        n: usize
    }
    let MODULO = 1_000_000_007;

    // n!を格納された配列
    let mut memo_factorial = vec![1;n+1];
    for i in 1..(n+1) {
        memo_factorial[i] = (i * memo_factorial[i-1]) % MODULO;
    }

    for k in 1..(n+1) {
        let mut count = 0;
        // 選ぶボールの数
        for x in 1..((n-1)/k+2) {
            count = (count + combination(n - (x-1) * (k-1), x, MODULO, &memo_factorial)) % MODULO;
        }
        println!("{}", count % MODULO);
    }
    


    // for k in 1..(n+1) {
    //     let mut count = 0;
    //     if k== 1 {
    //         println!("{}", 2_u32.pow(n as u32) - 1);
    //     }
    //     else {
    //         for x in 0..n {
    //             dfs(x, n, k, &mut count, MODULO);
    //         }
    //         println!("{}", count % MODULO);
    //     }
    // }
    
    
    
}

// 繰り返し2乗法 a^xを求める
fn iterative_square_method(mut a: usize, mut x: usize, MODULO: usize) -> usize {
    // answer = a ^ x を得たいとき
    //        = (a^2)^(x/2) * a^(x%2)

    // answer = 3 ^3 を得たいとき
    //        = (3^2)^(3/2) * 3^(3%2)
    //        = 9^1 * 3^1

    // answer = 3 ^ 4 を得たいとき
    //        = (3^2)^(4/2) * (3^2)^(4%2)
    //        = 9^2 * 3^0
    //        = (9^2)^(2/2) * 9^(2&2) * 3^0
    //        = 81^1 * 9^0 * 3^0

    // answer = 3 ^ 5を得たいとき
    // answer = (3^2)^(5/2) * 3^(5%2)
    //        = (3^2)^2 * 3^1
    //        = ((3^2)^2)^(2/2) * (3^2)^(2%2) * 3^1
    //        = ((3^2)^2)^1 * (3^2)^0 * 3^1
    //        = (3^4)^1 * (3^2)^0 * 3^1

    // answer = 3 ^ 7を得たいとき
    // answer = (3^2)^(7/2) * 3^(7%2)
    //        = (3^2)^3 * 3^1
    //        = 9^3 * 3^1
    //        = (9^2)^(3/2) * 9^(3%2) * 3^1
    //        = 81^1 * 9^1 * 3^1

    a %= MODULO;
    let mut answer = 1;
    while x >= 1 {
        if x % 2 == 1 {
            answer = (answer * a) % MODULO;
        }
        x = x / 2;
        a = a * a % MODULO;
    }

    return answer;
}

// フェルマーの小定理x^(p-1) = 1 (mod p)により逆元を求める x^(-1) = x ^ (p - 2) (mod p)
fn get_inverse(x: usize, MODULO: usize) -> usize {
    // x^(p-2)はO(p-2)の計算量がかかってしまうが、繰り返し二乗法で、O(log2(p-2))まで落とせる。
    let inverse =  iterative_square_method(x, MODULO - 2, MODULO);
    return inverse;
}

// nCrを求める
fn combination(n: usize, r: usize, MODULO: usize, memo_factorial: &Vec<usize>) -> usize {
    // nCr = n! / ((n-r)! * r!) % MODULO ;
    // n!は事前にメモ化して計算済み
    // 分母の逆数(逆元)は、フェルマーの小定理により求める
    let top = memo_factorial[n];
    let bottom = ((memo_factorial[n-r]) * (memo_factorial[r])) % MODULO;
    let ncr = (top * get_inverse(bottom, MODULO)) % MODULO;
    return ncr
}

// N = 7
// k = 2

// {1 2 3 4 5 6 7}

// {1 _ _ _ _ _ _}

// {1 _ 3 _ _ _ _}
// {1 _ _ 4 _ _ _}
// {1 _ _ _ 5 _ _}
// {1 _ _ _ _ 6 _}
// {1 _ _ _ _ _ 7}

// {1 _ 3 _ 5 _ _}
// {1 _ 3 _ _ 6 _}
// {1 _ 3 _ _ _ 7}

// {1 _ 3 _ 5 _ 7}

fn dfs(mut x: usize, n: usize, k: usize, count: &mut usize, MODULO: usize) {
    *count += 1;
    *count %= MODULO;

    x += k;
    while x < n {
        dfs(x, n, k, count, MODULO);
        x += 1;
    }
}