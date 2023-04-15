use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::{VecDeque, HashMap};
use std::collections::BinaryHeap;
use proconio::marker::Chars;
use superslice::*;

fn main() {
    // 平方分割の問題
    // 愚直にやってTLEしてしまうケースとそうでないケースに場合分けしてアルゴリズムを作る。
    // 場合分けの基準として、相加平均相乗平均の関係を使う(≒結果的に平方分割)
    input! {
        n: usize,
        m: usize,
    }
    let mut a = vec![];
    let mut b = vec![];
    let mut G = vec![vec![]; n];
    let mut g_hash = vec![HashMap::new(); n];
    let mut out_degree = vec![0; n];
    for i in 0..m {
        input! {
            a_i: usize,
            b_i: usize,
        }
        a.push(a_i);
        b.push(b_i);
        G[a_i-1].push(b_i-1);
        G[b_i-1].push(a_i-1);
        g_hash[a_i-1].insert(b_i-1, -1);
        g_hash[b_i-1].insert(a_i-1, -1);
        out_degree[a_i-1] += 1;
        out_degree[b_i-1] += 1;
    }
    input!{
        q: usize
    }
    let mut X = vec![];
    let mut Y = vec![];

    for i in 0..q {
        input! {
            x_i: usize,
            y_i: usize,
        }
        X.push(x_i-1);
        Y.push(y_i);
    }

    // 沢山エッジを持つ頂点(B本以上)は特別に扱うB := root(2*m)
    // 全体のアルゴリズムがO(Q(B+2M/B))だが、相加平均相乗平均の関係より、
    // root(B * 2M/B) <= (B+2M/B) / 2
    // であり、B ==2M/B <=> B = root(2m)のとき、
    // B+2M/B = 2*root(B * 2M/B) で最小だから、
    // O(2Q*root(2M)) = O(4√2 * 10^7.5)となり3秒以下で実行可能
    let many_edge_thresh = 2 * m;

    // ある頂点のエッジが、√many_edge_thresh (=:E8氏の解説のB)を超えたら特別扱いする
    // has_many_edge := key:エッジを沢山持つ頂点の番号, value: keyの隣接頂点が登場した最後のクエリ番号(-1で初期化) 
    let mut has_many_edge = HashMap::new();
    for i in 0..n {
        if out_degree[i] * out_degree[i] >= many_edge_thresh {
            has_many_edge.insert(i, -1);
            // 少 -> 多 へのアクセスが欲しい
            // 少が多iと繋がっているか? [iのループ]
            // 繋がっていたら、その位置を多i用のリストにキープしておく
        }
    }

    // アルゴリズム1: O(Q(B+2M/B))　>= O(2Q*root(2M))
    // 現在のクエリiで指定された頂点 xi に隣接する頂点 next_xi_j の中で、
    // 最後に塗られた頂点のクエリkの色がprintするべき色
    // ただし、これだけだとO(QM)でTLEするので、
    // エッジがB本以上の頂点が、最大2*M/B本しかないことに着目して特別扱いする
    // 特別扱い: エッジがB本以上の頂点と隣接する頂点が最後に出てきたクエリ番号をメモする(1クエリあたりO(2M/B))
    // current_last_x[i] := クエリの最後に出てきたxのクエリ番号を格納しておく。
    let mut current_last_x = vec![-1; n];
    for i in 0..q {
        let mut nearest_query: isize = -1;
        // X[i]のエッジの本数がB以上の場合: O(1)
        if has_many_edge.contains_key(&X[i]) {
            nearest_query = max(nearest_query, has_many_edge[&X[i]]);
        }
        // X[i]のエッジの本数がB未満の場合: O(B)
        else {
            for &next_x in G[X[i]].iter() {
                nearest_query = max(nearest_query, current_last_x[next_x]); 
            }
        }
        // 自分自身(X[i])が最後に塗られたタイミングもチェック
        nearest_query = max(nearest_query, current_last_x[X[i]]); 
        current_last_x[X[i]] = i as isize;

        // 沢山エッジを持っている各頂点と現在のクエリの頂点が隣接しているか?: O(2M/B)
        for (k, v) in has_many_edge.iter_mut() {
            // 隣接していたら、最後に出てきたクエリ番号をメモ
            if g_hash[*k].contains_key(&X[i]) {
                *v = i as isize;
            }
        }
        if nearest_query == -1 {
            println!("1");
        }
        else {
            println!("{}", Y[nearest_query as usize]);
        }
    }

    // アルゴリズム2: 愚直にシミュレーション(クエリ通りに色を塗っていく) (TLE)
    // if has_many_edge.len() == 0 {
    // // color[v] := 頂点vの色
    // let mut color = vec![1; n];
    // for i in 0..q {
    //     println!("{}", color[X[i]]);
    //     // 色塗り
    //     color[X[i]] = Y[i];
    //     for next_v in G[X[i]].iter() {
    //         color[*next_v] = Y[i];
    //     }
    // }

}
