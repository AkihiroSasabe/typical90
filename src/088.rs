use proconio::input;
// use itertools::Itertools;
// use std::cmp::{max, min};
// use std::cmp::Ordering;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
    }
    let mut prohibited_pair = vec![HashMap::new(); n];
    for i in 0..q {
        input! {
            x_i: usize,
            y_i: usize,
        }
        prohibited_pair[x_i - 1].insert(y_i-1, 0);
        prohibited_pair[y_i - 1].insert(x_i-1, 0);
    }





    // dbg!("====");

    // dp[所有カードの合計値s] = Vec<合計がsになるカードの組合せ>
    let mut dp: Vec<Vec<HashMap<usize, usize>>> = vec![vec![]; 20_000];
    // 初期化
    for i in 0..n {
        let mut temp_hash = HashMap::new();
        temp_hash.insert(i, 0_usize);
        dp[a[i]].push(temp_hash);
        
        // もし値が等しいカードがあれば、この時点で回答可能
        if dp[a[i]].len() > 1 {
            for dict in dp[a[i]].iter() {
                println!("1");
                let (k, v) = dict.iter().next().unwrap();
                println!("{}", k + 1);
            }
            return;
        }
    }
    // println!("dp={:?}", dp);


    // dbg!("----");
    // dp[sum]の中身が2つになった瞬間に回答する
    // 全体計算量O(N * (A1+A2+...An) * N) = O(88 x 8888 x 88) <= O(10^6)
    for i in 0..n {
        for j in 0..8889 {
            // println!("i={}, j={}, dp[{}]={:?}, dp[{}+{}]={:?}", i, j, j, dp[j], j, a[i], dp[j+a[i]]);

            // カードiを挿入可能か?
            let mut flag = true;
            // 挿入前の状態が存在しないときは挿入不可
            if dp[j].len() == 0 {
                continue;
            }
            if dp[j].len() != 0 {
                // 挿入前の状態に、組合せが禁止されているカードがあったら挿入不可
                // 計算量O(N) = O(88)
                for (pk, pv) in prohibited_pair[i].iter() {
                    if dp[j][0].contains_key(pk) {
                        flag = false;
                        break
                    }                
                }
                // 挿入前の状態に、既に自分自身のカードがあったら挿入不可
                if dp[j][0].contains_key(&i) {
                    flag = false;
                }
            }
            // 挿入先の状態に、カードiが既に存在している
            if dp[j+a[i]].len() != 0 {
                if dp[j+a[i]][0].contains_key(&i) {
                    flag = false;
                }
            }
            // カードiを挿入可能な時
            if flag {
                // 挿入先に他の状態が存在しないとき
                if dp[j+a[i]].len() == 0 {
                    let mut temp_hash: HashMap<usize, usize> = dp[j][0].clone();
                    temp_hash.insert(i, 0);
                    dp[j+a[i]].push(temp_hash);
                }
                // 挿入先に他の状態が存在するとき
                else {
                    // 既に存在する状態
                    println!("{}", dp[j+a[i]][0].len());
                    let mut ans1 = vec![];
                    for (k, v) in dp[j+a[i]][0].iter() {
                        // print!("{} ", k + 1);
                        ans1.push(k + 1);
                    }
                    ans1.sort();
                    for i in 0..ans1.len() {
                        print!("{} ", ans1[i]);
                    }
                    println!("");
                    // println!("i={} ", i);

                    // これから存在する状態
                    let mut temp_hash = dp[j][0].clone();
                    temp_hash.insert(i, 0);
                    println!("{}", temp_hash.len());
                    let mut ans2 = vec![];
                    for (k, v) in temp_hash.iter() {
                        ans2.push(k + 1);
                        // print!("{} ", k + 1);
                    }
                    ans2.sort();
                    for i in 0..ans2.len() {
                        print!("{} ", ans2[i]);
                    }
                    println!("");
                    return;
                }
            }
        }
    }
}


// E8氏はdfsで解いていて、かつっぱ氏もDFSで解いている https://youtu.be/5AFy_m6XPqc?t=2611
// 中身のコードを読むのが手間なのでスキップ
// fn dfs(card_index: usize, n: usize, a: &Vec<usize>) {
//     if card_index == n - 1 {
//         let sm = 
//     }
//     dfs(card_index + 1);
// }