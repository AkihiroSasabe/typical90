use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::f32::consts::E;
use proconio::marker::Chars;
fn main() {
    // https://atcoder.jp/contests/typical90/tasks/typical90_ci
    input! {
        n: usize, // N個の街
        p: isize, // 街iから街jへの許容最大コスト (0 <= i < j <= n-1)
        k: usize, // 可能な(i,j)の組の個数
        a: [[isize; n]; n] // i -> j のコスト (-1のとき、x)
    }

    // X = P + 1 としたときに、 K 個なら、A[i][j] == -1 となる辺(i,j)は通る必要がないため、どんな X を設定してもよい。
    // つまり、条件を満たすXが無限個ある
    if count_available_ij_patterns(p+1, &a, p) == k {
        println!("Infinity");
        return
    }

    // X が最小のとき (X=1)、P円以下で到達可能な(i,j)がK個以上にならないなら、どんなxでも不可 (count(x)は単調減少だから)
    if !judge(1, &a, k, p) {
        println!("0");
        return
    }

    // K個以上 の最小のX: x_for_k
    // K+1個以上 の最小のX: x_for_kp1
    let x_for_k = meguru_binary_search(k, &a, p);
    // println!("x_for_k = {:?}", x_for_k);

    let ans = if !judge(1, &a, k+1, p) {
        // X = 1 で、K+1個以上存在しないとき
        x_for_k
    } else {
        let x_for_kp1 = meguru_binary_search(k+1, &a, p);
        x_for_k - x_for_kp1
        // println!("x_for_kp1 = {:?}", x_for_kp1);
    };
    println!("{}", ans);
    
}

// 類題: abc73_D: https://atcoder.jp/contests/abc073/tasks/abc073_d
// 類題: abc257_D: https://github.com/AkihiroSasabe/atcoder/blob/main/src/abc257/d.rs
// フロイド・ワーシャル法で、全頂点対間の距離をO(V^3)で最小化 (全点対間最短経路問題)
fn floyd_warshall<T>(graph: &Vec<Vec<(usize, T)>>) -> Vec<Vec<T>> 
    where T: 
        Copy + 
        Ord +
        std::cmp::PartialEq + 
        std::ops::Div<Output = T> +
        num::Zero +
        num::One +
        num::Bounded // max_value() で要る
{
    // 頂点数
    let n = graph.len();

    // 初期化のために、任意の型に対応した、 0 と max / 2 が必要。
    let zero: T     = T::zero();
    let one: T      = T::one();
    let two: T      = one + one;
    let ten: T      = two + two + two + two + two;
    let inf: T      = T::max_value() / ten;
    // let INF: usize = usize::MAX / 10;


    // dp[i][j]で頂点iから頂点jに行くときの最短距離
    let mut dp: Vec<Vec<T>> = vec![vec![inf; n]; n];

    // dpの初期化
    for v in 0..n {
        // 同一頂点への移動は0
        dp[v][v] = zero;
        for i in 0..graph[v].len() {
            // 直接遷移可能な頂点への移動を格納
            let nv = graph[v][i].0;
            let dist = graph[v][i].1;
            dp[v][nv] = dp[v][nv].min(dist);
        }
    }
    
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                // dp[i][j] := i -> j へ、k未満の頂点(0 ~ k-1)のみを、中継点として通って良い。
                dp[i][j] = dp[i][j].min(dp[i][k] + dp[k][j]);
                // 例 k = 1の時
                // dp[0][0] = min(dp[0][0], dp[0][1] + dp[1][0]);
                // dp[0][1] = min(dp[0][1], dp[0][1] + dp[1][1]);
                // dp[0][2] = min(dp[0][2], dp[0][1] + dp[1][2]);
                // dp[0][3] = min(dp[0][3], dp[0][1] + dp[1][3]);
                // dp[0][4] = min(dp[0][4], dp[0][1] + dp[1][4]);
            }
        }
    }
    return dp
}




fn meguru_binary_search(k: usize, a: &Vec<Vec<isize>>, p: isize) -> isize {
    // めぐる式二分探索
    // (i,j)の組が、K個以上となる最小の x を求める 
    // p円以下となる(i,j)の組み合わせの数 count(x) は
    // xが大きければ大きいほど、小さくなり、単調減少なので二分探索が使える。
    let mut ng: isize = p + 1;
    let mut ok: isize = 1;
    if judge(ng, a, k, p) {
        ng = ok;
    }
    if !judge(ok, a, k, p) {
        return 0
    }
    while (ng as i128 - ok as i128).abs() > 1 {
        let mid = (ng + ok) / 2;
        let is_ok = judge(mid, a, k, p);
        if is_ok {
            ok = mid;
        }
        else {
            ng = mid;
        }
    }
    return ok
}

fn count_available_ij_patterns(x: isize, a: &Vec<Vec<isize>>, p: isize) -> usize {
    // a[i][j] == -1 の交通費が x 円のとき、
    // 合計交通費 p 円以下で到達可能な(i,j) の組み合わせ数

    // グラフの形成
    let n = a.len();
    let mut graph = vec![vec![]; n];
    for i in 0..n {
        for j in 0..n {
            if a[i][j] != -1 {
                graph[i].push((j, a[i][j]));
            }
            else {
                graph[i].push((j, x));
            }
        }
    }

    // 全頂点対間距離
    let dp = floyd_warshall(&graph);
    // count := 条件を満たす (i,j) の個数
    let mut count = 0;
    for i in 0..n {
        for j in i+1..n {
            if dp[i][j] <= p {
                count += 1;
            }
        }
    }
    return count
}

fn judge(mid: isize, a: &Vec<Vec<isize>>, k: usize, p: isize) -> bool {
    // x = mid のとき、k個以上の(i,j)の組み合わせが存在するか判定
    let count = count_available_ij_patterns(mid, a, p);
    return count >= k
}
