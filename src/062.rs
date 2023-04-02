use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::{VecDeque, HashMap};
use std::collections::BinaryHeap;
use proconio::marker::Chars;
fn main() {
    // 方針: 完成された状態(全ボールの色が塗られた状態)から、逆に色を消していく。
    // 色を消した順序の逆が塗られた順番となる
    input! {
        n: usize,
    }

    // 有向グラフ. i番目のノードには、AiとBiへのエッジが張られる。
    let mut graph = vec![vec![]; n];
    // graphの全エッジを逆向きに張ったグラフ
    let mut rev_graph = vec![vec![]; n];
    // 各ノードの出次数を記録
    let mut out_degrees = vec![0; n];
    // 追加したグラフのエッジを記録 (2回同じエッジを追加しないようにするために使用)
    let mut hash = HashMap::new();
    // 自分へのエッジが張られたノードを管理
    let mut self_loop = vec![false; n];
    for i in 0..n {
        input! {
            a_i: usize,
            b_i: usize,
        }
        // 特定のエッジを一意の数字で割り当てる
        let input_pattern1 = (a_i-1) * n + i;
        let input_pattern2 = (b_i-1) * n + i;
        
        // 既に存在しているエッジは二重に追加しない
        if !hash.contains_key(&input_pattern1) {
            hash.insert(input_pattern1, 0);
            if a_i - 1 == i {
                self_loop[i] = true;
                continue
            }

            graph[i].push(a_i-1);
            rev_graph[a_i-1].push(i);
            out_degrees[i] += 1;
        }
        if !hash.contains_key(&input_pattern2) {
            hash.insert(input_pattern2, 0);
            if b_i - 1 == i {
                self_loop[i] = true;
                continue}
            graph[i].push(b_i-1);
            rev_graph[b_i-1].push(i);
            out_degrees[i] += 1;
        }
    }

    // 完成された状態から、逆に色を消していく。
    // "出次数が0"または"自分へのエッジが張られたノード"は、色を消して良いので、deque に予約していく。
    // ↑よく考えたら、出自数0の条件は要らないかも。"出次数が0" <=> "自分へのエッジを2本張る"と同義なので。
    // 重複した予約を避ける為に、reserved (hashmap)で既に予約済みのものをメモしておく
    let mut deque = VecDeque::new();
    let mut reserved = HashMap::new();
    for i in 0..n {
        if out_degrees[i] ==0 || self_loop[i] {
            deque.push_back(i);
            reserved.insert(i, 0);
        }
    }
    // println!("deque: {:?}", deque);

    // 色を消したものから順に格納していく
    let mut unpainted = vec![];
    while deque.len() != 0 {
        let v = deque.pop_front().unwrap();
        unpainted.push(v);
        // println!("unpaint {}", v);
        for i in 0..rev_graph[v].len() {
            let next_v = rev_graph[v][i];
            if reserved.contains_key(&next_v) {continue}
            // println!("insert {}", next_v);
            deque.push_back(next_v);
            reserved.insert(next_v, 0);
        }
    }

    // 全部塗った状態から、全ての色を消すことが出来ているか?
    if unpainted.len() == n {
        for i in 0..n {
            // 色を消すのが遅いものから塗られている。
            println!("{}", unpainted[n-1-i] + 1);
        }
    }
    else {
        println!("-1");
    }
}