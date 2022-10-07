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
        mut dcs: [[usize; 3]; n]
    }
    dcs.sort();
    // let mut d = vec![];
    // let mut c = vec![];
    // let mut s = vec![];
    // for i in 0..n {
    //     input! {
    //         d_i: usize,
    //         c_i: usize,
    //         s_i: usize,
    //     }
    //     d.push(d_i);
    //     c.push(c_i);
    //     s.push(s_i);
    // }
    let MAX_DAYS = 5001;
    // let MAX_DAYS = 10;
    // dp[N個目の仕事まで熟す][D日目の終わり] = 最大N個目までの仕事が出来るとき、D日目の終わりにおける報酬の最大値
    let mut dp = vec![vec![0; MAX_DAYS]; n+1];
    for i in 1..(n+1) {
        for j in 1..MAX_DAYS {
            // 遷移のパターンは3通り
            // (1)上からそのまま降りてくる
            let cand_0 = dp[i-1][j];
            // (2)左からそのまま継続
            let cand_1 = dp[i][j-1];
            let mut cand_2 = 0;
            if j >= dcs[i-1][1] && j <= dcs[i-1][0] {
                cand_2 = dp[i-1][j-dcs[i-1][1]] + dcs[i-1][2];
            }
            dp[i][j] = max(cand_0, cand_1);
            dp[i][j] = max(dp[i][j], cand_2);
        }
    }

    // for i in 0..(n+1) {
    //     println!("{:?}", dp[i]);
    // }

    // n個目で、MAX_DAYSの最大報酬
    let ans = dp[n][MAX_DAYS - 1];
    println!("{}", ans);

}