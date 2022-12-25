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
        p: usize,
        k: usize,
        a: [[isize; n]; n]
    }

    // while (left < right)
    // let mut x = p + 1;
    // let mut ans = 0;

    // 個数がk以下になる最小の交通費x(x_for_k)を知りたい。
    let ng: usize = 0;
    let ok: usize = p + 1;
    let mut x_for_k = meguru_binary_search(ok, ng, n, p, k, &a);

    // 個数がk-1以下になる最小の交通費x(x_for_km1)を知りたい。(x_for_k =< x_for_km1)
    let ng: usize = 0;
    let ok: usize = p + 1;
    let x_for_km1: usize;
    if k != 0 {
        x_for_km1 = meguru_binary_search(ok, ng, n, p, k-1, &a);
    }
    else {
        // k = 0 ということは、xの選び方が無限に存在することになる。(最終的にInfinityとなるように適当に0にしておく)
        x_for_km1 = 2 * x_for_k;
    }

    let ans = x_for_km1 - x_for_k;
    // println!("{}", x_for_k);
    // println!("{}", x_for_km1);
    // println!("{}, n:{}, p:{}, k:{}", x_for_km1 - x_for_k, n, p, k);
    
    // xが大きければ大きいほど、p円以下となる(i,j)の組み合わせの数は小さくなる。
    // x = INFでも(i,j)の組み合わせの数がk個固定で存在してしまうケースでは、ansが無限になる
    // xがINFでも、ということは、x=p+1円でもok
    let dp = floyd_warshall(p+1, n, &a);
    let count = count_available_ij_patterns(&dp, n, p);
    let mut infinity_flag = false;
    if count == k {
        infinity_flag = true;
    }

    if infinity_flag {
        println!("Infinity");
    }
    else {
        println!("{}", ans);
    }
}

// フロイド・ワーシャル法で、全頂点対間の距離をO(V^3)で最小化 (全点対間最短経路問題)
// この問題を解く用にアレンジされているのでabc257_Dを参考: https://github.com/AkihiroSasabe/atcoder/blob/main/src/abc257/d.rs: 
fn floyd_warshall(x: usize, n: usize, graph: &Vec<Vec<isize>>) -> Vec<Vec<usize>> {
    // dp[i][j]で頂点iから頂点jに行くときの最短距離
    let mut dp = vec![vec![0; n]; n];

    // dpの初期化
    for i in 0..n {
        for j in 0..n {
            if i == j {
                dp[i][j] = 0;
            }
            else if graph[i][j] != -1 {
                dp[i][j] = graph[i][j] as usize;
            }
            else {
                dp[i][j] = x;
            }
        }
    }
    
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                // k未満の頂点(0-k-1)のみを、中継点として通って良い。
                dp[i][j] = min(dp[i][j], dp[i][k] + dp[k][j]);
                // 例
                // dp[0][2] = min(dp[0][2], dp[0][1] + dp[1][2]);
                // dp[0][3] = min(dp[0][3], dp[0][1] + dp[1][3]);
                // dp[0][4] = min(dp[0][4], dp[0][1] + dp[1][4]);
            }
        }
    }

    return dp
}

fn meguru_binary_search(mut ok: usize, mut ng: usize, n: usize, p: usize, k: usize, a: &Vec<Vec<isize>>) -> usize {
    while ok > ng + 1 {
        let mid = (ok + ng) / 2;
        // println!("ng: {}, mid: {}, ok: {}, k:{}", ng, mid, ok, k);

        // フロイド・ワーシャル法で、全頂点対間の距離を最小化
        let dp = floyd_warshall(mid, n, a);
    
        // for i in 0..n {
        //     println!("{:?}", dp[i]);
        // }
    
        let count = count_available_ij_patterns(&dp, n, p);
        if count <= k {
            ok = mid;
        }
        else {
            ng = mid;
        }
        // println!("ng: {}, mid: {}, ok: {}, count: {}, k:{}", ng, mid, ok, count, k);
    }

    return ok;
}

fn count_available_ij_patterns(dp: & Vec<Vec<usize>>, n: usize, p: usize) -> usize {
    let mut count = 0;
    for i in 0..n {
        for j in (i+1)..n {
            if dp[i][j] <= p {
                count += 1;
            }
        }
    }
    return count
}