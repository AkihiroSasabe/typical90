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
        s: Chars
    }

    let modulo = 1_000_000_007;

    // 入力例:
    // attcordeer
    // DP table
    //         a   t   c   o   d   e   r
    // 0   a   1   0   0   0   0   0   0
    // 1   t   1   1   0   0   0   0   0   
    // 2   t   1   2   0   0   0   0   0   
    // 3   c   1   2   2   0   0   0   0
    // 4   o   1   2   2   2   0   0   0
    // 5   r   1   2   2   2   0   0   0
    // 6   d   1   2   2   2   2   0   0
    // 7   e   1   2   2   2   2   2   0
    // 8   e   1   2   2   2   2   4   0
    // 9   r   1   2   2   2   2   4   4

    let template: Vec<char> = "atcoder".chars().collect();

    let mut dp = vec![vec![0; template.len()]; n];
    
    // dpの初期化
    let mut head_match_count = 0;
    for i in 0..n {
        if s[i] == template[0] {
            head_match_count += 1;
        }
        dp[i][0] = head_match_count;
    }
    // for i in 0..n {
    //     println!("{:?}", dp[i]);
    // }

    for i in 1..n {
        for j in 1..template.len() {
            if s[i] == template[j] {
                // println!("match!");
                if dp[i-1][j] == 0 {
                    // println!("update!!");
                    // println!("dp[i][j]: {}, dp[i][j-1]: {}", dp[i][j], dp[i][j-1] );
                    dp[i][j] = dp[i][j-1] % modulo;
                }
                else {
                    // println!("== update!!");
                    // dp[i][j] = dp[i-1][j] + 1;
                    dp[i][j] = (dp[i-1][j] + dp[i][j-1]) % modulo;
                }
            }
            else {
                if dp[i-1][j] != 0 {
                    dp[i][j] = dp[i-1][j] % modulo;
                }
            }
        }
    }
    println!("{}", dp[n-1][template.len() - 1] % modulo);
    // for i in 0..n {
    //     println!("{:?}", dp[i]);
    // }

}