use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::hash::Hash;
use proconio::marker::Chars;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        c: [char; n],
    }

    // let mut a = vec![];
    // let mut b = vec![];
    let mut graph = vec![vec![]; n];
    for i in 0..(n-1) {
        input! {
            a_i: usize,
            b_i: usize
        }
        graph[a_i - 1].push(b_i - 1);
        graph[b_i - 1].push(a_i - 1);
    }
    let MODULO = 1_000_000_007;

    // dp[i][状態]: 頂点iが、状態{0: aのみ、1: bのみ、2: a&b両方}における取りうる総数
    let mut dp = vec![vec![0; 3]; n];

    let mut seen = vec![false; n];
    for v in 0..n {
        if seen[v] {continue}
        dfs(&graph, &mut dp, v, &mut seen, &c, MODULO);
    }
    let ans = dp[0][2];
    // let mut ans = 0;
    // for i in 0..n {
    //     ans = max(ans, dp[i][2]);
    // }
    println!("{}", ans);

    // for i in 0..n {
    //     println!("i: {}, dp: {}", i, dp[i][2]);
    // }



}

fn dfs(graph: & Vec<Vec<usize>>, dp: &mut Vec<Vec<usize>>, v: usize, seen: &mut Vec<bool>, c: &Vec<char>, MODULO: usize) {
    seen[v] = true;

    let mut a_only = 1;
    let mut b_only = 1;
    if c[v] == 'a' {
        b_only = 0;
    }
    else if c[v] == 'b' {
        a_only = 0;
    }
    let mut ab_both = 1;
    let mut ab_both_exception = 1;
    for i in 0..graph[v].len() {
        let mut next_v = graph[v][i];
        if seen[next_v] {continue}
        dfs(graph, dp, next_v, seen, c, MODULO);        

        // next_vについて{切る, 切らない}で全てのケースを類別できる。
        // さらにnext_vを切る場合に、next_vが{aのみ, bのみ, ab両方}, 
        // next_vを切らない場合に、next_vが{aのみ, bのみ, ab両方}で、
        // 合わせて6パターンに類別できる(過不足無く全ての事象を網羅できる)

        // vを含む部分木だけがaのみの場合を考える。
        // next_vを切るケース(next_vがabのみ)と、切らないケース{next_vがaのみ}の2パターン 
        // (ちなみにnext_vがbのときは、切っても切らなくてもいけない。つまり、next_vがbである事象は存在してはいけない)
        a_only = (a_only * (dp[next_v][0] + dp[next_v][2])) % MODULO;

        // vを含む部分木がだけがbのみの場合を考える。
        // next_vを切るケース(next_vがabのみ)と、切らないケース{next_vがbのみ}の2パターン
        b_only = (b_only * (dp[next_v][1] + dp[next_v][2])) % MODULO;

        // vを含む部分木がab両方の場合を考える。
        // (i)vがaのとき
        // next_vを切るケース(next_vがabのみ)と、切らないケース{next_vがaのみ, next_vがbのみ, ab両方}の2パターン
        if c[v] == 'a' {
            ab_both = (ab_both * (dp[next_v][0] + dp[next_v][1] + 2 * dp[next_v][2])) % MODULO;
            // 上のうち、vを含む部分木がaのみになってしまう例外を差し引く。それは、切るケースでnext_vがabで、切らないケースで全てaのみになるとき
            ab_both_exception = (ab_both_exception * (dp[next_v][0] + dp[next_v][2])) % MODULO;
        }
        // (ii)vがbのとき、
        // next_vを切るケース(next_vがabのみ)と、切らないケース{next_vがaのみ, next_vがbのみ, ab両方}の2パターン
        else if c[v] == 'b' {
            ab_both = (ab_both * (dp[next_v][0] + dp[next_v][1] + 2 * dp[next_v][2])) % MODULO;
            // 上のうち、vを含む部分木がbのみになってしまう例外を差し引く。それは、切るケースでnext_vがabで、切らないケースで全てbのみになるとき
            ab_both_exception = (ab_both_exception * (dp[next_v][1] + dp[next_v][2])) % MODULO;
        }
    }

    dp[v][0] = a_only;
    dp[v][1] = b_only;
    dp[v][2] = (MODULO + ab_both - ab_both_exception) % MODULO;
    
}