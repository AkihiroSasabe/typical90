use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use proconio::marker::Chars;
fn main() {
    // 解説通りの実装
    // 頂点uから頂点vへの到達可能判定は、トポロジカルソート順DPで判定する
    // usizeが64ビットあるので、ビット演算で64クエリ毎にまとめて処理して計算量を1/64にする
    // 例：
    // dp[v][query=3] = 1 <- クエリ3のとき頂点vに到達可能(愚直なやり方)
    // dp[v] = 00100      <- クエリ3のとき頂点vに到達可能(ビット演算で計算量を1/64にしたやり方。1bitが1個のクエリに対応。)
    input! {
        n: usize,
        m: usize,
        q: usize,
    }
    let mut graph = vec![vec![]; n];
    for i in 0..m {
        input! {
            x_i: usize,
            y_i: usize,
        }
        graph[x_i-1].push(y_i-1);
    }
    let mut a = vec![];
    let mut b = vec![];
    for i in 0..q {
        input! {
            a_i: usize,
            b_i: usize,
        }
        a.push(a_i-1);
        b.push(b_i-1);
    }
    
    // dp[v]のiビット目の数 := クエリiの始点から頂点vに到達可能なら1, 不可能なら0が格納されている(usizeは64bitなので、64クエリまで計算可能)
    let mut dp: Vec<usize> = vec![0; n];

    // 全体の計算量はO(Q/64 * M) クエリの計算量を1/64にできる
    for i in 0..q {
        // クエリの始点の頂点を1にしておく
        dp[a[i]] |= 1 << (i % 64);

        // 64回毎にまとめて計算.(最後のクエリ番号は64の倍数じゃなくても実行する必要がある)
        if i % 64 == 63 || i == q-1 {
            // debug
            // println!("initial dp");
            // for ii in 0..n {
            //     println!("v={}, dp={:064b}", ii, dp[ii]);
            // }

            // 64個分の各クエリの始点から、全N個の頂点まで到達可能か計算 
            // トポロジカルソート順にDP (O(M)の計算量)
            // 本問は既に0,1,2,...nの順が、トポロジカルソートされた順になっている。
            for v in 0..n {
                for v_next in graph[v].iter() {
                    // v -> v_next のとき、vが到達可能ならv_nextも到達可能
                    dp[*v_next] |= dp[v]; 
                }
            }
            // debug
            // println!("after dp");
            // for ii in 0..n {
            //     println!("v={}, dp={:064b}", ii, dp[ii]);
            // }

            // 各クエリについて回答
            // O(64)
            for query_index in (i-i%64)..i+1 {    
                // let query_index = i - 63 + j;
                let query_v = b[query_index];
                if (dp[query_v] & (1 << (query_index % 64))) != 0 {
                    println!("Yes");
                    // debug
                    // println!("query={:02}, Yes {:2} -> {:2}", query_index, a[query_index] + 1, query_v + 1);
                }
                else {
                    println!("No");
                    // debug
                    // println!("query={:02}, No  {:2} -> {:2}", query_index, a[query_index] + 1, query_v + 1);
                }
            }
            // 64回に1度、dpを初期化する
            dp = vec![0; n];
        }
    }
}

// デバッグ用テストケース (解説図のグラフと同じ)
// 9 10 81
// 1 2
// 2 7
// 7 8
// 8 9
// 1 3
// 3 5
// 3 4
// 4 6
// 5 6
// 6 9
// 1 1
// 2 1
// 3 1
// 4 1
// 5 1
// 6 1
// 7 1
// 8 1
// 9 1
// 1 2
// 2 2
// 3 2
// 4 2
// 5 2
// 6 2
// 7 2
// 8 2
// 9 2
// 1 3
// 2 3
// 3 3
// 4 3
// 5 3
// 6 3
// 7 3
// 8 3
// 9 3
// 1 4
// 2 4
// 3 4
// 4 4
// 5 4
// 6 4
// 7 4
// 8 4
// 9 4
// 1 5
// 2 5
// 3 5
// 4 5
// 5 5
// 6 5
// 7 5
// 8 5
// 9 5
// 1 6
// 2 6
// 3 6
// 4 6
// 5 6
// 6 6
// 7 6
// 8 6
// 9 6
// 1 7
// 2 7
// 3 7
// 4 7
// 5 7
// 6 7
// 7 7
// 8 7
// 9 7
// 1 8
// 2 8
// 3 8
// 4 8
// 5 8
// 6 8
// 7 8
// 8 8
// 9 8
// 1 9
// 2 9
// 3 9
// 4 9
// 5 9
// 6 9
// 7 9
// 8 9
// 9 9