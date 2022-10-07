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
        k: usize,
    }

    // let AB_MAX = 10;
    let AB_MAX = 5000;
    let mut table = vec![vec![0; AB_MAX + 1]; AB_MAX + 1];
    for i in 0..n {
        input! {
            a_i: usize,
            b_i: usize,
        }
        table[a_i][b_i] += 1;
    }
    // println!("INPUT OK");
    // for i in 0..(AB_MAX + 1) {
    //     println!("{:?}", table[i]);
    // }

    // 身長A=iで、体重がB=j以下の人数が格納 (体重Bについての累積和)
    let mut table_cum = vec![vec![0; AB_MAX + 1]; AB_MAX + 1];
    for i in 1..(AB_MAX+1) {
        for j in 1..(AB_MAX+1) {
            table_cum[i][j] = table_cum[i][j-1] + table[i][j];
        }
    }
    // dbg!("table_cum: ok");
    // for i in 0..(AB_MAX + 1) {
    //     println!("{:?}", table_cum[i]);
    // }

    // 身長A=iのとき、体重Bが[j:j+k]の範囲に何人いるかを確認するテーブル
    let mut table_range = vec![vec![0; AB_MAX + 1]; AB_MAX + 1];
    for i in 1..(AB_MAX+1) {
        for j in 1..(AB_MAX+1) {
            let right = min(AB_MAX, j + k);
            table_range[i][j] = table_cum[i][right] - table_cum[i][j-1];
        }
    }
    // dbg!("table_range: ok");
    // for i in 0..(AB_MAX + 1) {
    //     println!("{:?}", table_range[i]);
    // }

    // 身長A=i以下で、体重Bが[j:j+k]の範囲にある人の累積和
    let mut table_range_cum = vec![vec![0; AB_MAX + 1]; AB_MAX + 1];
    for i in 1..(AB_MAX+1) {
        for j in 1..(AB_MAX+1) {
            // let bottom = min(AB_MAX, i + k);
            table_range_cum[i][j] = table_range_cum[i-1][j] + table_range[i][j];
        }
    }
    // dbg!("table_range_cum: ok");
    // for i in 0..(AB_MAX + 1) {
    //     println!("{:?}", table_range_cum[i]);
    // }

    // 身長Aが[i:i+k]で、体重Bが[j:j+k]の範囲にある人の累積和
    let mut table_answer = vec![vec![0; AB_MAX + 1]; AB_MAX + 1];
    let mut answer = 0;
    for i in 1..(AB_MAX+1) {
        for j in 1..(AB_MAX+1) {
            let bottom = min(AB_MAX, i + k);
            table_answer[i][j] = table_range_cum[bottom][j] - table_range_cum[i-1][j];
            answer = max(answer, table_range_cum[bottom][j] - table_range_cum[i-1][j]);
        }
    }
    println!("{}", answer);
    // dbg!("table_answer: ok");
    // for i in 0..(AB_MAX + 1) {
    //     println!("{:?}", table_answer[i]);
    // }

}

