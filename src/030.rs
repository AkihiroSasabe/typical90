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
fn sieve_oferatosthenes(n: usize) -> Vec<usize> {
    // let mut root_n = 1;
    // while root_n * root_n <= n {
    //     root_n += 1;
    // }
    // let mut sosu_list = vec![1; root_n + 1];
    let mut sosu_list = vec![1; n + 1];
    sosu_list[0] = 0;
    sosu_list[1] = 0;

    let mut sosu = 2;
    
    for i in 2..(n+1) {
        if sosu_list[i] == 0 {continue}
        if n % i == 0 {
            for j in 2..(n/i) {
                sosu_list[j * i] = 0;
            }
        }
    }
    return sosu_list;
}