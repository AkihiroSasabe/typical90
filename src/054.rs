use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use proconio::marker::Chars;
use std::collections::HashMap;

fn main() {
    // 2023-03-26 15:00-17:47 (約3h)で自力AC
    input! {
        n: usize,
        m: usize
    }
    // ◆条件
    // N <= 10^5
    // M <= 10^5
    // Ki <= N
    // K1 + K2 + ... KM <= 2x10^5
    // ◆高橋数Tの定義
    // (A): 高橋数が n の研究者との共著経験があり、
    // (B): 高橋数が n 未満の研究者との共著経験がない研究者の高橋数は  n+1 とする。

    // ◆考察
    // ・高橋(T=0)と繋がっている人は全員T=1になる。(B)の要請より、T >= 2だと、1未満(=高橋)と共著がないことになるので。
    // ・T=1の人と繋がっている人は全員T=2になる。(B)の要請より、T >= 3だと、2未満のやつ(=T1)と共著がないことになるので。
    // ...
    // 以上を繰り返すと結局、高橋からの最短距離が高橋数となる。

    // ただし、普通のBFSで最短距離を求めるとTLEする
    // そこでグラフの辺の削減を考える
    // ある論文に共著者がn人居た場合、辺の数はn(n-1)/2となるが、
    // 高橋からもっとも近い代表著者から各著者への辺だけ考えればいいので、それ以外を無視すると結局n-1本に減らせる。
    // これで制限時間内のBFSで最短経路問題を解くことが可能である

    // ◆実装
    // authors_list[i] := 論文iの著者達が格納されてる
    let mut authors_list: Vec<Vec<usize>> = vec![vec![]; m];

    // papers_list[i] := 人iが書いた論文番号が格納される
    let mut papers_list: Vec<Vec<usize>> = vec![vec![]; n];

    for i in 0..m {
        input! {
            k_i: usize,
            r_i: [usize; k_i]
        }
        for j in 0..k_i {
            let author = r_i[j] - 1;
            authors_list[i].push(author);
            papers_list[author].push(i);
        }
    }

    // 高橋からの最短経路を問題を解く
    let mut dist = vec![0; n];
    let mut queue = VecDeque::new();
    let mut ronbun_seen = vec![false; m];
    // 高橋からスタート
    queue.push_back(0);

    while queue.len() != 0 {
        let v = queue.pop_front().unwrap();
        // println!("v: {}", v);
        for i in 0..papers_list[v].len() {
            let ronbun = papers_list[v][i];
            if ronbun_seen[ronbun] {continue}
            ronbun_seen[ronbun] = true;
            for &next_v in authors_list[ronbun].iter() {
                if dist[next_v] != 0 {continue}
                if next_v == v {continue}
                dist[next_v] = dist[v] + 1;
                queue.push_back(next_v);
            }
        }
    }

    println!("{}", 0);
    for i in 1..n {
        if dist[i] != 0 {
            println!("{}", dist[i]);
        }
        else {
            println!("{}", -1);
        }
    }
}