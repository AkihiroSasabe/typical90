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
        s: usize,
    }
    let mut a = vec![];
    let mut b = vec![];
    for i in 0..n {
        input! {
            a_i: usize,
            b_i: usize,
        }
        a.push(a_i);
        b.push(b_i);
    }
    // dp[Num][Money]は、それを実現することが可能か否かがboolで格納されている。
    let mut dp = vec![vec![false; s+1]; n+1];
    dp[0][0] = true;

    for i in 0..n {
        for j in 0..(s+1) {
            // 配る方式のDP。
            // 配れるかどうかを判定
            if dp[i][j] {
                if j + a[i] <= s{
                    dp[i+1][j+a[i]] = true;
                }
                if j + b[i] <= s{
                    dp[i+1][j+b[i]] = true;
                }
            }
        }
    }
    // for i in 0..(n+1) {
    //     println!("{:?}", dp[i]);
    // }
    
    if !dp[n][s] {
        println!("Impossible");
    }
    else {
        // DP復元をする
        let mut ans = vec!['a'; n];
        let mut money = s;
        for i in 0..n {
            if money >= a[n-1-i] && dp[n-1-i][money-a[n-1-i]] {
                ans[n-1-i] = 'A';
                money -= a[n-1-i];
            }
            else if money >= b[n-1-i] && dp[n-1-i][money-b[n-1-i]] {
                ans[n-1-i] = 'B';
                money -= b[n-1-i];
            }
        }

        for i in 0..n {
            print!("{}", ans[i]);
        }
        
        // let mut ans = vec![];
        // let mut end_flag = false;
        // dp_reconstruction(&dp, n, s, &a, &b, ans, &mut end_flag, n);
        // println!("{}", dp[n][s]);
    }
    

    // DP復元しないでそのままやるとTLEする。
    // // dp[Num][Money]は、それを実現する上で必要な文字列が格納されている。
    // let mut dp = vec![vec![vec![]; s+1+100_000]; n+1];
    // dp[1][a[0]] = vec!['A'];
    // dp[1][b[0]] = vec!['B'];
    // // let negative_sample = vec![];

    // for i in 1..n {
    //     for j in 0..(s+1) {
    //             // 配る方式のDP。
    //             // 配れるかどうかを判定
    //         if dp[i][j].len() != 0 {
    //             let mut dp_ij_for_a = dp[i][j].clone();
    //             dp_ij_for_a.push('A');
    //             let mut dp_ij_for_b = dp[i][j].clone();
    //             dp_ij_for_b.push('B');
    //             dp[i+1][j+a[i]] = dp_ij_for_a;
    //             dp[i+1][j+b[i]] = dp_ij_for_b;
    //         }
    //     }
    // }
    // // for i in 0..(n+1) {
    // //     println!("{:?}", dp[i]);
    // // }
    // if dp[n][s].len() == 0 {
    //     println!("Impossible");
    // }
    // else {
    //     for i in 0..dp[n][s].len() {
    //         print!("{}", dp[n][s][i]);
    //     }
    // }

}

// fn dp_reconstruction(dp: &Vec<Vec<bool>>, num: usize, money: usize, a: &Vec<usize>, b: &Vec<usize>, ans: Vec<char>, end_flag: &mut bool, n: usize) {
//     if num == 1 {
//         if money == a[0] {
//             let mut ans2 = ans.clone();
//             ans2.push('A');
//             if !*end_flag {
//                 for i in 0..n {
//                     print!("{}", ans2[n - 1 - i]);
//                 }
//                 *end_flag = true;
//             }
//         }
//         else if money == b[0] {
//             let mut ans2 = ans.clone();
//             ans2.push('B');
//             if !*end_flag {
//                 for i in 0..n {
//                     print!("{}", ans2[n - 1 - i]);
//                 }
//                 *end_flag = true;
//             }
//         }
//         return;
//     }
//     if money >= a[num-1] && dp[num - 1][money - a[num-1]] {
//         let mut ans2 = ans.clone();
//         ans2.push('A');
//         dp_reconstruction(dp, num-1, money - a[num-1], a, b, ans2, end_flag, n);
//     }
//     if money >= b[num-1] && dp[num - 1][money - b[num-1]] {
//         let mut ans2 = ans.clone();
//         ans2.push('B');
//         dp_reconstruction(dp, num-1, money - b[num-1], a, b, ans2, end_flag, n);
//     }
// }

