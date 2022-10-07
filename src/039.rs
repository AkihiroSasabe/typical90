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
    }
    let mut graph = vec![vec![]; n];
    for i in 0..(n-1) {
        input! {
            a_i: usize,
            b_i: usize,
        }
        graph[a_i - 1].push(b_i - 1);
        graph[b_i - 1].push(a_i - 1);
    }
    
    let mut seen = vec![false; n];
    // dp[i]には、頂点iを根とする部分木のサイズが格納される
    let mut dp = vec![0; n];
    for v in 0..n {
        if seen[v] {continue}
        dfs(&graph, v, &mut seen, &mut dp);
    }

    let mut seen = vec![false; n];
    let mut sum = 0;
    for v in 0..n {
        if seen[v] {continue}
        dfs2(&graph, v, &mut seen, & dp, &mut sum, n);
    }
    println!("{}", sum);

}

// 頂点iを根とする部分木のサイズdp[i]を取得する
fn dfs(graph: &Vec<Vec<usize>>, v: usize, seen: &mut Vec<bool>, dp: &mut Vec<usize>) {
    seen[v] = true;
    dp[v] += 1;
    for i in 0..graph[v].len() {
        let next_v = graph[v][i];
        if seen[next_v] {continue;}
        dfs(graph, next_v, seen, dp);
        dp[v] += dp[next_v];
    }
}

// 各辺の貢献度を足し合わせていく。dp[next_v] * (n - dp[next_v])
fn dfs2(graph: &Vec<Vec<usize>>, v: usize, seen: &mut Vec<bool>, dp: &Vec<usize>, sum: &mut usize, n: usize) {
    seen[v] = true;
    for i in 0..graph[v].len() {
        let next_v = graph[v][i];
        if seen[next_v] {continue}
        dfs2(graph, next_v, seen, dp, sum, n);
        *sum += dp[next_v] * (n - dp[next_v]);
    }
}