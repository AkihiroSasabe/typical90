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
        a: [isize; 2*n]
    }

    // 列の操作は区間DP
    // 短い区間から順に、求めていく。
    let INF = 1 << 60;
    // lからrまでの全ての数列を削除するのに必要なコスト
    let mut dp = vec![vec![INF; 2*n]; 2*n];

    for diff in (1..(2*n)).step_by(2) {
        for l in 0..2*n {
            let r = l + diff;
            if r > 2*n - 1 {continue}
            if diff == 1 {
                dp[l][r] = (a[l] - a[r]).abs();
            }
            else {
                // (1)最後にlとrが消し合うケース
                dp[l][r] = min(dp[l][r], dp[l+1][r-1] + (a[l] - a[r]).abs());
                // (2)それ以外のケース。[l, mid]と[mid+1, r]の2区間に分割できる。隣り合う項同士(A_iとA_i+1)でしか消せ合えないので、lとmidが消し合うなら、rが[l+1,mid-1]間で消し合うことはない
                for mid in ((l+1)..(r-1)).step_by(2) {
                    dp[l][r] = min(dp[l][mid] + dp[mid+1][r], dp[l][r]);
                }
            }
        }
    }
    println!("{}", dp[0][2*n-1]);


    // // コストが最小のものを貪欲に外していく (この方法だとAC出来ない)
    // let INF = 1 << 60;
    // let mut cost_list = vec![];
    // for i in 0..(2*n-1) {
    //     let cost = (a[i] - a[i+1]).abs() as usize;
    //     let mut pre_index = INF;
    //     if i >= 1 {
    //         pre_index = i - 1;
    //     }
    //     let next_index = i + 1;
    //     // let future_index = i + 2;
    //     // cost_list.push(vec![cost, pre_index, next_index, future_index]);
    //     cost_list.push(vec![cost, pre_index, next_index]);
    // }
    // println!("{:?}", cost_list);


    // let mut ans = 0;
    // for _ in 0..n {
    //     println!("========================================");
    //     let mut min_cost = INF;
    //     let mut min_index = INF as usize;
    //     for i in 0..(2*n-1) {
    //         if min_cost > cost_list[i][0] {
    //             min_index = i;
    //             min_cost = cost_list[i][0];
    //         }
    //     }
    //     println!("min_cost: {}", min_cost);
    //     ans += min_cost;
    //     let pre_index = cost_list[min_index][1];
    //     let next_index = cost_list[min_index][2];
    //     // let mut next_next_index = 2*n - 1;
    //     let mut next_next_index = INF;
    //     if next_index <= 2*n-2{
    //         next_next_index = cost_list[next_index][2];
    //     }
    //     // let future_index = cost_list[min_index][3];
    //     println!("min_index: {}, pre_index: {}, next_index: {}, next_next_index: {}", min_index, pre_index, next_index, next_next_index);
    //     if next_next_index <= 2*n-2 {
    //         cost_list[next_next_index][1] = pre_index;
    //     }
    //     if pre_index != INF {
    //         if next_next_index <= 2*n-1 {
    //             cost_list[pre_index][0] = (a[pre_index] - a[next_next_index]).abs() as usize;
    //         }
    //         else {
    //             cost_list[pre_index][0] = INF;
    //         }
    //         cost_list[pre_index][2] = next_next_index;
    //     }
    //     cost_list[min_index][0] = INF;
    //     if next_index <= 2*n-2 {
    //         cost_list[next_index][0] = INF;
    //     }
    //     print!("==cost_list==: ");
    //     for j in 0..(2*n-1) {
    //         print!("{:?}, ", cost_list[j][0]);
    //     }
    //     println!("");
    // }

    // println!("{}", ans);
    

}

