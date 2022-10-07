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
        k: usize,
    }
    let mut x = vec![];
    let mut y = vec![];

    for i in 0..n {
        input! {
            x_i: isize,
            y_i: isize,
        }
        x.push(x_i);
        y.push(y_i);
    }
    // 2^60 = 1152921504606846976 = 10^(18.06179973983887)
    let INF: usize = 1 << 60; 

    // dist[i][j]: 頂点iとjのユークリッド距離の2乗
    let mut dist = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            let distance_ij = ((x[i] - x[j]) * (x[i] - x[j]) + (y[i] - y[j]) * (y[i] - y[j])) as usize;
            dist[i][j] = distance_ij;
        }
        // println!("{:?}", dist[i]);
    }

    // // E869120君の解答の翻訳
    // let mut cost = vec![0; 1 << n];
    // for bit in 1..(1<<n) {
    //     for i in 0..n {
    //         for j in 0..i {
    //             if (((bit >> i) & 1) == 1) && (((bit >> j) & 1) == 1) {
    //                 cost[bit] = max(cost[bit], dist[i][j]);
    //             }
    //         }
    //     }    
    // }
    // let mut dp = vec![vec![INF; 1 << n]; k+1];
    // dp[0][0] = 0;
    // // println!("dp.len(): {}", dp.len());
    // // println!("dp[0].len(): {}", dp[0].len());
    // for i in 1..(k+1) {
    //     for bit in 1..(1 << n) {
    //         let mut subset_bit = bit;
    //         while subset_bit != 0 {
    //             dp[i][bit] = min(dp[i][bit], max(dp[i-1][bit-subset_bit], cost[subset_bit]));
    //             subset_bit = (subset_bit - 1) & bit;
    //         }
    //     }
    // }
    // println!("{}", dp[k][(1 << n) - 1]);

    // 自分の解答
    // cost[bit]: グループに含まれている頂点のパターンが、bit(n個の各頂点が含むor含まれないを網羅した全2^nのパターン)　のとき、そのグループ内の最大距離
    let mut cost = vec![0; 1 << n];
    for bit in 0..(1 << n) {
        for i in 0..n {
            for j in (i+1)..n {
                if ((bit & (1 << i)) != 0) && ((bit & (1 << j)) != 0) {
                    cost[bit] = max(cost[bit], dist[i][j]);
                }
            }
        }
        // println!("bit:{:10b}, cost[bit]:{}", bit, cost[bit]);
    }

    // dp[既に選んだ点達(bit)][グループ数-1] = グループ間の最大距離を最小化した値
    let mut dp = vec![vec![INF; k]; 1 << n];
    // 初期化
    for bit in 0..(1 << n){
        dp[bit][0] = cost[bit];
    }

    // // 計算量: ビットの部分集合の探索
    // 2^15 = 32,768 > 3*10^4
    // 3^15 = 14,348,907 > 1*10^7 (工夫するとこうなる。subset_bit = (subset_bit - 1) & bitで更新)
    // 4^15 = 1,073,741,824 < 1*10^9 => TLE (愚直にやるとこうなる)
    for i in 1..k {
        for bit in 0..(1 << n) {
            // bitの部分集合を大きいものから順に調べていく
            let mut subset_bit = bit;
            while subset_bit != 0 {
                // dp[bit][i] = min(dp[bit][i], dp[subset_bit][i-1] + cost[bit - subset_bit]);
                dp[bit][i] = min(dp[bit][i], max(dp[bit - subset_bit][i-1], cost[subset_bit]));
                // 次に小さいbitの部分集合に更新
                subset_bit = (subset_bit - 1) & bit;
            }
        }
    }
    // for i in 0..(1<<n) {
    //     println!("bit: {:10b}, dp: {:?}", i, dp[i]);
    //     // if i == 1 << 9 {break}
    // }

    let max_bit = (1 << n) - 1;
    let ans = dp[max_bit][k - 1];
    println!("{}", ans);
}
