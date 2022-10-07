use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use proconio::marker::Chars;
fn main() {
    input! {
        n: usize
    }
    let mut graph = vec![vec![]; n];
    for i in 0..(n-1) {
        input! {
            mut a_i: usize,
            mut b_i: usize
        }
        a_i -= 1;
        b_i -= 1;
        graph[a_i].push(b_i);
        graph[b_i].push(a_i);
    }

    // 色
    let mut color = vec![2; n];
    // グラフ彩色 (木のdfsなのでn回ループしなくても、全頂点を探索可能)
    dfs(&graph, 0, &mut color, 0);

    // 同色の頂点を格納する(深さが偶数、奇数の頂点で分ける)
    // 参考: https://logicalbear.net/%E3%80%90%E7%AB%B6%E3%83%97%E3%83%AD%E5%85%B8%E5%9E%8B90%E5%95%8F%E3%80%91%E3%80%8C008-atcounter%EF%BC%88%E2%98%854%EF%BC%89%E3%80%8D%E8%A7%A3%E6%B3%95-2/
    let mut g0 = vec![];
    let mut g1 = vec![];
    for v in 0..n {
        if color[v] == 0 {g0.push(v)}
        if color[v] == 1 {g1.push(v)}
    }

    // 数が多い方の色からN/2個取る
    let answer: Vec<usize>;
    if g0.len() <= g1.len() {
        answer = g1[0..n/2].to_vec();
    }
    else {
        answer = g0[0..n/2].to_vec();
    }
    for i in 0..n/2 {
        // print!("{} ", answer[i]);
        print!("{} ", answer[i]+1);
    }
}

// 2部グラフ
fn dfs(graph: &Vec<Vec<usize>>, v: usize, color: &mut Vec<usize>, current_color: usize) {
    color[v] = current_color;
    // println!("i: {}, color: {}", v, color[v]);

    for i in 0..graph[v].len() {
        let next_v = graph[v][i];
        if color[next_v] != 2 {continue}
        dfs(graph, next_v, color, 1 - current_color);
    }
}