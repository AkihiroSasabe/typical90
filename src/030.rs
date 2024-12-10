use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use proconio::marker::Chars;
use std::collections::HashMap;
fn main() {
    input! {
        n: usize,
        k: usize
    }

    // 数字iが素数か否かを判定するリスト。要素が0なら非素数、1なら素数。
    let mut prime_factor_list = vec![1; n+1];
    prime_factor_list[0] = 0;
    prime_factor_list[1] = 0;
    // 数字iの素因数の個数を格納したリスト。
    let mut the_number_of_prime_factor = vec![0; n+1];
    
    for prime_factor in 2..(n+1) {
        // prime_factorが素数じゃないならスキップ
        if prime_factor_list[prime_factor] == 0 {continue}
        // prime_factorが素数であれば、その素因数は自身のみなので1となる。
        the_number_of_prime_factor[prime_factor] = 1;

        // n以下の素因数の倍数をカウントしていく
        for j in 2..(n / prime_factor + 1) {
            // println!("prime_factor:{} j:{}", prime_factor, j);
            // 素因数の倍数は、素数ではないので篩(ふるい)にかける。(エラトステネスの篩という)
            prime_factor_list[j * prime_factor] = 0;
            the_number_of_prime_factor[j * prime_factor] += 1;
        }
    }
    let mut ans = 0;
    for i in 2..(n+1) {
        // println!("i:{}, num:{}", i, the_number_of_prime_factor[i]);
        if the_number_of_prime_factor[i] >= k {
            ans += 1;
        }
    }
    println!("{}", ans);

}

// エラトステネスの篩(ふるい)
// n以下の素数を全て列挙する為のアルゴリズムO(n*log(log(n)))
fn sieve_of_eratosthenes(n: usize) -> Vec<usize> {
    // is_prime_list[i] := iが素数なら true , そうでなければ false
    let mut is_prime_list = vec![true; n + 1]; // この初期化でO(N)かかる!
    is_prime_list[0] = false; // 0は素数ではない
    is_prime_list[1] = false; // 1は素数ではない

    // prime_list := n以下の素数を格納したリスト
    let mut prime_list = vec![];
    
    // ここの計算で、O(N/2 + N/3 + N/5 + N/7 + N/11 + ...)  = O(N (1/2 + 1/3 + 1/5 + 1/7 + 1/11 + ... )) = O(Nlog(logN))かかる。
    // ※素数の逆数和は、log(logN)と漸近していくため。自然数の逆数和は、logNに漸近する。
    for i in 2..(n+1) {
        if !is_prime_list[i] {continue}
        for j in 2..((n/i)+1) {
            // i の倍数が素数ではないことを記録
            is_prime_list[j * i] = false;
        }
        prime_list.push(i);
    }
    return prime_list;
}