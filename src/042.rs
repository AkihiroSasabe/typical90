use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use proconio::marker::Chars;
fn main() {
    input! {
        k: usize,
    }
    let modulo = 1_000_000_007;
    
    if k % 9 != 0 {
        println!("0");
    }
    else {
        // 定義
        // dp[各桁の和] = 通り数
        // dp[i] = 先頭が1のものの個数 + 先頭が2のものの個数 + ... * 先頭が9のものの個数 
        //       = dp[i-1] + dp[i-2] + ... + dp[i-9]

        // 確認
        // dp[0] = 0; // <= 本当は0だが、便宜的に1にしておくと計算が楽
        // dp[1] = 1; // 1
        // dp[2] = 2; // 2, 11
        // dp[3] = 4; // 3, 12,21,         111
        // // 先頭が1のものの個数=dp[2]=2、先頭が2のものの個数=dp[1]=1、先頭が3のものの個数=1個　

        // dp[4] = 8; // 4, 13,22,31,      112,121,211,                1111
        // // 先頭が1のものの個数=dp[3]=4, dp[2]=2, dp[1], 先頭が4のものの個数1. 総和=4+2+1+1=8\\\\\\\\
        // dp[5] = 16; // 5, 14,23,32,41,   113,122,131,212,221,311,    1112,1121,1211,2111, 11111
        // dp[6] = 32;
        // dp[7] = 64
        // dp[8] = 128
        // dp[9] = 256;

        let mut dp = vec![0; k+1];
        dp[1] = 1;
        for i in 2..(k+1) {
            for j in 1..10 {
                if i < j {continue}
                if i < 10 && i - j == 0 {dp[i] += 1;}
                dp[i] = (dp[i] + dp[i-j]) % modulo;
            }
            // if i < 10 {
            //     dp[i] += dp[i];
            // }
        }
        println!("{}", dp[k]);

    }
}