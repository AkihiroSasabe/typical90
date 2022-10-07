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
        l: usize
    }

    let mut dp: Vec<usize> = vec![0; n+1];
    if n < l {
        println!("{}", 1);
        return;
    }
    // 初期化
    for i in 0..l {
        dp[i] = 1;
    }
    for i in l..n+1 {
        dp[i] = dp[i-1] % (1_000_000_000 + 7) + dp[i-l] % (1_000_000_000 + 7);
    }
    println!("{}", dp[n] % (1_000_000_000 + 7));

}

// 階乗の計算でゴリ押ししようとすると、オーバーフローを起こす
// fn main() {
//     input! {
//         n: usize,
//         l: usize
//     }
//     let mut l_max = n / l;

//     let mut result = vec![0; n+1];
//     result[0] = 1;
//     let mut answer = 0;
//     for i in 0..(l_max+1) {
//         answer += combination(n - i * l + i, i, &mut result);
//     }
//     println!("{}", answer);
// }

// fn kaijo(n: usize, result: &mut Vec<usize>) -> usize {
//     if result[n] != 0 {
//         return result[n]
//     }
//     else {
//         result[n] = n*kaijo(n-1, result);
//         return result[n];
//     }
// }

// fn combination(n: usize, r: usize, result: &mut Vec<usize>) -> usize {
//     // nCr = n! / (r!*(n-r)!)
    
//     return kaijo(n, result) / (kaijo(r, result) * kaijo(n-r, result))
// }

